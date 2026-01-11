use crate::config::{DistroFamily, SystemState};
use crate::error::EshuResult;
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};

/// Configuration file translator
pub struct ConfigTranslator {
    translations: HashMap<String, ConfigTranslation>,
}

#[derive(Debug, Clone)]
pub struct ConfigTranslation {
    pub source_path: PathBuf,
    pub target_path: PathBuf,
    pub transform: TransformType,
}

#[derive(Debug, Clone)]
pub enum TransformType {
    Copy,
    Merge,
    Transform(fn(&str) -> String),
    Skip,
}

impl ConfigTranslator {
    pub fn new() -> Self {
        Self {
            translations: HashMap::new(),
        }
    }

    /// Build translation rules based on source and target distros
    pub fn build_rules(
        &mut self,
        source_family: &DistroFamily,
        target_family: &DistroFamily,
    ) {
        // Network configuration
        self.add_network_rules(source_family, target_family);
        
        // User configuration
        self.add_user_rules();
        
        // Service configuration
        self.add_service_rules(source_family, target_family);
        
        // Package manager configuration
        self.add_package_manager_rules(source_family, target_family);
    }

    fn add_network_rules(&mut self, source: &DistroFamily, target: &DistroFamily) {
        match (source, target) {
            (DistroFamily::Debian, DistroFamily::Arch) => {
                self.translations.insert(
                    "network".to_string(),
                    ConfigTranslation {
                        source_path: PathBuf::from("/etc/network/interfaces"),
                        target_path: PathBuf::from("/etc/systemd/network/"),
                        transform: TransformType::Transform(debian_to_systemd_network),
                    },
                );
            }
            (DistroFamily::Arch, DistroFamily::Debian) => {
                self.translations.insert(
                    "network".to_string(),
                    ConfigTranslation {
                        source_path: PathBuf::from("/etc/systemd/network/"),
                        target_path: PathBuf::from("/etc/network/interfaces"),
                        transform: TransformType::Transform(systemd_to_debian_network),
                    },
                );
            }
            _ => {}
        }
    }

    fn add_user_rules(&mut self) {
        // User files are generally compatible across distros
        self.translations.insert(
            "passwd".to_string(),
            ConfigTranslation {
                source_path: PathBuf::from("/etc/passwd"),
                target_path: PathBuf::from("/etc/passwd"),
                transform: TransformType::Merge,
            },
        );

        self.translations.insert(
            "shadow".to_string(),
            ConfigTranslation {
                source_path: PathBuf::from("/etc/shadow"),
                target_path: PathBuf::from("/etc/shadow"),
                transform: TransformType::Merge,
            },
        );

        self.translations.insert(
            "group".to_string(),
            ConfigTranslation {
                source_path: PathBuf::from("/etc/group"),
                target_path: PathBuf::from("/etc/group"),
                transform: TransformType::Merge,
            },
        );
    }

    fn add_service_rules(&mut self, source: &DistroFamily, target: &DistroFamily) {
        // Most modern distros use systemd
        if matches!(source, DistroFamily::Debian | DistroFamily::Arch | DistroFamily::RedHat)
            && matches!(target, DistroFamily::Debian | DistroFamily::Arch | DistroFamily::RedHat)
        {
            self.translations.insert(
                "systemd".to_string(),
                ConfigTranslation {
                    source_path: PathBuf::from("/etc/systemd/system/"),
                    target_path: PathBuf::from("/etc/systemd/system/"),
                    transform: TransformType::Copy,
                },
            );
        }
    }

    fn add_package_manager_rules(&mut self, source: &DistroFamily, target: &DistroFamily) {
        // Package manager configs are distro-specific and shouldn't be copied
        match target {
            DistroFamily::Arch => {
                self.translations.insert(
                    "pacman".to_string(),
                    ConfigTranslation {
                        source_path: PathBuf::from("/etc/pacman.conf"),
                        target_path: PathBuf::from("/etc/pacman.conf"),
                        transform: TransformType::Skip,
                    },
                );
            }
            DistroFamily::Debian => {
                self.translations.insert(
                    "apt".to_string(),
                    ConfigTranslation {
                        source_path: PathBuf::from("/etc/apt/"),
                        target_path: PathBuf::from("/etc/apt/"),
                        transform: TransformType::Skip,
                    },
                );
            }
            _ => {}
        }
    }

