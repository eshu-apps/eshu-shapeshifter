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
    
    // Generate unique snapshot ID with random suffix to avoid collisions
    let timestamp = chrono::Utc::now().timestamp();
    let random_suffix = format!("{:04x}", rand::random::<u16>());
    let snapshot_id = format!("snapshot_{}_{}", timestamp, random_suffix);
    let snapshot_path = config.snapshot_dir.join(&snapshot_id);
    
    // Check if snapshot already exists (shouldn't happen with random suffix, but be safe)
    if snapshot_path.exists() {
        println!("  {}", "⚠️  Snapshot path already exists, generating new ID...".yellow());
        let random_suffix2 = format!("{:04x}", rand::random::<u16>());
        let snapshot_id = format!("snapshot_{}_{}", timestamp, random_suffix2);
        let snapshot_path = config.snapshot_dir.join(&snapshot_id);
        
        if snapshot_path.exists() {
            return Err(EshuError::Snapshot(
                "Failed to generate unique snapshot ID after retries".to_string()
            ));
        }
    }
    
    // For btrfs, don't create the directory - btrfs will do it
    if !matches!(snapshot_type, SnapshotType::Btrfs) {
        std::fs::create_dir_all(&snapshot_path)
            .map_err(|e| EshuError::Snapshot(format!("Failed to create snapshot directory: {}", e)))?;
    }
    
    let snapshot = Snapshot {
        id: snapshot_id.clone(),
        timestamp,
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
    
    // First, try to find the root subvolume
    let findmnt_output = Command::new("findmnt")
        .args(&["-n", "-o", "SOURCE", "/"])
        .output()
        .map_err(|e| EshuError::Snapshot(format!("Failed to find root mount: {}", e)))?;
    
    let root_source = String::from_utf8_lossy(&findmnt_output.stdout).trim().to_string();
    
    // Create the snapshot
    let output = Command::new("btrfs")
        .args(&[
            "subvolume",
            "snapshot",
            "-r", // Read-only snapshot for safety
            "/",
            snapshot.path.to_str().unwrap(),
        ])
        .output()
        .map_err(|e| EshuError::Snapshot(format!("Btrfs snapshot failed: {}", e)))?;
    
    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        
        // If it's a "not a btrfs subvolume" error, try without the -r flag
        if stderr.contains("not a btrfs") || stderr.contains("Invalid argument") {
            println!("  Retrying without read-only flag...");
            
            let output2 = Command::new("btrfs")
                .args(&[
                    "subvolume",
                    "snapshot",
                    "/",
                    snapshot.path.to_str().unwrap(),
                ])
                .output()
                .map_err(|e| EshuError::Snapshot(format!("Btrfs snapshot failed: {}", e)))?;
            
            if !output2.status.success() {
                return Err(EshuError::Snapshot(
                    format!("Btrfs snapshot failed: {}", String::from_utf8_lossy(&output2.stderr))
                ));
            }
        } else {
            return Err(EshuError::Snapshot(
                format!("Btrfs snapshot failed: {}", stderr)
            ));
        }
    }
    
    println!("  {}", "✓ Btrfs snapshot created successfully".green());
    
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
    
    println!("  {}", "✓ LVM snapshot created successfully".green());
    
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
        
        // Check if snapshot path exists
        if snapshot.path.exists() {
            println!("  {}: {}", "Status".yellow(), "✓ Available".green());
        } else {
            println!("  {}: {}", "Status".yellow(), "⚠️  Path missing".red());
        }
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
    
    // Verify snapshot exists
    if !snapshot.path.exists() {
        println!("{}", "❌ Snapshot path does not exist!".red().bold());
        println!("Expected at: {}", snapshot.path.display());
        println!("The snapshot may have been deleted or moved.");
        return Ok(());
    }
    
    // Confirm
    println!("\n{}", "⚠️  WARNING: This will revert your system!".red().bold());
    println!("Target snapshot: {} - {} {}", 
        snapshot.id.yellow(), 
        snapshot.distro_name, 
        snapshot.distro_version
    );
    println!("Snapshot type: {:?}", snapshot.snapshot_type);
    
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
    
    let revert_result = match snapshot.snapshot_type {
        SnapshotType::Btrfs => revert_btrfs_snapshot(&snapshot),
        SnapshotType::LVM => revert_lvm_snapshot(&snapshot),
        SnapshotType::Rsync => revert_rsync_snapshot(&snapshot),
    };
    
    match revert_result {
        Ok(_) => {
            println!("{}", "\n✅ System reverted successfully!".green().bold());
            println!("{}", "⚠️  Please reboot your system for changes to take effect.".yellow());
        }
        Err(e) => {
            eprintln!("{}", "\n❌ Revert failed!".red().bold());
            eprintln!("Error: {}", e);
            eprintln!("\n{}", "Manual recovery may be required.".yellow());
            return Err(e.into());
        }
    }
    
    Ok(())
}

