use crate::config::{DistroProfile, EshuConfig};
use colored::Colorize;
use reqwest;
use std::fs;

/// List available distributions from the repository
pub async fn list_available_distros() -> anyhow::Result<()> {
    println!("{}", "\n📚 Available Distributions:".cyan().bold());
    println!("{}", "═══════════════════════════════════════".cyan());

    let distros = get_curated_distros();

    for distro in distros {
        println!("\n{}: {}", "Name".yellow(), distro.name.green().bold());
        println!("  {}: {}", "Version".yellow(), distro.version);
        println!("  {}: {:?}", "Family".yellow(), distro.family);
        println!("  {}: {:?}", "Package Manager".yellow(), distro.package_manager.name);
        println!("  {}: {:?}", "Init System".yellow(), distro.init_system);
    }

    println!("\n{}", "💡 Tip: Use 'eshu-shapeshifter shapeshift <name>' to transform".cyan());

    Ok(())
}

/// Get curated list of supported distributions
pub fn get_curated_distros() -> Vec<DistroProfile> {
    vec![
        create_arch_profile(),
        create_ubuntu_profile(),
        create_debian_profile(),
        create_fedora_profile(),
        create_opensuse_profile(),
        create_kali_profile(),
        create_hyprland_profile(),
        create_garuda_dragonized_profile(),
        create_nixos_profile(),
        create_pop_cosmic_profile(),
    ]
}

/// Get a specific distro profile by name
pub fn get_distro_profile(name: &str) -> Option<DistroProfile> {
    get_curated_distros()
        .into_iter()
        .find(|d| d.name.to_lowercase().contains(&name.to_lowercase()))
}

/// Download distro profile from remote repository
pub async fn download_distro_profile(name: &str) -> anyhow::Result<DistroProfile> {
    let config = EshuConfig::load()?;
    let url = format!("{}/profiles/{}.toml", config.repository_url, name.to_lowercase());

    println!("Downloading profile from: {}", url);

    let response = reqwest::get(&url).await?;
    let content = response.text().await?;

    let profile: DistroProfile = toml::from_str(&content)?;

    // Cache the profile
    let cache_path = config.cache_dir.join(format!("{}.toml", name));
    fs::create_dir_all(&config.cache_dir)?;
    fs::write(cache_path, content)?;

    Ok(profile)
}

fn create_arch_profile() -> DistroProfile {
    use crate::config::{DistroFamily, InitSystem, PackageManager};
    use std::collections::HashMap;

    DistroProfile {
        name: "Arch Linux".to_string(),
        version: "rolling".to_string(),
        family: DistroFamily::Arch,
        package_manager: PackageManager {
            name: "pacman".to_string(),
            install_cmd: "pacman -S --noconfirm".to_string(),
            remove_cmd: "pacman -R --noconfirm".to_string(),
            update_cmd: "pacman -Syu --noconfirm".to_string(),
            search_cmd: "pacman -Ss".to_string(),
            list_installed_cmd: "pacman -Q".to_string(),
        },
        init_system: InitSystem::Systemd,
        base_packages: vec![
            "base".to_string(),
            "linux".to_string(),
            "linux-firmware".to_string(),
            "base-devel".to_string(),
            "networkmanager".to_string(),
        ],
        config_paths: HashMap::new(),
        pre_migration_hooks: vec![
            "pacman-key --init".to_string(),
            "pacman-key --populate archlinux".to_string(),
        ],
        post_migration_hooks: vec![
            "systemctl enable NetworkManager".to_string(),
            "mkinitcpio -P".to_string(),
            "grub-mkconfig -o /boot/grub/grub.cfg".to_string(),
        ],
        package_mappings: HashMap::new(),
    }
}

fn create_ubuntu_profile() -> DistroProfile {
    use crate::config::{DistroFamily, InitSystem, PackageManager};
    use std::collections::HashMap;

    DistroProfile {
        name: "Ubuntu".to_string(),
        version: "22.04".to_string(),
        family: DistroFamily::Debian,
        package_manager: PackageManager {
            name: "apt".to_string(),
            install_cmd: "apt install -y".to_string(),
            remove_cmd: "apt remove -y".to_string(),
            update_cmd: "apt update && apt upgrade -y".to_string(),
            search_cmd: "apt search".to_string(),
            list_installed_cmd: "dpkg -l".to_string(),
        },
        init_system: InitSystem::Systemd,
        base_packages: vec![
            "ubuntu-minimal".to_string(),
            "linux-generic".to_string(),
            "network-manager".to_string(),
        ],
        config_paths: HashMap::new(),
        pre_migration_hooks: vec![
            "apt update".to_string(),
        ],
        post_migration_hooks: vec![
            "systemctl enable NetworkManager".to_string(),
            "update-grub".to_string(),
        ],
        package_mappings: HashMap::new(),
    }
}