    /// Translate configuration files from source to target
    pub fn translate_configs(&self, backup_dir: &Path) -> EshuResult<Vec<ConfigOperation>> {
        let mut operations = Vec::new();

        for (name, translation) in &self.translations {
            match &translation.transform {
                TransformType::Copy => {
                    if translation.source_path.exists() {
                        operations.push(ConfigOperation::Copy {
                            from: translation.source_path.clone(),
                            to: translation.target_path.clone(),
                        });
                    }
                }
                TransformType::Merge => {
                    if translation.source_path.exists() {
                        operations.push(ConfigOperation::Merge {
                            source: translation.source_path.clone(),
                            target: translation.target_path.clone(),
                            backup: backup_dir.join(name),
                        });
                    }
                }
                TransformType::Transform(func) => {
                    if translation.source_path.exists() {
                        operations.push(ConfigOperation::Transform {
                            source: translation.source_path.clone(),
                            target: translation.target_path.clone(),
                            transform: *func,
                        });
                    }
                }
                TransformType::Skip => {
                    // Do nothing
                }
            }
        }

        Ok(operations)
    }
}

#[derive(Debug)]
pub enum ConfigOperation {
    Copy {
        from: PathBuf,
        to: PathBuf,
    },
    Merge {
        source: PathBuf,
        target: PathBuf,
        backup: PathBuf,
    },
    Transform {
        source: PathBuf,
        target: PathBuf,
        transform: fn(&str) -> String,
    },
}

impl ConfigOperation {
    pub fn execute(&self) -> EshuResult<()> {
        match self {
            ConfigOperation::Copy { from, to } => {
                if let Some(parent) = to.parent() {
                    fs::create_dir_all(parent)?;
                }
                fs::copy(from, to)?;
            }
            ConfigOperation::Merge { source, target, backup } => {
                // Backup existing target
                if target.exists() {
                    if let Some(parent) = backup.parent() {
                        fs::create_dir_all(parent)?;
                    }
                    fs::copy(target, backup)?;
                }

                // Read both files
                let source_content = fs::read_to_string(source)?;
                let target_content = if target.exists() {
                    fs::read_to_string(target)?
                } else {
                    String::new()
                };

                // Merge (simple append for now - could be more sophisticated)
                let merged = merge_configs(&source_content, &target_content);
                fs::write(target, merged)?;
            }
            ConfigOperation::Transform { source, target, transform } => {
                let content = fs::read_to_string(source)?;
                let transformed = transform(&content);
                
                if let Some(parent) = target.parent() {
                    fs::create_dir_all(parent)?;
                }
                fs::write(target, transformed)?;
            }
        }
        Ok(())
    }
}

/// Merge two configuration files
fn merge_configs(source: &str, target: &str) -> String {
    let mut result = target.to_string();
    
    if !result.is_empty() && !result.ends_with('\n') {
        result.push('\n');
    }
    
    result.push_str("\n# Merged from previous system\n");
    result.push_str(source);
    
    result
}

/// Transform Debian network config to systemd-networkd
fn debian_to_systemd_network(content: &str) -> String {
    // This is a simplified transformation
    // In production, you'd need proper parsing
    let mut output = String::new();
    
    output.push_str("[Match]\n");
    output.push_str("Name=eth0\n\n");
    output.push_str("[Network]\n");
    
    for line in content.lines() {
        if line.contains("address") {
            if let Some(addr) = line.split_whitespace().nth(1) {
                output.push_str(&format!("Address={}\n", addr));
            }
        } else if line.contains("gateway") {
            if let Some(gw) = line.split_whitespace().nth(1) {
                output.push_str(&format!("Gateway={}\n", gw));
            }
        } else if line.contains("dns-nameservers") {
            if let Some(dns) = line.split_whitespace().nth(1) {
                output.push_str(&format!("DNS={}\n", dns));
            }
        }
    }
    
    output
}

