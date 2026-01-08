use crate::config::{EshuConfig, InstalledPackage, Service, SystemState, User};
use crate::distro;
use crate::error::EshuResult;
use colored::Colorize;
use std::fs;
use std::path::PathBuf;
use std::process::Command;

/// Scan the current system and display comprehensive information
pub async fn scan_system() -> anyhow::Result<()> {
    println!("{}", "\n🔍 Scanning system...".cyan().bold());
    
    let state = collect_system_state().await?;
    
    display_system_info(&state);
    
    // Save state to disk
    let config = EshuConfig::load()?;
    let state_path = config.data_dir.join("current_state.json");
    std::fs::create_dir_all(&config.data_dir)?;
    let json = serde_json::to_string_pretty(&state)?;
    std::fs::write(state_path, json)?;
    
    println!("{}", "\n✅ System scan complete!".green().bold());
    
    Ok(())
}

/// Show current system status
pub async fn show_status() -> anyhow::Result<()> {
    let config = EshuConfig::load()?;
    let state_path = config.data_dir.join("current_state.json");
    
    if !state_path.exists() {
        println!("{}", "⚠️  No system state found. Run 'scan' first.".yellow());
        return Ok(());
    }
    
    let content = fs::read_to_string(state_path)?;
    let state: SystemState = serde_json::from_str(&content)?;
    
    display_system_info(&state);
    
    // Show transformation history
    let history_path = config.data_dir.join("history.json");
    if history_path.exists() {
        println!("\n{}", "📜 Transformation History:".cyan().bold());
        let history_content = fs::read_to_string(history_path)?;
        if let Ok(history) = serde_json::from_str::<Vec<TransformationRecord>>(&history_content) {
            for record in history.iter().rev().take(5) {
                println!("  {} → {} ({})", 
                    record.from_distro.yellow(),
                    record.to_distro.green(),
                    record.timestamp
                );
            }
        }
    }
    
    Ok(())
}

#[derive(serde::Serialize, serde::Deserialize)]
struct TransformationRecord {
    from_distro: String,
    to_distro: String,
    timestamp: String,
    snapshot_id: String,
}

/// Collect comprehensive system state
pub async fn collect_system_state() -> EshuResult<SystemState> {
    let (distro, version, family) = distro::detect_current_distro()?;
    let kernel = distro::get_kernel_version();
    let architecture = distro::get_architecture();
    let filesystem_type = distro::detect_filesystem();
    let boot_loader = distro::detect_bootloader();
    
    let installed_packages = collect_installed_packages().await?;
    let services = collect_services().await?;
    let users = collect_users().await?;
    
    Ok(SystemState {
        distro,
        version,
        family,
        kernel,
        architecture,
        installed_packages,
        services,
        users,
        filesystem_type,
        boot_loader,
    })
}

async fn collect_installed_packages() -> EshuResult<Vec<InstalledPackage>> {
    let pm = distro::detect_package_manager()?;
    let mut packages = Vec::new();
    
    let output = Command::new("sh")
        .arg("-c")
        .arg(&pm.list_installed_cmd)
        .output();
    
    if let Ok(output) = output {
        let stdout = String::from_utf8_lossy(&output.stdout);
        
        match pm.name.as_str() {
            "pacman" => {
                for line in stdout.lines() {
                    let parts: Vec<&str> = line.split_whitespace().collect();
                    if parts.len() >= 2 {
                        packages.push(InstalledPackage {
                            name: parts[0].to_string(),
                            version: parts[1].to_string(),
                            description: None,
                            dependencies: Vec::new(),
                        });
                    }
                }
            }
            "apt" => {
                for line in stdout.lines() {
                    if line.starts_with("ii") {
                        let parts: Vec<&str> = line.split_whitespace().collect();
                        if parts.len() >= 3 {
                            packages.push(InstalledPackage {
                                name: parts[1].to_string(),
                                version: parts[2].to_string(),
                                description: parts.get(3..).map(|s| s.join(" ")),
                                dependencies: Vec::new(),
                            });
                        }
                    }
                }
            }
            "dnf" => {
                for line in stdout.lines() {
                    let parts: Vec<&str> = line.split_whitespace().collect();
                    if parts.len() >= 2 && !line.starts_with("Installed") {
                        let name_arch: Vec<&str> = parts[0].split('.').collect();
                        packages.push(InstalledPackage {
                            name: name_arch[0].to_string(),
                            version: parts[1].to_string(),
                            description: None,
                            dependencies: Vec::new(),
                        });
                    }
                }
            }
            _ => {}
        }
    }
    
    Ok(packages)
}