fn create_debian_profile() -> DistroProfile {
    use crate::config::{DistroFamily, InitSystem, PackageManager};
    use std::collections::HashMap;

    DistroProfile {
        name: "Debian".to_string(),
        version: "12".to_string(),
        family: DistroFamily::Debian,
        package_manager: PackageManager {
            name: "apt".to_string(),
            install_cmd: "apt install -y".to_string(),
            remove_cmd: "apt remove -y".to_string(),
            update_cmd: "apt update && apt upgrade -y".to_string(),
            search_cmd: "apt search".to_string(),
            list_installed_cmd: "dpkg -l".to_string(),
        },
        init_system: InitSystem::Systemd,
        base_packages: vec![
            "base-files".to_string(),
            "linux-image-amd64".to_string(),
            "network-manager".to_string(),
        ],
        config_paths: HashMap::new(),
        pre_migration_hooks: vec![
            "apt update".to_string(),
        ],
        post_migration_hooks: vec![
            "systemctl enable NetworkManager".to_string(),
            "update-grub".to_string(),
        ],
        package_mappings: HashMap::new(),
    }
}

fn create_fedora_profile() -> DistroProfile {
    use crate::config::{DistroFamily, InitSystem, PackageManager};
    use std::collections::HashMap;

    DistroProfile {
        name: "Fedora".to_string(),
        version: "39".to_string(),
        family: DistroFamily::RedHat,
        package_manager: PackageManager {
            name: "dnf".to_string(),
            install_cmd: "dnf install -y".to_string(),
            remove_cmd: "dnf remove -y".to_string(),
            update_cmd: "dnf upgrade -y".to_string(),
            search_cmd: "dnf search".to_string(),
            list_installed_cmd: "dnf list installed".to_string(),
        },
        init_system: InitSystem::Systemd,
        base_packages: vec![
            "basesystem".to_string(),
            "kernel".to_string(),
            "NetworkManager".to_string(),
        ],
        config_paths: HashMap::new(),
        pre_migration_hooks: vec![],
        post_migration_hooks: vec![
            "systemctl enable NetworkManager".to_string(),
            "grub2-mkconfig -o /boot/grub2/grub.cfg".to_string(),
        ],
        package_mappings: HashMap::new(),
    }
}

fn create_opensuse_profile() -> DistroProfile {
    use crate::config::{DistroFamily, InitSystem, PackageManager};
    use std::collections::HashMap;

    DistroProfile {
        name: "openSUSE".to_string(),
        version: "Tumbleweed".to_string(),
        family: DistroFamily::Suse,
        package_manager: PackageManager {
            name: "zypper".to_string(),
            install_cmd: "zypper install -y".to_string(),
            remove_cmd: "zypper remove -y".to_string(),
            update_cmd: "zypper update -y".to_string(),
            search_cmd: "zypper search".to_string(),
            list_installed_cmd: "zypper packages --installed-only".to_string(),
        },
        init_system: InitSystem::Systemd,
        base_packages: vec![
            "patterns-base-base".to_string(),
            "kernel-default".to_string(),
            "NetworkManager".to_string(),
        ],
        config_paths: HashMap::new(),
        pre_migration_hooks: vec![],
        post_migration_hooks: vec![
            "systemctl enable NetworkManager".to_string(),
            "grub2-mkconfig -o /boot/grub2/grub.cfg".to_string(),
        ],
        package_mappings: HashMap::new(),
    }
}

