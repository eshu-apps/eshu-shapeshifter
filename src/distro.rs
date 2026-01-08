use crate::config::{DistroFamily, DistroProfile, InitSystem, PackageManager};
use crate::error::{EshuError, EshuResult};
use std::fs;
use std::process::Command;

/// Detect the current Linux distribution
pub fn detect_current_distro() -> EshuResult<(String, String, DistroFamily)> {
    // Try /etc/os-release first (standard)
    if let Ok(content) = fs::read_to_string("/etc/os-release") {
        return parse_os_release(&content);
    }

    // Fallback to /etc/lsb-release
    if let Ok(content) = fs::read_to_string("/etc/lsb-release") {
        return parse_lsb_release(&content);
    }

    // Check specific distro files
    if fs::metadata("/etc/arch-release").is_ok() {
        return Ok(("Arch Linux".to_string(), "rolling".to_string(), DistroFamily::Arch));
    }

    if fs::metadata("/etc/debian_version").is_ok() {
        let version = fs::read_to_string("/etc/debian_version")
            .unwrap_or_else(|_| "unknown".to_string())
            .trim()
            .to_string();
        return Ok(("Debian".to_string(), version, DistroFamily::Debian));
    }

    if fs::metadata("/etc/redhat-release").is_ok() {
        let content = fs::read_to_string("/etc/redhat-release")
            .unwrap_or_else(|_| "unknown".to_string());
        return parse_redhat_release(&content);
    }

    Err(EshuError::UnsupportedDistro("Unable to detect distribution".to_string()))
}

fn parse_os_release(content: &str) -> EshuResult<(String, String, DistroFamily)> {
    let mut name = String::new();
    let mut version = String::new();
    let mut id = String::new();

    for line in content.lines() {
        if let Some(value) = line.strip_prefix("NAME=") {
            name = value.trim_matches('"').to_string();
        } else if let Some(value) = line.strip_prefix("VERSION_ID=") {
            version = value.trim_matches('"').to_string();
        } else if let Some(value) = line.strip_prefix("ID=") {
            id = value.trim_matches('"').to_string();
        }
    }

    let family = match id.to_lowercase().as_str() {
        "arch" | "manjaro" | "endeavouros" | "garuda" => DistroFamily::Arch,
        "ubuntu" | "debian" | "linuxmint" | "pop" | "elementary" => DistroFamily::Debian,
        "fedora" | "rhel" | "centos" | "rocky" | "almalinux" => DistroFamily::RedHat,
        "opensuse" | "sles" => DistroFamily::Suse,
        "gentoo" => DistroFamily::Gentoo,
        "alpine" => DistroFamily::Alpine,
        "void" => DistroFamily::Void,
        "nixos" => DistroFamily::NixOS,
        _ => DistroFamily::Other(id.clone()),
    };

    if name.is_empty() {
        return Err(EshuError::UnsupportedDistro("Could not parse distribution name".to_string()));
    }

    Ok((name, version, family))
}

fn parse_lsb_release(content: &str) -> EshuResult<(String, String, DistroFamily)> {
    let mut name = String::new();
    let mut version = String::new();

    for line in content.lines() {
        if let Some(value) = line.strip_prefix("DISTRIB_ID=") {
            name = value.trim_matches('"').to_string();
        } else if let Some(value) = line.strip_prefix("DISTRIB_RELEASE=") {
            version = value.trim_matches('"').to_string();
        }
    }

    let family = match name.to_lowercase().as_str() {
        "arch" => DistroFamily::Arch,
        "ubuntu" | "debian" => DistroFamily::Debian,
        "fedora" | "redhat" => DistroFamily::RedHat,
        _ => DistroFamily::Other(name.clone()),
    };

    Ok((name, version, family))
}

fn parse_redhat_release(content: &str) -> EshuResult<(String, String, DistroFamily)> {
    // Example: "Red Hat Enterprise Linux release 8.5 (Ootpa)"
    let parts: Vec<&str> = content.split_whitespace().collect();
    let name = parts.iter()
        .take_while(|&&s| s != "release")
        .cloned()
        .collect::<Vec<_>>()
        .join(" ");
    
    let version = parts.iter()
        .skip_while(|&&s| s != "release")
        .nth(1)
        .unwrap_or(&"unknown")
        .to_string();

    Ok((name, version, DistroFamily::RedHat))
}

