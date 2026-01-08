# Eshu Shapeshifter Architecture

## Overview

Eshu Shapeshifter is designed as a modular system that safely transforms Linux distributions while preserving user data and configurations.

## Core Components

### 1. Scanner Module (`scanner.rs`)

**Purpose**: Analyze the current system state

**Functions**:
- Detect current distribution and version
- List all installed packages
- Enumerate running services
- Catalog user accounts
- Identify filesystem type
- Detect bootloader

**Output**: `SystemState` struct containing complete system snapshot

### 2. Distro Detection (`distro.rs`)

**Purpose**: Identify Linux distributions and their characteristics

**Methods**:
- Parse `/etc/os-release`
- Fallback to `/etc/lsb-release`
- Check distro-specific files
- Detect package manager
- Identify init system

**Supported Families**:
- Debian (Ubuntu, Debian, Mint, Pop!_OS)
- Arch (Arch, Manjaro, EndeavourOS)
- RedHat (Fedora, RHEL, CentOS, Rocky)
- Suse (openSUSE, SLES)
- Others (Gentoo, Alpine, Void, NixOS)

### 3. Package Translator (`package.rs`)

**Purpose**: Map packages between different distributions

**Database**: SQLite with package mappings

**Features**:
- Direct package mappings (e.g., `python3-pip` → `python-pip`)
- Fuzzy matching for similar packages
- Confidence scoring
- System package filtering (prevents breaking core system)

**Example Mappings**:
```
Debian → Arch:
  python3 → python
  libssl-dev → openssl
  gcc + g++ → gcc

Arch → Debian:
  python → python3
  python-pip → python3-pip
```

### 4. Configuration Translator (`translation.rs`)

**Purpose**: Translate system configurations between distros

**Translations**:
- Network configs (interfaces ↔ systemd-networkd)
- Service definitions
- User accounts (merge with existing)
- Application configs

**Transform Types**:
- **Copy**: Direct file copy
- **Merge**: Combine with existing config
- **Transform**: Apply conversion function
- **Skip**: Don't migrate (distro-specific)

### 5. Snapshot Manager (`snapshot.rs`)

**Purpose**: Create and manage system snapshots for rollback

**Methods**:
1. **Btrfs Snapshots** (Preferred)
   - Instant copy-on-write
   - Space-efficient
   - Fast rollback

2. **LVM Snapshots**
   - Block-level snapshots
   - Good for non-btrfs systems
   - Requires LVM setup

3. **Rsync Backup** (Fallback)
   - Full file copy
   - Works on any filesystem
   - Slower but reliable

**Snapshot Metadata**:
```rust
struct Snapshot {
    id: String,
    timestamp: i64,
    distro_name: String,
    distro_version: String,
    description: String,
    snapshot_type: SnapshotType,
    size_bytes: u64,
    path: PathBuf,
}
```

### 6. Migration Engine (`migration.rs`)

**Purpose**: Orchestrate the entire transformation process

**Migration Phases**:

1. **Scan** (5%)
   - Analyze current system
   - Collect package list
   - Identify services

2. **Validate** (5%)
   - Check compatibility
   - Verify filesystem support
   - Confirm architecture match

3. **Snapshot** (10%)
   - Create system snapshot
   - Save metadata
   - Verify snapshot integrity

4. **Translate** (20%)
   - Map packages
   - Prepare config translations
   - Build migration plan

5. **Execute** (50%)
   - Setup target package manager
   - Install base packages
   - Install translated packages
   - Apply config translations
   - Run post-migration hooks

6. **Finalize** (10%)
   - Update bootloader
   - Record transformation
   - Cleanup temporary files

### 7. Repository Manager (`repository.rs`)

**Purpose**: Manage distribution profiles

**Features**:
- Curated distro profiles
- Remote profile downloading
- Local caching
- Profile validation

**Profile Structure**:
```toml
name = "Arch Linux"
version = "rolling"
family = "Arch"

[package_manager]
name = "pacman"
install_cmd = "pacman -S --noconfirm"
# ...

base_packages = ["base", "linux", ...]
pre_migration_hooks = [...]
post_migration_hooks = [...]
```

## Data Flow