fn create_kali_profile() -> DistroProfile {
    use crate::config::{DistroFamily, InitSystem, PackageManager};
    use std::collections::HashMap;

    DistroProfile {
        name: "Kali Linux".to_string(),
        version: "2024.1".to_string(),
        family: DistroFamily::Debian,
        package_manager: PackageManager {
            name: "apt".to_string(),
            install_cmd: "apt install -y".to_string(),
            remove_cmd: "apt remove -y".to_string(),
            update_cmd: "apt update && apt upgrade -y".to_string(),
            search_cmd: "apt search".to_string(),
            list_installed_cmd: "dpkg -l".to_string(),
        },
        init_system: InitSystem::Systemd,
        base_packages: vec![
            "kali-linux-core".to_string(),
            "kali-linux-default".to_string(),
            "kali-tools-top10".to_string(),
            "linux-image-amd64".to_string(),
            "network-manager".to_string(),
            "nmap".to_string(),
            "metasploit-framework".to_string(),
            "burpsuite".to_string(),
            "wireshark".to_string(),
            "aircrack-ng".to_string(),
            "john".to_string(),
            "hashcat".to_string(),
            "sqlmap".to_string(),
        ],
        config_paths: HashMap::new(),
        pre_migration_hooks: vec![
            "wget -q -O - https://archive.kali.org/archive-key.asc | apt-key add -".to_string(),
            "echo 'deb http://http.kali.org/kali kali-rolling main contrib non-free non-free-firmware' > /etc/apt/sources.list".to_string(),
            "apt update".to_string(),
        ],
        post_migration_hooks: vec![
            "systemctl enable NetworkManager".to_string(),
            "systemctl enable lightdm".to_string(),
            "update-grub".to_string(),
            "update-initramfs -u".to_string(),
        ],
        package_mappings: HashMap::new(),
    }
}

fn create_hyprland_profile() -> DistroProfile {
    use crate::config::{DistroFamily, InitSystem, PackageManager};
    use std::collections::HashMap;

    DistroProfile {
        name: "Hyprland".to_string(),
        version: "rolling".to_string(),
        family: DistroFamily::Arch,
        package_manager: PackageManager {
            name: "pacman".to_string(),
            install_cmd: "pacman -S --noconfirm".to_string(),
            remove_cmd: "pacman -R --noconfirm".to_string(),
            update_cmd: "pacman -Syu --noconfirm".to_string(),
            search_cmd: "pacman -Ss".to_string(),
            list_installed_cmd: "pacman -Q".to_string(),
        },
        init_system: InitSystem::Systemd,
        base_packages: vec![
            "base".to_string(),
            "linux".to_string(),
            "linux-firmware".to_string(),
            "base-devel".to_string(),
            "networkmanager".to_string(),
            "hyprland".to_string(),
            "hyprpaper".to_string(),
            "hyprlock".to_string(),
            "waybar".to_string(),
            "rofi-wayland".to_string(),
            "kitty".to_string(),
            "dunst".to_string(),
            "pipewire".to_string(),
            "pipewire-pulse".to_string(),
            "wireplumber".to_string(),
            "ttf-jetbrains-mono-nerd".to_string(),
            "papirus-icon-theme".to_string(),
        ],
        config_paths: HashMap::new(),
        pre_migration_hooks: vec![
            "pacman-key --init".to_string(),
            "pacman-key --populate archlinux".to_string(),
        ],
        post_migration_hooks: vec![
            "systemctl enable NetworkManager".to_string(),
            "systemctl --user enable pipewire".to_string(),
            "systemctl --user enable wireplumber".to_string(),
            "mkinitcpio -P".to_string(),
            "grub-mkconfig -o /boot/grub/grub.cfg".to_string(),
            "mkdir -p ~/.config/hypr".to_string(),
            "mkdir -p ~/.config/waybar".to_string(),
        ],
        package_mappings: HashMap::new(),
    }
}

fn create_garuda_dragonized_profile() -> DistroProfile {
    use crate::config::{DistroFamily, InitSystem, PackageManager};
    use std::collections::HashMap;

    DistroProfile {
        name: "Garuda Dragonized".to_string(),
        version: "rolling".to_string(),
        family: DistroFamily::Arch,
        package_manager: PackageManager {
            name: "pacman".to_string(),
            install_cmd: "pacman -S --noconfirm".to_string(),
            remove_cmd: "pacman -R --noconfirm".to_string(),
            update_cmd: "pacman -Syu --noconfirm".to_string(),
            search_cmd: "pacman -Ss".to_string(),
            list_installed_cmd: "pacman -Q".to_string(),
        },
        init_system: InitSystem::Systemd,
        base_packages: vec![
            "base".to_string(),
            "linux-zen".to_string(),
            "linux-zen-headers".to_string(),
            "linux-firmware".to_string(),
            "base-devel".to_string(),
            "networkmanager".to_string(),
            "plasma-meta".to_string(),
            "sddm".to_string(),
            "gamemode".to_string(),
            "mangohud".to_string(),
            "steam".to_string(),
            "lutris".to_string(),
            "wine-staging".to_string(),
            "timeshift".to_string(),
            "pipewire".to_string(),
            "pipewire-pulse".to_string(),
            "wireplumber".to_string(),
        ],
        config_paths: HashMap::new(),
        pre_migration_hooks: vec![
            "pacman-key --init".to_string(),
            "pacman-key --populate archlinux".to_string(),
            "pacman-key --recv-key 3056513887B78AEB --keyserver keyserver.ubuntu.com".to_string(),
            "pacman-key --lsign-key 3056513887B78AEB".to_string(),
        ],
        post_migration_hooks: vec![
            "systemctl enable NetworkManager".to_string(),
            "systemctl enable sddm".to_string(),
            "systemctl enable fstrim.timer".to_string(),
            "systemctl --user enable pipewire".to_string(),
            "systemctl --user enable wireplumber".to_string(),
            "mkinitcpio -P".to_string(),
            "grub-mkconfig -o /boot/grub/grub.cfg".to_string(),
            "echo 'vm.swappiness=10' >> /etc/sysctl.d/99-swappiness.conf".to_string(),
        ],
        package_mappings: HashMap::new(),
    }
}

