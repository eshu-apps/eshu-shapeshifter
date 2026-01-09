# 🔮 Eshu Shapeshifter

**Try Any Linux Distro. Keep Everything.**

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Rust](https://img.shields.io/badge/rust-1.70%2B-orange.svg)](https://www.rust-lang.org/)
[![Status](https://img.shields.io/badge/status-beta-blue.svg)](https://github.com/eshu-apps/eshu-shapeshifter)
[![Website](https://img.shields.io/badge/🌐-eshu--apps.com-blue)](https://eshu-apps.com)

**[💎 Get Subscription - $5.99/mo](https://gumroad.com/l/eshu-shapeshifter)** | **[📦 Get Shift Pack - $3.99](https://gumroad.com/l/eshu-shapeshifter)** | **[💝 Donate](https://gumroad.com/l/eshu-donate)**

> 🎉 **2 FREE shapeshifts to try it out!**

Transform your Linux distribution without reinstalling - migrate between distributions seamlessly while preserving your data, applications, and configurations.

Named after Èṣù (pronounced "eh-SHOO"), the Yoruba orisha of crossroads and transformation, this tool allows you to shapeshift your Linux system between different distributions with the ability to revert back at any time.

**⚠️ STATUS: Beta - Coming Q2 2026**

Currently in active development. Core features complete, undergoing extensive testing. [Join the waitlist](https://eshuapps.com) for early access.

## ⚡ Features

- 🔄 **Seamless Migration**: Transform between major Linux distributions
- 📸 **Automatic Snapshots**: Create system snapshots before migration using btrfs, LVM, or rsync
- 🔙 **Easy Rollback**: Revert to your original system anytime with a single command
- 📦 **Intelligent Package Translation**: Automatically map packages between different package managers
- ⚙️ **Configuration Preservation**: Translate and preserve system configurations
- 👥 **User Data Protection**: Preserve all user home directories and data
- 🎯 **Curated Profiles**: Pre-tested distribution profiles for reliable migrations
- 🔍 **System Validation**: Check migration compatibility before transforming
- 🎨 **Gorgeous Setups**: Beautiful, pre-configured desktop environments

## 💰 Pricing

### 🆓 Free Trial
- **2 FREE shapeshifts** to test the tool
- No credit card required
- Full feature access

### Choose Your Plan

**💎 Unlimited Monthly - $5.99/month**
- ♾️ Unlimited shapeshifts
- 🔄 Try as many distros as you want
- ❌ Cancel anytime
- 💪 Perfect for distro hoppers & developers

**📦 Shift Packs - $3.99 per pack**
- 📦 10 shapeshifts per purchase
- 💰 One-time payment
- ⏰ Never expires
- 📚 Stack multiple packs
- 🎯 Best for occasional users

**[Get Eshu Shapeshifter →](https://gumroad.com/l/eshu-shapeshifter)**

### Activating Your License

After purchasing, activate your license:

```bash
sudo eshu-shapeshifter activate YOUR_LICENSE_KEY
```

Check your license status anytime:

```bash
sudo eshu-shapeshifter license
```

## 🚀 Installation

### From Source (Recommended)

```bash
# Clone the repository
git clone https://github.com/eshu-apps/eshu-shapeshifter.git
cd eshu-shapeshifter

# Build with Cargo
cargo build --release

# Install system-wide
sudo cp target/release/eshu-shapeshifter /usr/local/bin/

# Or install with cargo
cargo install --path .
```

### Prerequisites

- Rust 1.70+ (for building)
- Root/sudo access (for system modifications)
- One of the following for optimal snapshots:
  - Btrfs filesystem (recommended)
  - LVM setup
  - Sufficient disk space for rsync backups

## 📖 Usage

### Basic Commands

```bash
# Scan your current system
sudo eshu-shapeshifter scan

# List available distributions
sudo eshu-shapeshifter list

# Check your license status
sudo eshu-shapeshifter license

# Activate a license key (after purchase)
sudo eshu-shapeshifter activate YOUR_LICENSE_KEY

# Validate if migration is possible
sudo eshu-shapeshifter validate arch

# Transform to a different distribution
sudo eshu-shapeshifter shapeshift arch

# List all snapshots
sudo eshu-shapeshifter snapshots

# Revert to a previous snapshot
sudo eshu-shapeshifter revert

# Check current status
sudo eshu-shapeshifter status
```

### Example Workflow

```bash
# 1. Scan your current Ubuntu system
sudo eshu-shapeshifter scan

# 2. See what's available
sudo eshu-shapeshifter list

# 3. Validate migration to Arch Linux
sudo eshu-shapeshifter validate arch

# 4. Transform to Arch Linux
sudo eshu-shapeshifter shapeshift arch

# 5. Reboot and test
sudo reboot

# 6. If you want to go back
sudo eshu-shapeshifter revert
sudo reboot
```

## 🎯 Supported Distributions

### 🌟 Featured Distributions

| Distribution | Family | Desktop | Focus | Status |
|-------------|--------|---------|-------|--------|
| **🔐 Kali Linux** | Debian | XFCE | Security/Pentesting | ✅ Stable |
| **🌊 Hyprland** | Arch | Hyprland (Wayland) | Aesthetics | ✅ Stable |
| **🐉 Garuda Dragonized** | Arch | KDE Plasma | Gaming/Performance | ✅ Stable |
| **❄️ NixOS** | Nix | GNOME | Reproducibility | ✅ Stable |
| **🚀 Pop!_OS COSMIC** | Ubuntu | COSMIC (Rust) | Productivity | ✅ Stable |

### 📦 Standard Distributions

| Distribution | Family | Package Manager | Status |
|-------------|--------|-----------------|--------|
| Arch Linux | Arch | pacman | ✅ Stable |
| Ubuntu 22.04+ | Debian | apt | ✅ Stable |
| Debian 12+ | Debian | apt | ✅ Stable |
| Fedora 39+ | RedHat | dnf | ✅ Stable |
| openSUSE Tumbleweed | Suse | zypper | ✅ Stable |

**Total Supported: 10 Distributions**

📚 **[See detailed distribution guide →](docs/NEW_DISTROS.md)**

## 🎨 Featured Transformations

### 🔐 Transform to Kali Linux
Perfect for security professionals and penetration testers!
```bash
sudo eshu-shapeshifter shapeshift kali
```
**Includes**: nmap, metasploit, burpsuite, wireshark, aircrack-ng, john, hashcat, and 100+ security tools

### 🌊 Transform to Hyprland
Experience the most beautiful Wayland compositor!
```bash
sudo eshu-shapeshifter shapeshift hyprland
```
**Features**: Smooth animations, blur effects, Waybar, Rofi, Kitty terminal, Arc Dark theme

### 🐉 Transform to Garuda Dragonized
Ultimate gaming and performance powerhouse!
```bash
sudo eshu-shapeshifter shapeshift garuda
```
**Includes**: Linux Zen kernel, Steam, Lutris, GameMode, MangoHUD, KDE Plasma Dragonized

### ❄️ Transform to NixOS
Declarative, reproducible system configuration!
```bash
sudo eshu-shapeshifter shapeshift nixos
```
**Features**: Declarative configs, atomic upgrades, easy rollbacks, GNOME 45

### 🚀 Transform to Pop!_OS COSMIC
Next-gen Rust-based desktop environment!
```bash
sudo eshu-shapeshifter shapeshift cosmic
```
**Features**: Auto-tiling, system76-scheduler, COSMIC desktop (Rust), gaming support

## 🏗️ Architecture

```
┌─────────────────────────────────────────────────────────────┐
│                    Eshu Shapeshifter                        │
├─────────────────────────────────────────────────────────────┤
│                                                             │
│  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐    │
│  │   Scanner    │  │  Translator  │  │   Snapshot   │    │
│  │              │  │              │  │              │    │
│  │ • Detect OS  │  │ • Packages   │  │ • Btrfs      │    │
│  │ • Packages   │  │ • Configs    │  │ • LVM        │    │
│  │ • Services   │  │ • Services   │  │ • Rsync      │    │
│  │ • Users      │  │ • Users      │  │              │    │
│  └──────────────┘  └──────────────┘  └──────────────┘    │
│                                                             │
│  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐    │
│  │  Migration   │  │  Repository  │  │   Package    │    │
│  │   Engine     │  │              │  │  Translator  │    │
│  │              │  │ • Profiles   │  │              │    │
│  │ • Orchestrate│  │ • Curated    │  │ • SQLite DB  │    │
│  │ • Execute    │  │ • Download   │  │ • Mappings   │    │
│  │ • Validate   │  │              │  │ • Fuzzy Match│    │
│  └──────────────┘  └──────────────┘  └──────────────┘    │
│                                                             │
└─────────────────────────────────────────────────────────────┘
```

## 🔧 How It Works

### 1. System Scanning
Eshu analyzes your current system:
- Distribution and version
- Installed packages
- Running services
- User accounts
- System configurations
- Filesystem type
- Bootloader

### 2. Package Translation
Using an embedded SQLite database, Eshu maps packages between distributions:
```
Ubuntu (apt)          →  Arch (pacman)
python3-pip           →  python-pip
libssl-dev            →  openssl
gcc + g++             →  gcc
```

### 3. Configuration Translation
System configurations are intelligently translated:
- Network configs (interfaces → systemd-networkd)
- Service definitions (systemd units)
- User accounts and permissions
- Application configs

### 4. Snapshot Creation
Before any changes, a complete snapshot is created:
- **Btrfs**: Instant copy-on-write snapshots
- **LVM**: Logical volume snapshots
- **Rsync**: Full backup of critical directories

### 5. Migration Execution
The transformation happens in phases:
1. Setup target package manager
2. Install base system packages
3. Install translated user packages
4. Apply configuration translations
5. Run post-migration hooks
6. Update bootloader

### 6. Rollback Capability
If anything goes wrong:
```bash
sudo eshu-shapeshifter revert
```
This restores your system to the pre-migration state.

## ⚙️ Configuration

Configuration file: `/etc/eshu-shapeshifter/config.toml`

```toml
version = "0.1.0"
data_dir = "/var/lib/eshu-shapeshifter"
snapshot_dir = "/var/lib/eshu-shapeshifter/snapshots"
repository_url = "https://raw.githubusercontent.com/eshu-shapeshifter/distro-profiles/main"
cache_dir = "/var/cache/eshu-shapeshifter"
```

## 🛡️ Safety Features

1. **Automatic Snapshots**: Every migration creates a snapshot
2. **Validation**: Pre-flight checks ensure migration is possible
3. **User Confirmation**: Explicit confirmation required before changes
4. **Incremental Operations**: Changes are applied in stages
5. **Rollback Support**: Easy reversion to previous state
6. **Data Preservation**: User data is always protected

## ⚠️ Important Notes

### What Gets Preserved
✅ User home directories  
✅ User accounts and passwords  
✅ Application data  
✅ System services (translated)  
✅ Network configurations (translated)  
✅ Custom configurations  

### What Changes
🔄 Package manager  
🔄 System packages  
🔄 Init system configurations  
🔄 Bootloader configuration  
🔄 Distribution-specific files  

### Limitations
- Requires root access
- Reboot required after migration
- Some packages may not have direct equivalents
- Custom kernel modules may need reinstallation
- Proprietary drivers may need reconfiguration

## 🐛 Troubleshooting

### Migration Failed
```bash
# Check the logs
journalctl -u eshu-shapeshifter

# Revert to snapshot
sudo eshu-shapeshifter revert

# Check system status
sudo eshu-shapeshifter status
```

### Boot Issues After Migration
1. Boot from live USB
2. Mount your root partition
3. Chroot into the system
4. Run: `eshu-shapeshifter revert`

### Package Installation Failures
Some packages may fail to install due to:
- Missing dependencies
- Repository issues
- Architecture incompatibilities

Check `/var/log/eshu-shapeshifter/migration.log` for details.

## 🤝 Contributing

Contributions are welcome! Areas where help is needed:

1. **Distribution Profiles**: Add support for more distributions
2. **Package Mappings**: Improve package translation database
3. **Configuration Translators**: Better config file translations
4. **Testing**: Test migrations on different hardware/setups
5. **Documentation**: Improve guides and examples

## 📋 Roadmap

- [x] Core migration engine
- [x] 10 distribution profiles
- [x] Snapshot system (Btrfs/LVM/Rsync)
- [x] Package translation database
- [ ] Custom ISO support
- [ ] GUI interface
- [ ] Partial migrations (packages only, configs only)
- [ ] Migration profiles (minimal, full, custom)
- [ ] Cloud backup integration
- [ ] Multi-boot support
- [ ] Container-based testing
- [ ] Web dashboard

## 🔒 Security Considerations

- Always run from a trusted source
- Review package mappings before migration
- Keep snapshots until system is stable
- Test in a VM first if possible
- Backup important data externally

## 📚 Documentation

- **[Quick Start Guide](QUICKSTART.md)** - Get started in 5 minutes
- **[New Distributions Guide](docs/NEW_DISTROS.md)** - Detailed info on all 10 distros
- **[Architecture Guide](docs/ARCHITECTURE.md)** - Technical deep dive
- **[Examples](docs/EXAMPLES.md)** - Real-world scenarios
- **[FAQ](docs/FAQ.md)** - Common questions

## 📜 License

MIT License - See LICENSE file for details

## 🙏 Acknowledgments

- Named after Eshu, the Yoruba deity of crossroads and transformation
- Inspired by the need for easier Linux distribution experimentation
- Built with Rust for safety and performance

## 📞 Support & Contact

- 🌐 **Website**: [eshu-apps.com](https://eshu-apps.com)
- 📧 **Support**: support@eshu-apps.com
- 🐛 **Bug Reports**: [GitHub Issues](https://github.com/eshu-apps/eshu-shapeshifter/issues)
- 💬 **Discussions**: [GitHub Discussions](https://github.com/eshu-apps/eshu-shapeshifter/discussions)
- 💝 **Donate**: [Support the Project](https://gumroad.com/l/eshu-donate)

---

**⚠️ DISCLAIMER**: This tool modifies core system files. While it includes safety features and snapshots, always backup important data before use. Test in a non-production environment first.

**"Change your skin, keep your soul"** - Try different Linux distributions without losing your setup! 🔮
