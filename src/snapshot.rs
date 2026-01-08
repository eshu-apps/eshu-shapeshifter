use crate::config::{EshuConfig, Snapshot, SnapshotType};
use crate::distro;
use crate::error::{EshuError, EshuResult};
use colored::Colorize;
use dialoguer::{Confirm, Select};
use indicatif::{ProgressBar, ProgressStyle};
use std::fs;
use std::path::PathBuf;
use std::process::Command;

/// Create a system snapshot before migration
pub async fn create_snapshot(description: String) -> EshuResult<Snapshot> {
    println!("{}", "\n📸 Creating system snapshot...".cyan().bold());

    let config = EshuConfig::load().map_err(|e| EshuError::Config(e.to_string()))?;
    let (distro, version, _) = distro::detect_current_distro()?;

    // Detect best snapshot method
    let snapshot_type = detect_snapshot_method()?;

    // Check available disk space for rsync snapshots
    if matches!(snapshot_type, SnapshotType::Rsync) {
        check_disk_space(&config.snapshot_dir)?;
    }
    
    let snapshot_id = format!("snapshot_{}", chrono::Utc::now().timestamp());
    let snapshot_path = config.snapshot_dir.join(&snapshot_id);
    
    std::fs::create_dir_all(&snapshot_path)
        .map_err(|e| EshuError::Snapshot(format!("Failed to create snapshot directory: {}", e)))?;
    
    let snapshot = Snapshot {
        id: snapshot_id.clone(),
        timestamp: chrono::Utc::now().timestamp(),
        distro_name: distro,
        distro_version: version,
        description,
        snapshot_type: snapshot_type.clone(),
        size_bytes: 0,
        path: snapshot_path.clone(),
    };
    
    match snapshot_type {
        SnapshotType::Btrfs => create_btrfs_snapshot(&snapshot)?,
        SnapshotType::LVM => create_lvm_snapshot(&snapshot)?,
        SnapshotType::Rsync => create_rsync_snapshot(&snapshot)?,
    }
    
    // Save snapshot metadata
    save_snapshot_metadata(&snapshot)?;
    
    println!("{}", format!("✅ Snapshot created: {}", snapshot_id).green().bold());
    
    Ok(snapshot)
}

/// Detect the best snapshot method for the system
fn detect_snapshot_method() -> EshuResult<SnapshotType> {
    let fs_type = distro::detect_filesystem();

    // Check for btrfs
    if fs_type == "btrfs" {
        return Ok(SnapshotType::Btrfs);
    }

    // Check for LVM
    let lvm_check = Command::new("lvdisplay")
        .output()
        .map(|o| o.status.success())
        .unwrap_or(false);

    if lvm_check {
        return Ok(SnapshotType::LVM);
    }

    // Fallback to rsync
    Ok(SnapshotType::Rsync)
}

/// Check if there's sufficient disk space for snapshot
fn check_disk_space(snapshot_dir: &PathBuf) -> EshuResult<()> {
    // Get disk usage statistics using df
    let output = Command::new("df")
        .args(&["-B1", "/"])
        .output()
        .map_err(|e| EshuError::Snapshot(format!("Failed to check disk space: {}", e)))?;

    if !output.status.success() {
        return Err(EshuError::Snapshot("Failed to check disk space".to_string()));
    }

    let stdout = String::from_utf8_lossy(&output.stdout);
    let lines: Vec<&str> = stdout.lines().collect();

    if lines.len() < 2 {
        return Err(EshuError::Snapshot("Could not parse disk space information".to_string()));
    }

    // Parse the second line (first data line)
    let parts: Vec<&str> = lines[1].split_whitespace().collect();
    if parts.len() < 4 {
        return Err(EshuError::Snapshot("Could not parse disk space information".to_string()));
    }

    let available_bytes: u64 = parts[3].parse()
        .map_err(|_| EshuError::Snapshot("Could not parse available space".to_string()))?;

    // Require at least 20GB free for rsync snapshots
    const MIN_SPACE_BYTES: u64 = 20 * 1024 * 1024 * 1024; // 20GB

    if available_bytes < MIN_SPACE_BYTES {
        let available_gb = available_bytes as f64 / (1024.0 * 1024.0 * 1024.0);
        return Err(EshuError::Snapshot(
            format!(
                "Insufficient disk space. Need at least 20GB, but only {:.2}GB available",
                available_gb
            )
        ));
    }

    println!("  ✓ Sufficient disk space available ({:.2}GB free)",
        available_bytes as f64 / (1024.0 * 1024.0 * 1024.0));

    Ok(())
}