/// Detect the package manager for the current system
pub fn detect_package_manager() -> EshuResult<PackageManager> {
    let managers = vec![
        ("pacman", PackageManager {
            name: "pacman".to_string(),
            install_cmd: "pacman -S --noconfirm".to_string(),
            remove_cmd: "pacman -R --noconfirm".to_string(),
            update_cmd: "pacman -Syu --noconfirm".to_string(),
            search_cmd: "pacman -Ss".to_string(),
            list_installed_cmd: "pacman -Q".to_string(),
        }),
        ("apt", PackageManager {
            name: "apt".to_string(),
            install_cmd: "apt install -y".to_string(),
            remove_cmd: "apt remove -y".to_string(),
            update_cmd: "apt update && apt upgrade -y".to_string(),
            search_cmd: "apt search".to_string(),
            list_installed_cmd: "dpkg -l".to_string(),
        }),
        ("dnf", PackageManager {
            name: "dnf".to_string(),
            install_cmd: "dnf install -y".to_string(),
            remove_cmd: "dnf remove -y".to_string(),
            update_cmd: "dnf upgrade -y".to_string(),
            search_cmd: "dnf search".to_string(),
            list_installed_cmd: "dnf list installed".to_string(),
        }),
        ("zypper", PackageManager {
            name: "zypper".to_string(),
            install_cmd: "zypper install -y".to_string(),
            remove_cmd: "zypper remove -y".to_string(),
            update_cmd: "zypper update -y".to_string(),
            search_cmd: "zypper search".to_string(),
            list_installed_cmd: "zypper packages --installed-only".to_string(),
        }),
    ];

    for (cmd, manager) in managers {
        // Try to execute the command with --version to check if it exists
        if Command::new(cmd).arg("--version").output().is_ok() {
            return Ok(manager);
        }
    }

    Err(EshuError::PackageManager("No supported package manager found".to_string()))
}

/// Detect the init system
pub fn detect_init_system() -> InitSystem {
    if fs::metadata("/run/systemd/system").is_ok() {
        return InitSystem::Systemd;
    }

    if fs::metadata("/etc/init.d").is_ok() && fs::metadata("/sbin/openrc").is_ok() {
        return InitSystem::OpenRC;
    }

    if fs::metadata("/etc/runit").is_ok() {
        return InitSystem::Runit;
    }

    InitSystem::Other("unknown".to_string())
}

/// Get kernel version
pub fn get_kernel_version() -> String {
    let output = Command::new("uname")
        .arg("-r")
        .output()
        .ok();

    output
        .and_then(|o| String::from_utf8(o.stdout).ok())
        .map(|s| s.trim().to_string())
        .unwrap_or_else(|| "unknown".to_string())
}

/// Get system architecture
pub fn get_architecture() -> String {
    let output = Command::new("uname")
        .arg("-m")
        .output()
        .ok();

    output
        .and_then(|o| String::from_utf8(o.stdout).ok())
        .map(|s| s.trim().to_string())
        .unwrap_or_else(|| "unknown".to_string())
}

/// Detect filesystem type for root partition
pub fn detect_filesystem() -> String {
    let output = Command::new("findmnt")
        .args(&["-n", "-o", "FSTYPE", "/"])
        .output()
        .ok();

    output
        .and_then(|o| String::from_utf8(o.stdout).ok())
        .map(|s| s.trim().to_string())
        .unwrap_or_else(|| "unknown".to_string())
}

/// Detect bootloader
pub fn detect_bootloader() -> String {
    if fs::metadata("/boot/grub").is_ok() || fs::metadata("/boot/grub2").is_ok() {
        return "GRUB".to_string();
    }

    if fs::metadata("/boot/loader/entries").is_ok() {
        return "systemd-boot".to_string();
    }

    if fs::metadata("/boot/syslinux").is_ok() {
        return "SYSLINUX".to_string();
    }

    "unknown".to_string()
}