/// Transform systemd-networkd config to Debian network interfaces
fn systemd_to_debian_network(content: &str) -> String {
    let mut output = String::new();
    
    output.push_str("auto eth0\n");
    output.push_str("iface eth0 inet static\n");
    
    for line in content.lines() {
        if line.starts_with("Address=") {
            if let Some(addr) = line.strip_prefix("Address=") {
                output.push_str(&format!("    address {}\n", addr));
            }
        } else if line.starts_with("Gateway=") {
            if let Some(gw) = line.strip_prefix("Gateway=") {
                output.push_str(&format!("    gateway {}\n", gw));
            }
        } else if line.starts_with("DNS=") {
            if let Some(dns) = line.strip_prefix("DNS=") {
                output.push_str(&format!("    dns-nameservers {}\n", dns));
            }
        }
    }
    
    output
}

/// Preserve user home directories
pub fn preserve_home_directories(state: &SystemState, backup_dir: &Path) -> EshuResult<()> {
    // Ensure backup directory exists
    fs::create_dir_all(backup_dir)?;
    
    let mut preserved_count = 0;
    let mut skipped_count = 0;
    
    for user in &state.users {
        // Only backup regular users (UID >= 1000) and skip system users
        if user.uid < 1000 {
            continue;
        }
        
        // Check if home directory exists
        if !user.home.exists() {
            println!("  ⚠️  Skipping {}: home directory doesn't exist ({})", 
                user.name, user.home.display());
            skipped_count += 1;
            continue;
        }
        
        // Check if home directory is accessible
        if let Err(e) = fs::read_dir(&user.home) {
            println!("  ⚠️  Skipping {}: cannot access home directory ({})", 
                user.name, e);
            skipped_count += 1;
            continue;
        }
        
        let backup_path = backup_dir.join(format!("home_{}", user.name));
        
        // Create backup directory
        if let Err(e) = fs::create_dir_all(&backup_path) {
            println!("  ⚠️  Skipping {}: failed to create backup directory ({})", 
                user.name, e);
            skipped_count += 1;
            continue;
        }
        
        // Copy user home directory
        match copy_dir_recursive(&user.home, &backup_path) {
            Ok(_) => {
                println!("  ✓ Preserved home directory for user: {}", user.name);
                preserved_count += 1;
            }
            Err(e) => {
                println!("  ⚠️  Partial backup for {}: {} (continuing anyway)", 
                    user.name, e);
                // Don't fail the whole operation, just warn
                skipped_count += 1;
            }
        }
    }
    
    if preserved_count == 0 && skipped_count == 0 {
        println!("  ℹ️  No user home directories to preserve");
    } else {
        println!("  Summary: {} preserved, {} skipped", preserved_count, skipped_count);
    }
    
    Ok(())
}

/// Recursively copy a directory
fn copy_dir_recursive(src: &Path, dst: &Path) -> std::io::Result<()> {
    if !dst.exists() {
        fs::create_dir_all(dst)?;
    }
    
    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let path = entry.path();
        let dest_path = dst.join(entry.file_name());
        
        // Skip certain directories that are large or unnecessary
        let filename = entry.file_name();
        let filename_str = filename.to_string_lossy();
        if filename_str == ".cache" 
            || filename_str == ".local/share/Trash"
            || filename_str == ".thumbnails"
            || filename_str == ".mozilla/firefox/*/Cache"
            || filename_str == ".config/google-chrome/*/Cache"
        {
            continue;
        }
        
        if path.is_dir() {
            // Recursively copy subdirectories, but catch errors to continue
            if let Err(e) = copy_dir_recursive(&path, &dest_path) {
                eprintln!("    Warning: Failed to copy {}: {}", path.display(), e);
                // Continue with other files
            }
        } else {
            // Copy file, but catch errors to continue
            if let Err(e) = fs::copy(&path, &dest_path) {
                eprintln!("    Warning: Failed to copy {}: {}", path.display(), e);
                // Continue with other files
            }
        }
    }
    
    Ok(())
}