fn create_btrfs_snapshot(snapshot: &Snapshot) -> EshuResult<()> {
    println!("  Using btrfs snapshot (instant, copy-on-write)");
    
    let output = Command::new("btrfs")
        .args(&[
            "subvolume",
            "snapshot",
            "/",
            snapshot.path.to_str().unwrap(),
        ])
        .output()
        .map_err(|e| EshuError::Snapshot(format!("Btrfs snapshot failed: {}", e)))?;
    
    if !output.status.success() {
        return Err(EshuError::Snapshot(
            format!("Btrfs snapshot failed: {}", String::from_utf8_lossy(&output.stderr))
        ));
    }
    
    Ok(())
}

fn create_lvm_snapshot(snapshot: &Snapshot) -> EshuResult<()> {
    println!("  Using LVM snapshot");
    
    // Get root logical volume
    let output = Command::new("findmnt")
        .args(&["-n", "-o", "SOURCE", "/"])
        .output()
        .map_err(|e| EshuError::Snapshot(format!("Failed to find root device: {}", e)))?;
    
    let root_lv = String::from_utf8_lossy(&output.stdout).trim().to_string();
    
    // Create LVM snapshot (10GB size)
    let snapshot_name = format!("{}_lv", snapshot.id);
    let output = Command::new("lvcreate")
        .args(&[
            "-L", "10G",
            "-s",
            "-n", &snapshot_name,
            &root_lv,
        ])
        .output()
        .map_err(|e| EshuError::Snapshot(format!("LVM snapshot failed: {}", e)))?;
    
    if !output.status.success() {
        return Err(EshuError::Snapshot(
            format!("LVM snapshot failed: {}", String::from_utf8_lossy(&output.stderr))
        ));
    }
    
    Ok(())
}

fn create_rsync_snapshot(snapshot: &Snapshot) -> EshuResult<()> {
    println!("  Using rsync backup (this may take a while)");
    
    let pb = ProgressBar::new_spinner();
    pb.set_style(
        ProgressStyle::default_spinner()
            .template("{spinner:.cyan} {msg}")
            .unwrap()
    );
    pb.set_message("Backing up system files...");
    
    // Critical directories to backup
    let backup_dirs = vec![
        "/etc",
        "/var/lib",
        "/usr/local",
        "/opt",
        "/home",
        "/root",
    ];
    
    for dir in backup_dirs {
        if !PathBuf::from(dir).exists() {
            continue;
        }
        
        pb.set_message(format!("Backing up {}...", dir));
        
        let target = snapshot.path.join(dir.trim_start_matches('/'));
        std::fs::create_dir_all(&target)
            .map_err(|e| EshuError::Snapshot(format!("Failed to create target dir: {}", e)))?;
        
        let output = Command::new("rsync")
            .args(&[
                "-aAXv",
                "--exclude=/dev",
                "--exclude=/proc",
                "--exclude=/sys",
                "--exclude=/tmp",
                "--exclude=/run",
                "--exclude=/mnt",
                "--exclude=/media",
                "--exclude=/lost+found",
                "--exclude=/var/cache/apt",
                "--exclude=/var/cache/pacman",
                "--exclude=/var/cache/yum",
                "--exclude=/var/cache/dnf",
                "--exclude=/var/tmp",
                "--exclude=.cache",
                "--exclude=*.log",
                "--exclude=/home/*/.cache",
                "--exclude=/root/.cache",
                dir,
                target.to_str().unwrap(),
            ])
            .output()
            .map_err(|e| EshuError::Snapshot(format!("Rsync failed: {}", e)))?;
        
        if !output.status.success() {
            pb.finish_with_message("Failed");
            return Err(EshuError::Snapshot(
                format!("Rsync failed for {}: {}", dir, String::from_utf8_lossy(&output.stderr))
            ));
        }
    }
    
    pb.finish_with_message("Backup complete");
    
    Ok(())
}

fn save_snapshot_metadata(snapshot: &Snapshot) -> EshuResult<()> {
    let config = EshuConfig::load().map_err(|e| EshuError::Config(e.to_string()))?;
    let metadata_path = config.snapshot_dir.join(format!("{}.json", snapshot.id));
    
    let json = serde_json::to_string_pretty(snapshot)
        .map_err(|e| EshuError::Serialization(e.to_string()))?;
    
    std::fs::write(metadata_path, json)
        .map_err(|e| EshuError::Snapshot(format!("Failed to save metadata: {}", e)))?;
    
    Ok(())
}