async fn collect_services() -> EshuResult<Vec<Service>> {
    let mut services = Vec::new();
    
    let init = distro::detect_init_system();
    
    match init {
        crate::config::InitSystem::Systemd => {
            let output = Command::new("systemctl")
                .args(&["list-unit-files", "--type=service", "--no-pager"])
                .output();
            
            if let Ok(output) = output {
                let stdout = String::from_utf8_lossy(&output.stdout);
                for line in stdout.lines().skip(1) {
                    let parts: Vec<&str> = line.split_whitespace().collect();
                    if parts.len() >= 2 {
                        let name = parts[0].trim_end_matches(".service").to_string();
                        let enabled = parts[1] == "enabled";
                        
                        // Check if running
                        let running = Command::new("systemctl")
                            .args(&["is-active", &format!("{}.service", name)])
                            .output()
                            .map(|o| String::from_utf8_lossy(&o.stdout).trim() == "active")
                            .unwrap_or(false);
                        
                        services.push(Service {
                            name,
                            enabled,
                            running,
                        });
                    }
                }
            }
        }
        _ => {
            // Basic service detection for other init systems
            if let Ok(entries) = fs::read_dir("/etc/init.d") {
                for entry in entries.flatten() {
                    if let Ok(name) = entry.file_name().into_string() {
                        services.push(Service {
                            name,
                            enabled: false,
                            running: false,
                        });
                    }
                }
            }
        }
    }
    
    Ok(services)
}

async fn collect_users() -> EshuResult<Vec<User>> {
    let mut users = Vec::new();
    
    if let Ok(content) = fs::read_to_string("/etc/passwd") {
        for line in content.lines() {
            let parts: Vec<&str> = line.split(':').collect();
            if parts.len() >= 7 {
                let uid: u32 = parts[2].parse().unwrap_or(0);
                let gid: u32 = parts[3].parse().unwrap_or(0);
                
                // Only include real users (UID >= 1000) and root
                if uid >= 1000 || uid == 0 {
                    users.push(User {
                        name: parts[0].to_string(),
                        uid,
                        gid,
                        home: PathBuf::from(parts[5]),
                        shell: parts[6].to_string(),
                    });
                }
            }
        }
    }
    
    Ok(users)
}

fn display_system_info(state: &SystemState) {
    println!("\n{}", "═══════════════════════════════════════".cyan());
    println!("{}", "           SYSTEM INFORMATION          ".cyan().bold());
    println!("{}", "═══════════════════════════════════════".cyan());
    
    println!("\n{}", "📦 Distribution:".yellow().bold());
    println!("  Name:         {}", state.distro.green());
    println!("  Version:      {}", state.version.green());
    println!("  Family:       {:?}", state.family);
    
    println!("\n{}", "🖥️  System:".yellow().bold());
    println!("  Kernel:       {}", state.kernel);
    println!("  Architecture: {}", state.architecture);
    println!("  Filesystem:   {}", state.filesystem_type);
    println!("  Bootloader:   {}", state.boot_loader);
    
    println!("\n{}", "📚 Packages:".yellow().bold());
    println!("  Installed:    {} packages", state.installed_packages.len());
    
    println!("\n{}", "⚙️  Services:".yellow().bold());
    let enabled_count = state.services.iter().filter(|s| s.enabled).count();
    let running_count = state.services.iter().filter(|s| s.running).count();
    println!("  Total:        {}", state.services.len());
    println!("  Enabled:      {}", enabled_count);
    println!("  Running:      {}", running_count);
    
    println!("\n{}", "👥 Users:".yellow().bold());
    for user in &state.users {
        println!("  {} (UID: {}, Home: {})", 
            user.name.cyan(), 
            user.uid, 
            user.home.display()
        );
    }
}