fn revert_btrfs_snapshot(snapshot: &Snapshot) -> EshuResult<()> {
    println!("  Reverting btrfs snapshot...");

    // For btrfs, we need to handle this carefully
    // The safest approach is to provide instructions rather than attempting automatic revert
    
    println!("\n{}", "⚠️  IMPORTANT: Btrfs snapshot revert requires special handling!".yellow().bold());
    println!("\n{}", "Automatic btrfs revert from a running system is not safe.".yellow());
    println!("{}", "Here are your options:".green().bold());
    println!("\n{}", "Option 1: Boot from snapshot (Recommended)".cyan().bold());
    println!("  1. Reboot your system");
    println!("  2. At the bootloader, select the snapshot subvolume");
    println!("  3. If using GRUB, you may need to regenerate config:");
    println!("     sudo grub-mkconfig -o /boot/grub/grub.cfg");
    
    println!("\n{}", "Option 2: Manual revert from live USB".cyan().bold());
    println!("  1. Boot from a live USB");
    println!("  2. Mount your btrfs filesystem:");
    println!("     sudo mount -o subvolid=5 /dev/sdXY /mnt");
    println!("  3. Set the snapshot as default:");
    println!("     sudo btrfs subvolume set-default {} /mnt", snapshot.path.display());
    println!("  4. Unmount and reboot:");
    println!("     sudo umount /mnt && sudo reboot");
    
    println!("\n{}", "Option 3: Copy snapshot contents (Slower but safer)".cyan().bold());
    println!("  The rsync method below will be used instead.");
    
    let use_rsync = Confirm::new()
        .with_prompt("Use rsync to copy snapshot contents? (slower but works from running system)")
        .default(true)
        .interact()
        .unwrap_or(false);
    
    if use_rsync {
        println!("\n  Falling back to rsync method...");
        return revert_rsync_snapshot(snapshot);
    }
    
    println!("\n{}", "Revert cancelled. Please follow the manual steps above.".yellow());
    Err(EshuError::Snapshot(
        "Btrfs revert requires manual steps or rsync fallback".to_string()
    ))
}

fn revert_lvm_snapshot(snapshot: &Snapshot) -> EshuResult<()> {
    println!("  Reverting LVM snapshot...");
    
    let snapshot_name = format!("{}_lv", snapshot.id);
    
    // Check if LVM snapshot exists
    let check_output = Command::new("lvdisplay")
        .arg(&snapshot_name)
        .output()
        .map_err(|e| EshuError::Snapshot(format!("Failed to check LVM snapshot: {}", e)))?;
    
    if !check_output.status.success() {
        return Err(EshuError::Snapshot(
            format!("LVM snapshot '{}' not found", snapshot_name)
        ));
    }
    
    println!("  Merging LVM snapshot...");
    let output = Command::new("lvconvert")
        .args(&["--merge", &snapshot_name])
        .output()
        .map_err(|e| EshuError::Snapshot(format!("LVM merge failed: {}", e)))?;
    
    if !output.status.success() {
        return Err(EshuError::Snapshot(
            format!("LVM merge failed: {}", String::from_utf8_lossy(&output.stderr))
        ));
    }
    
    println!("  {}", "✓ LVM snapshot merge initiated".green());
    println!("  {}", "Note: Merge will complete on next reboot".yellow());
    
    Ok(())
}

fn revert_rsync_snapshot(snapshot: &Snapshot) -> EshuResult<()> {
    println!("  Reverting rsync backup...");
    println!("  {}", "⚠️  This will overwrite current system files!".yellow());
    
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
                &format!("{}/", entry.path().display()), // Trailing slash important!
                target.to_str().unwrap(),
            ])
            .output()
            .map_err(|e| EshuError::Snapshot(format!("Rsync restore failed: {}", e)))?;
        
        if !output.status.success() {
            pb.finish_with_message("Failed");
            return Err(EshuError::Snapshot(
                format!("Rsync restore failed for /{}: {}", 
                    dir_name.to_string_lossy(),
                    String::from_utf8_lossy(&output.stderr))
            ));
        }
    }
    
    pb.finish_with_message("Restore complete");
    
    Ok(())
}
