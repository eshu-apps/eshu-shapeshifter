use crate::config::{DistroFamily, InstalledPackage};
use crate::error::{EshuError, EshuResult};
use rusqlite::{Connection, Result as SqlResult};
use std::collections::HashMap;

/// Package translation database
pub struct PackageTranslator {
    conn: Connection,
}

impl PackageTranslator {
    /// Create a new package translator with embedded mappings
    pub fn new(db_path: &str) -> EshuResult<Self> {
        let conn = Connection::open(db_path)?;
        
        let translator = Self { conn };
        translator.initialize_database()?;
        translator.populate_default_mappings()?;
        
        Ok(translator)
    }

    fn initialize_database(&self) -> EshuResult<()> {
        self.conn.execute(
            "CREATE TABLE IF NOT EXISTS package_mappings (
                id INTEGER PRIMARY KEY,
                source_family TEXT NOT NULL,
                source_package TEXT NOT NULL,
                target_family TEXT NOT NULL,
                target_package TEXT NOT NULL,
                confidence REAL DEFAULT 1.0,
                UNIQUE(source_family, source_package, target_family)
            )",
            [],
        )?;

        self.conn.execute(
            "CREATE INDEX IF NOT EXISTS idx_source ON package_mappings(source_family, source_package)",
            [],
        )?;