```
┌─────────────┐
│  User Input │
└──────┬──────┘
       │
       ▼
┌─────────────────────────────────────────┐
│         CLI Parser (cli.rs)             │
└──────┬──────────────────────────────────┘
       │
       ▼
┌─────────────────────────────────────────┐
│      Main Orchestrator (main.rs)        │
└──────┬──────────────────────────────────┘
       │
       ├──────────────────────────────────┐
       │                                  │
       ▼                                  ▼
┌─────────────┐                    ┌─────────────┐
│   Scanner   │                    │ Repository  │
│             │                    │             │
│ • Detect OS │                    │ • Get       │
│ • Packages  │                    │   Profile   │
│ • Services  │                    │             │
└──────┬──────┘                    └──────┬──────┘
       │                                  │
       └──────────────┬───────────────────┘
                      │
                      ▼
              ┌───────────────┐
              │  Validator    │
              │               │
              │ • Check       │
              │   Compat      │
              └───────┬───────┘
                      │
                      ▼
              ┌───────────────┐
              │  Snapshot     │
              │  Manager      │
              │               │
              │ • Create      │
              │   Backup      │
              └───────┬───────┘
                      │
                      ▼
              ┌───────────────┐
              │  Translator   │
              │               │
              │ • Packages    │
              │ • Configs     │
              └───────┬───────┘
                      │
                      ▼
              ┌───────────────┐
              │  Migration    │
              │  Engine       │
              │               │
              │ • Execute     │
              │   Changes     │
              └───────┬───────┘
                      │
                      ▼
              ┌───────────────┐
              │   Success!    │
              └───────────────┘
```

## Safety Mechanisms

### 1. Pre-flight Validation
- Check filesystem compatibility
- Verify sufficient disk space
- Confirm architecture match
- Validate target distro exists

### 2. Atomic Operations
- All changes are logged
- Rollback points at each phase
- Transaction-like behavior

### 3. Snapshot Protection
- Automatic snapshot before changes
- Metadata preservation
- Easy rollback command

### 4. User Confirmation
- Explicit confirmation required
- Clear warning messages
- Summary of changes

### 5. Error Handling
- Graceful failure handling
- Detailed error messages
- Automatic cleanup on failure

## File System Layout

```
/etc/eshu-shapeshifter/
├── config.toml              # Main configuration

/var/lib/eshu-shapeshifter/
├── current_state.json       # Current system state
├── history.json             # Transformation history
├── package_mappings.db      # SQLite package database
├── snapshots/               # Snapshot storage
│   ├── snapshot_123456/
│   │   └── ...
│   └── snapshot_123456.json # Snapshot metadata
├── config_backup/           # Config backups
└── user_backup/             # User data backups

/var/cache/eshu-shapeshifter/
└── profiles/                # Cached distro profiles
    ├── arch.toml
    ├── ubuntu.toml
    └── ...

/var/log/eshu-shapeshifter/
└── migration.log            # Migration logs
```

## Extension Points

### Custom Package Mappings
```rust
translator.add_mapping(
    &DistroFamily::Debian,
    "my-package",
    &DistroFamily::Arch,
    "my-arch-package",
    1.0
);
```

### Custom Config Translators
```rust
impl ConfigTranslator {
    fn add_custom_rule(&mut self, rule: ConfigTranslation) {
        self.translations.insert(rule.name, rule);
    }
}
```

### Custom Hooks
Add to distro profile:
```toml
pre_migration_hooks = [
    "echo 'Starting migration'",
    "/path/to/custom/script.sh"
]
```

## Performance Considerations

### Snapshot Speed
- **Btrfs**: Instant (copy-on-write)
- **LVM**: Seconds (block-level)
- **Rsync**: Minutes to hours (depends on data size)

### Package Installation
- Batch installations (50 packages at a time)
- Parallel downloads where supported
- Progress indicators

### Memory Usage
- Streaming file operations
- Lazy loading of package lists
- Efficient data structures

## Security Considerations

### Root Privileges
- Required for system modifications
- Checked at startup
- Clear warning if not root

### Package Verification
- Use official repositories
- Verify package signatures
- Check checksums

### Snapshot Integrity
- Metadata checksums
- Verification before rollback
- Protected storage

## Future Enhancements

1. **Parallel Operations**: Speed up package installation
2. **Incremental Snapshots**: Save space with differential backups
3. **Cloud Backup**: Sync snapshots to cloud storage
4. **GUI Interface**: Graphical user interface
5. **Container Testing**: Test migrations in containers first
6. **Custom Profiles**: User-defined distribution profiles
7. **Partial Migrations**: Migrate only packages or configs
8. **Multi-boot Support**: Manage multiple distros on one system