fn create_nixos_profile() -> DistroProfile {
    use crate::config::{DistroFamily, InitSystem, PackageManager};
    use std::collections::HashMap;

    DistroProfile {
        name: "NixOS".to_string(),
        version: "23.11".to_string(),
        family: DistroFamily::Nix,
        package_manager: PackageManager {
            name: "nix".to_string(),
            install_cmd: "nix-env -iA nixos.".to_string(),
            remove_cmd: "nix-env -e".to_string(),
            update_cmd: "nixos-rebuild switch --upgrade".to_string(),
            search_cmd: "nix search nixpkgs".to_string(),
            list_installed_cmd: "nix-env -q".to_string(),
        },
        init_system: InitSystem::Systemd,
        base_packages: vec![
            "gnome.gnome-shell".to_string(),
            "gnome.nautilus".to_string(),
            "gnome.gnome-terminal".to_string(),
            "firefox".to_string(),
            "git".to_string(),
            "vim".to_string(),
            "htop".to_string(),
        ],
        config_paths: HashMap::new(),
        pre_migration_hooks: vec![
            "curl -L https://nixos.org/nix/install | sh".to_string(),
            "nix-channel --add https://nixos.org/channels/nixos-23.11 nixos".to_string(),
            "nix-channel --update".to_string(),
        ],
        post_migration_hooks: vec![
            "systemctl enable NetworkManager".to_string(),
            "systemctl enable gdm".to_string(),
            "nixos-rebuild switch".to_string(),
            "nix-collect-garbage -d".to_string(),
        ],
        package_mappings: HashMap::new(),
    }
}

fn create_pop_cosmic_profile() -> DistroProfile {
    use crate::config::{DistroFamily, InitSystem, PackageManager};
    use std::collections::HashMap;

    DistroProfile {
        name: "Pop!_OS COSMIC".to_string(),
        version: "24.04".to_string(),
        family: DistroFamily::Debian,
        package_manager: PackageManager {
            name: "apt".to_string(),
            install_cmd: "apt install -y".to_string(),
            remove_cmd: "apt remove -y".to_string(),
            update_cmd: "apt update && apt upgrade -y".to_string(),
            search_cmd: "apt search".to_string(),
            list_installed_cmd: "dpkg -l".to_string(),
        },
        init_system: InitSystem::Systemd,
        base_packages: vec![
            "pop-desktop".to_string(),
            "linux-generic".to_string(),
            "network-manager".to_string(),
            "cosmic-session".to_string(),
            "cosmic-comp".to_string(),
            "cosmic-panel".to_string(),
            "cosmic-launcher".to_string(),
            "cosmic-settings".to_string(),
            "system76-power".to_string(),
            "system76-scheduler".to_string(),
            "pipewire".to_string(),
            "pipewire-pulse".to_string(),
        ],
        config_paths: HashMap::new(),
        pre_migration_hooks: vec![
            "add-apt-repository -y ppa:system76/pop".to_string(),
            "add-apt-repository -y ppa:system76-dev/stable".to_string(),
            "apt update".to_string(),
        ],
        post_migration_hooks: vec![
            "systemctl enable NetworkManager".to_string(),
            "systemctl enable system76-power".to_string(),
            "systemctl enable system76-scheduler".to_string(),
            "systemctl --user enable pipewire".to_string(),
            "update-grub".to_string(),
            "update-initramfs -u".to_string(),
        ],
        package_mappings: HashMap::new(),
    }
}