/// List all available snapshots
pub async fn list_snapshots() -> anyhow::Result<()> {
    let config = EshuConfig::load()?;
    
    if !config.snapshot_dir.exists() {
        println!("{}", "No snapshots found.".yellow());
        return Ok(());
    }
    
    let mut snapshots = Vec::new();
    
    for entry in fs::read_dir(&config.snapshot_dir)? {
        let entry = entry?;
        let path = entry.path();
        
        if path.extension().and_then(|s| s.to_str()) == Some("json") {
            if let Ok(content) = fs::read_to_string(&path) {
                if let Ok(snapshot) = serde_json::from_str::<Snapshot>(&content) {
                    snapshots.push(snapshot);
                }
            }
        }
    }
    
    if snapshots.is_empty() {
        println!("{}", "No snapshots found.".yellow());
        return Ok(());
    }
    
    snapshots.sort_by(|a, b| b.timestamp.cmp(&a.timestamp));
    
    println!("\n{}", "📸 Available Snapshots:".cyan().bold());
    println!("{}", "═══════════════════════════════════════════════════════════".cyan());
    
    for snapshot in snapshots {
        let date = chrono::DateTime::<chrono::Utc>::from_timestamp(snapshot.timestamp, 0)
            .map(|dt| dt.format("%Y-%m-%d %H:%M:%S").to_string())
            .unwrap_or_else(|| "Unknown".to_string());
        
        println!("\n{}: {}", "ID".yellow(), snapshot.id.green());
        println!("  {}: {}", "Date".yellow(), date);
        println!("  {}: {} {}", "Distro".yellow(), snapshot.distro_name, snapshot.distro_version);
        println!("  {}: {}", "Description".yellow(), snapshot.description);
        println!("  {}: {:?}", "Type".yellow(), snapshot.snapshot_type);
    }
    
    Ok(())
}

/// Revert to a previous snapshot
pub async fn revert_snapshot(snapshot_id: Option<String>) -> anyhow::Result<()> {
    let config = EshuConfig::load()?;
    
    // Get list of snapshots
    let mut snapshots = Vec::new();
    
    for entry in fs::read_dir(&config.snapshot_dir)? {
        let entry = entry?;
        let path = entry.path();
        
        if path.extension().and_then(|s| s.to_str()) == Some("json") {
            if let Ok(content) = fs::read_to_string(&path) {
                if let Ok(snapshot) = serde_json::from_str::<Snapshot>(&content) {
                    snapshots.push(snapshot);
                }
            }
        }
    }
    
    if snapshots.is_empty() {
        println!("{}", "No snapshots available to revert to.".red());
        return Ok(());
    }
    
    snapshots.sort_by(|a, b| b.timestamp.cmp(&a.timestamp));
    
    // Select snapshot
    let snapshot = if let Some(id) = snapshot_id {
        snapshots.into_iter().find(|s| s.id == id)
            .ok_or_else(|| anyhow::anyhow!("Snapshot not found: {}", id))?
    } else {
        // Interactive selection
        let items: Vec<String> = snapshots.iter().map(|s| {
            let date = chrono::DateTime::<chrono::Utc>::from_timestamp(s.timestamp, 0)
                .map(|dt| dt.format("%Y-%m-%d %H:%M:%S").to_string())
                .unwrap_or_else(|| "Unknown".to_string());
            format!("{} - {} {} ({})", s.id, s.distro_name, s.distro_version, date)
        }).collect();
        
        let selection = Select::new()
            .with_prompt("Select snapshot to revert to")
            .items(&items)
            .interact()?;
        
        snapshots.into_iter().nth(selection).unwrap()
    };
    
    // Confirm
    println!("\n{}", "⚠️  WARNING: This will revert your system!".red().bold());
    println!("Target snapshot: {} - {} {}", 
        snapshot.id.yellow(), 
        snapshot.distro_name, 
        snapshot.distro_version
    );
    
    let confirmed = Confirm::new()
        .with_prompt("Are you sure you want to continue?")
        .default(false)
        .interact()?;
    
    if !confirmed {
        println!("{}", "Revert cancelled.".yellow());
        return Ok(());
    }
    
    // Perform revert
    println!("{}", "\n🔄 Reverting system...".cyan().bold());
    
    match snapshot.snapshot_type {
        SnapshotType::Btrfs => revert_btrfs_snapshot(&snapshot)?,
        SnapshotType::LVM => revert_lvm_snapshot(&snapshot)?,
        SnapshotType::Rsync => revert_rsync_snapshot(&snapshot)?,
    }
    
    println!("{}", "\n✅ System reverted successfully!".green().bold());
    println!("{}", "⚠️  Please reboot your system for changes to take effect.".yellow());
    
    Ok(())
}

