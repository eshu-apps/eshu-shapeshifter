use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;

/// Main configuration for Eshu Shapeshifter
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct EshuConfig {
    pub version: String,
    pub data_dir: PathBuf,
    pub snapshot_dir: PathBuf,
    pub repository_url: String,
    pub cache_dir: PathBuf,
}

impl Default for EshuConfig {
    fn default() -> Self {
        Self {
            version: env!("CARGO_PKG_VERSION").to_string(),
            data_dir: PathBuf::from("/var/lib/eshu-shapeshifter"),
            snapshot_dir: PathBuf::from("/var/lib/eshu-shapeshifter/snapshots"),
            repository_url: "https://raw.githubusercontent.com/eshu-apps/eshu-shapeshifter/main/profiles".to_string(),
            cache_dir: PathBuf::from("/var/cache/eshu-shapeshifter"),
        }
    }
}

/// Distribution profile defining how to handle a specific distro
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DistroProfile {
    pub name: String,
    pub version: String,
    pub family: DistroFamily,
    pub package_manager: PackageManager,
    pub init_system: InitSystem,
    pub base_packages: Vec<String>,
    pub config_paths: HashMap<String, String>,
    pub pre_migration_hooks: Vec<String>,
    pub post_migration_hooks: Vec<String>,
    pub package_mappings: HashMap<String, String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum DistroFamily {
    Debian,
    RedHat,
    Arch,
    Suse,
    Gentoo,
    Alpine,
    Void,
    Nix,
    NixOS,
    Other(String),
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PackageManager {
    pub name: String,
    pub install_cmd: String,
    pub remove_cmd: String,
    pub update_cmd: String,
    pub search_cmd: String,
    pub list_installed_cmd: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum InitSystem {
    Systemd,
    OpenRC,
    Runit,
    SysVinit,
    Other(String),
}

/// System snapshot metadata
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Snapshot {
    pub id: String,
    pub timestamp: i64,
    pub distro_name: String,
    pub distro_version: String,
    pub description: String,
    pub snapshot_type: SnapshotType,
    pub size_bytes: u64,
    pub path: PathBuf,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum SnapshotType {
    Btrfs,
    LVM,
    Rsync,
}

/// Current system state
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SystemState {
    pub distro: String,
    pub version: String,
    pub family: DistroFamily,
    pub kernel: String,
    pub architecture: String,
    pub installed_packages: Vec<InstalledPackage>,
    pub services: Vec<Service>,
    pub users: Vec<User>,
    pub filesystem_type: String,
    pub boot_loader: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct InstalledPackage {
    pub name: String,
    pub version: String,
    pub description: Option<String>,
    pub dependencies: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Service {
    pub name: String,
    pub enabled: bool,
    pub running: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct User {
    pub name: String,
    pub uid: u32,
    pub gid: u32,
    pub home: PathBuf,
    pub shell: String,
}

impl EshuConfig {
    pub fn load() -> anyhow::Result<Self> {
        let config_path = PathBuf::from("/etc/eshu-shapeshifter/config.toml");
        
        if config_path.exists() {
            let content = std::fs::read_to_string(&config_path)?;
            Ok(toml::from_str(&content)?)
        } else {
            let config = Self::default();
            config.save()?;
            Ok(config)
        }
    }

    pub fn save(&self) -> anyhow::Result<()> {
        let config_dir = PathBuf::from("/etc/eshu-shapeshifter");
        std::fs::create_dir_all(&config_dir)?;
        
        let config_path = config_dir.join("config.toml");
        let content = toml::to_string_pretty(self)?;
        std::fs::write(config_path, content)?;
        
        // Create necessary directories
        std::fs::create_dir_all(&self.data_dir)?;
        std::fs::create_dir_all(&self.snapshot_dir)?;
        std::fs::create_dir_all(&self.cache_dir)?;
        
        Ok(())
    }
}