        Ok(())
    }

    fn populate_default_mappings(&self) -> EshuResult<()> {
        // Check if already populated
        let count: i64 = self.conn.query_row(
            "SELECT COUNT(*) FROM package_mappings",
            [],
            |row| row.get(0),
        )?;

        if count > 0 {
            return Ok(());
        }

        // Common package mappings
        let mappings = vec![
            // Core system
            ("Debian", "systemd", "Arch", "systemd"),
            ("Debian", "systemd", "RedHat", "systemd"),
            
            // Shells
            ("Debian", "bash", "Arch", "bash"),
            ("Debian", "zsh", "Arch", "zsh"),
            ("Debian", "fish", "Arch", "fish"),
            
            // Text editors
            ("Debian", "vim", "Arch", "vim"),
            ("Debian", "neovim", "Arch", "neovim"),
            ("Debian", "emacs", "Arch", "emacs"),
            ("Debian", "nano", "Arch", "nano"),
            
            // Development tools
            ("Debian", "gcc", "Arch", "gcc"),
            ("Debian", "g++", "Arch", "gcc"),
            ("Debian", "make", "Arch", "make"),
            ("Debian", "cmake", "Arch", "cmake"),
            ("Debian", "git", "Arch", "git"),
            ("Debian", "python3", "Arch", "python"),
            ("Debian", "python3-pip", "Arch", "python-pip"),
            ("Debian", "nodejs", "Arch", "nodejs"),
            ("Debian", "npm", "Arch", "npm"),
            ("Debian", "rust", "Arch", "rust"),
            ("Debian", "cargo", "Arch", "rust"),
            
            // Libraries
            ("Debian", "libssl-dev", "Arch", "openssl"),
            ("Debian", "libcurl4-openssl-dev", "Arch", "curl"),
            ("Debian", "libsqlite3-dev", "Arch", "sqlite"),
            ("Debian", "libpq-dev", "Arch", "postgresql-libs"),
            
            // Network tools
            ("Debian", "curl", "Arch", "curl"),
            ("Debian", "wget", "Arch", "wget"),
            ("Debian", "openssh-server", "Arch", "openssh"),
            ("Debian", "openssh-client", "Arch", "openssh"),
            ("Debian", "net-tools", "Arch", "net-tools"),
            
            // Desktop environments
            ("Debian", "kde-plasma-desktop", "Arch", "plasma-desktop"),
            ("Debian", "gnome", "Arch", "gnome"),
            ("Debian", "xfce4", "Arch", "xfce4"),
            
            // Display servers
            ("Debian", "xorg", "Arch", "xorg-server"),
            ("Debian", "wayland", "Arch", "wayland"),
            
            // Browsers
            ("Debian", "firefox", "Arch", "firefox"),
            ("Debian", "chromium", "Arch", "chromium"),
            
            // Media
            ("Debian", "vlc", "Arch", "vlc"),
            ("Debian", "ffmpeg", "Arch", "ffmpeg"),
            
            // Arch to Debian
            ("Arch", "systemd", "Debian", "systemd"),
            ("Arch", "bash", "Debian", "bash"),
            ("Arch", "vim", "Debian", "vim"),
            ("Arch", "python", "Debian", "python3"),
            ("Arch", "python-pip", "Debian", "python3-pip"),
            ("Arch", "gcc", "Debian", "gcc"),
            ("Arch", "openssh", "Debian", "openssh-server"),
            
            // Arch to RedHat
            ("Arch", "systemd", "RedHat", "systemd"),
            ("Arch", "bash", "RedHat", "bash"),
            ("Arch", "vim", "RedHat", "vim"),
            ("Arch", "python", "RedHat", "python3"),
            ("Arch", "python-pip", "RedHat", "python3-pip"),
            
            // RedHat to Arch
            ("RedHat", "systemd", "Arch", "systemd"),
            ("RedHat", "bash", "Arch", "bash"),
            ("RedHat", "vim", "Arch", "vim"),
            ("RedHat", "python3", "Arch", "python"),
            ("RedHat", "python3-pip", "Arch", "python-pip"),
        ];

        for (src_family, src_pkg, tgt_family, tgt_pkg) in mappings {
            self.conn.execute(
                "INSERT OR IGNORE INTO package_mappings 
                 (source_family, source_package, target_family, target_package, confidence)
                 VALUES (?1, ?2, ?3, ?4, 1.0)",
                [src_family, src_pkg, tgt_family, tgt_pkg],
            )?;
        }

        Ok(())
    }

    /// Translate a package from source distro family to target distro family
    pub fn translate_package(
        &self,
        source_family: &DistroFamily,
        target_family: &DistroFamily,
        package_name: &str,
    ) -> EshuResult<Option<String>> {
        let source_str = format!("{:?}", source_family);
        let target_str = format!("{:?}", target_family);

        let result: SqlResult<String> = self.conn.query_row(
            "SELECT target_package FROM package_mappings 
             WHERE source_family = ?1 AND source_package = ?2 AND target_family = ?3
             ORDER BY confidence DESC LIMIT 1",
            [&source_str, package_name, &target_str],
            |row| row.get(0),
        );

        match result {
            Ok(pkg) => Ok(Some(pkg)),
            Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
            Err(e) => Err(EshuError::Database(e)),
        }
    }

    /// Translate a list of packages
    pub fn translate_packages(
        &self,
        source_family: &DistroFamily,
        target_family: &DistroFamily,
        packages: &[InstalledPackage],
    ) -> EshuResult<TranslationResult> {
        let mut translated = Vec::new();
        let mut untranslated = Vec::new();
        let mut skipped = Vec::new();

        for package in packages {
            // Skip system-critical packages that shouldn't be migrated
            if is_system_package(&package.name) {
                skipped.push(package.name.clone());
                continue;
            }

            match self.translate_package(source_family, target_family, &package.name)? {
                Some(target_pkg) => {
                    translated.push(PackageMapping {
                        source: package.name.clone(),
                        target: target_pkg,
                        confidence: 1.0,
                    });
                }
                None => {
                    // Try fuzzy matching
                    if let Some(fuzzy) = self.fuzzy_match(&package.name, target_family)? {
                        translated.push(fuzzy);
                    } else {
                        untranslated.push(package.name.clone());
                    }
                }
            }
        }

        Ok(TranslationResult {
            translated,
            untranslated,
            skipped,
        })
    }

    /// Attempt fuzzy matching for packages without direct mappings
    fn fuzzy_match(
        &self,
        package_name: &str,
        target_family: &DistroFamily,
    ) -> EshuResult<Option<PackageMapping>> {
        let target_str = format!("{:?}", target_family);

        // Try common patterns
        let patterns = vec![
            package_name.to_string(),
            package_name.replace("-dev", ""),
            package_name.replace("lib", ""),
            package_name.replace("python3-", "python-"),
            package_name.replace("python-", "python3-"),
        ];

        for pattern in patterns {
            let result: SqlResult<String> = self.conn.query_row(
                "SELECT target_package FROM package_mappings 
                 WHERE target_family = ?1 AND (source_package LIKE ?2 OR target_package LIKE ?2)
                 LIMIT 1",
                [&target_str, &format!("%{}%", pattern)],
                |row| row.get(0),
            );

            if let Ok(pkg) = result {
                return Ok(Some(PackageMapping {
                    source: package_name.to_string(),
                    target: pkg,
                    confidence: 0.7,
                }));
            }
        }

        Ok(None)
    }

    /// Add a custom package mapping
    pub fn add_mapping(
        &self,
        source_family: &DistroFamily,
        source_package: &str,
        target_family: &DistroFamily,
        target_package: &str,
        confidence: f64,
    ) -> EshuResult<()> {
        let source_str = format!("{:?}", source_family);
        let target_str = format!("{:?}", target_family);

        self.conn.execute(
            "INSERT OR REPLACE INTO package_mappings 
             (source_family, source_package, target_family, target_package, confidence)
             VALUES (?1, ?2, ?3, ?4, ?5)",
            [&source_str, source_package, &target_str, target_package, &confidence.to_string()],
        )?;

        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct PackageMapping {
    pub source: String,
    pub target: String,
    pub confidence: f64,
}

#[derive(Debug)]
pub struct TranslationResult {
    pub translated: Vec<PackageMapping>,
    pub untranslated: Vec<String>,
    pub skipped: Vec<String>,
}

/// Check if a package is a system-critical package that shouldn't be migrated
fn is_system_package(name: &str) -> bool {
    let system_packages = vec![
        "base",
        "linux",
        "linux-firmware",
        "grub",
        "udev",
        "kernel",
        "initramfs",
        "base-files",
        "dpkg",
        "apt",
        "pacman",
        "dnf",
        "rpm",
    ];

    system_packages.iter().any(|&pkg| name.contains(pkg))
}

/// Get essential packages for a distro family
pub fn get_essential_packages(family: &DistroFamily) -> Vec<String> {
    match family {
        DistroFamily::Arch => vec![
            "base".to_string(),
            "linux".to_string(),
            "linux-firmware".to_string(),
            "base-devel".to_string(),
        ],
        DistroFamily::Debian => vec![
            "base-files".to_string(),
            "base-passwd".to_string(),
            "bash".to_string(),
            "coreutils".to_string(),
        ],
        DistroFamily::RedHat => vec![
            "basesystem".to_string(),
            "bash".to_string(),
            "coreutils".to_string(),
            "systemd".to_string(),
        ],
        _ => vec![],
    }
}