fn revert_btrfs_snapshot(snapshot: &Snapshot) -> EshuResult<()> {
    println!("  Reverting btrfs snapshot...");

    // IMPORTANT: Btrfs snapshot revert requires special handling
    // The proper approach is to:
    // 1. Boot from a live USB/rescue environment
    // 2. Mount the btrfs filesystem
    // 3. Use btrfs subvolume set-default to switch to the snapshot
    // 4. Reboot
    //
    // This cannot be done safely from within the running system.
    // For now, we'll create a script that can be run from a rescue environment.

    let script_path = PathBuf::from("/root/eshu-revert-btrfs.sh");
    let script_content = format!(
        "#!/bin/bash\n\
         # Eshu Shapeshifter Btrfs Revert Script\n\
         # Run this from a live USB/rescue environment\n\
         \n\
         echo 'Mounting btrfs root...'\n\
         mkdir -p /mnt/btrfs\n\
         mount -o subvolid=5 /dev/YOUR_ROOT_DEVICE /mnt/btrfs\n\
         \n\
         echo 'Setting snapshot as default subvolume...'\n\
         btrfs subvolume set-default {}/@ /mnt/btrfs\n\
         \n\
         echo 'Done! Unmount and reboot.'\n\
         umount /mnt/btrfs\n",
        snapshot.path.display()
    );

    std::fs::write(&script_path, script_content)
        .map_err(|e| EshuError::Snapshot(format!("Failed to create revert script: {}", e)))?;

    // Make script executable
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let mut perms = std::fs::metadata(&script_path)?.permissions();
        perms.set_mode(0o755);
        std::fs::set_permissions(&script_path, perms)?;
    }

    println!("  ⚠️  IMPORTANT: Btrfs snapshot revert requires manual steps!");
    println!("  1. Reboot into a live USB/rescue environment");
    println!("  2. Run the script at: {}", script_path.display());
    println!("  3. Edit the script to set YOUR_ROOT_DEVICE (e.g., /dev/sda1)");
    println!("  4. Execute the script as root");
    println!("  5. Reboot into the restored system");

    Err(EshuError::Snapshot(
        "Btrfs revert requires manual steps from rescue environment. See script at /root/eshu-revert-btrfs.sh".to_string()
    ))
}

fn revert_lvm_snapshot(snapshot: &Snapshot) -> EshuResult<()> {
    println!("  Reverting LVM snapshot...");
    
    let snapshot_name = format!("{}_lv", snapshot.id);
    
    let output = Command::new("lvconvert")
        .args(&["--merge", &snapshot_name])
        .output()
        .map_err(|e| EshuError::Snapshot(format!("LVM merge failed: {}", e)))?;
    
    if !output.status.success() {
        return Err(EshuError::Snapshot(
            format!("LVM merge failed: {}", String::from_utf8_lossy(&output.stderr))
        ));
    }
    
    Ok(())
}

fn revert_rsync_snapshot(snapshot: &Snapshot) -> EshuResult<()> {
    println!("  Reverting rsync backup...");
    
    let pb = ProgressBar::new_spinner();
    pb.set_style(
        ProgressStyle::default_spinner()
            .template("{spinner:.cyan} {msg}")
            .unwrap()
    );
    
    // Restore backed up directories
    for entry in fs::read_dir(&snapshot.path)
        .map_err(|e| EshuError::Snapshot(format!("Failed to read snapshot: {}", e)))? 
    {
        let entry = entry.map_err(|e| EshuError::Snapshot(e.to_string()))?;
        let dir_name = entry.file_name();
        let target = PathBuf::from("/").join(&dir_name);
        
        pb.set_message(format!("Restoring /{}...", dir_name.to_string_lossy()));
        
        let output = Command::new("rsync")
            .args(&[
                "-aAXv",
                "--delete",
                entry.path().to_str().unwrap(),
                target.to_str().unwrap(),
            ])
            .output()
            .map_err(|e| EshuError::Snapshot(format!("Rsync restore failed: {}", e)))?;
        
        if !output.status.success() {
            pb.finish_with_message("Failed");
            return Err(EshuError::Snapshot(
                format!("Rsync restore failed: {}", String::from_utf8_lossy(&output.stderr))
            ));
        }
    }
    
    pb.finish_with_message("Restore complete");
    
    Ok(())
}
