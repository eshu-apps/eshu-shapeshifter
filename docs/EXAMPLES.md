# Usage Examples

## Example 1: Ubuntu to Arch Linux

### Scenario
You're running Ubuntu 22.04 and want to try Arch Linux without losing your setup.

### Steps

```bash
# 1. Scan your current Ubuntu system
sudo eshu-shapeshifter scan
```

**Output:**
```
🔍 Scanning system...

═══════════════════════════════════════
           SYSTEM INFORMATION          
═══════════════════════════════════════

📦 Distribution:
  Name:         Ubuntu
  Version:      22.04
  Family:       Debian

🖥️  System:
  Kernel:       5.15.0-91-generic
  Architecture: x86_64
  Filesystem:   ext4
  Bootloader:   GRUB

📚 Packages:
  Installed:    1847 packages

⚙️  Services:
  Total:        156
  Enabled:      42
  Running:      38

👥 Users:
  root (UID: 0, Home: /root)
  john (UID: 1000, Home: /home/john)

✅ System scan complete!
```

```bash
# 2. Validate migration to Arch
sudo eshu-shapeshifter validate arch
```

**Output:**
```
🔍 Validating migration...

  ⚠️  Non-btrfs filesystem (ext4) - snapshots will use rsync (slower)
  ✓ Architecture compatible (x86_64)
  ✓ Bootloader detected (GRUB)

✅ Migration is possible!
From: Ubuntu 22.04
To:   Arch Linux rolling
```

```bash
# 3. Perform the transformation
sudo eshu-shapeshifter shapeshift arch
```

**Output:**
```
🔮 Eshu Shapeshifter - System Transformation
═══════════════════════════════════════════════

Step 1: Scanning current system...
  Current: Ubuntu 22.04 (Debian)

Step 2: Loading target distribution profile...
  Target: Arch Linux rolling (Arch)

Step 3: Validating migration...
  ✓ Migration is possible

Step 4: Confirm with user...
⚠️  WARNING: This will transform your system!
From: Ubuntu 22.04
To:   Arch Linux rolling

A snapshot will be created for rollback.
? Do you want to continue? (y/N) y

Step 5: Creating system snapshot...
  Using rsync backup (this may take a while)
  ✓ Snapshot created: snapshot_1234567890

Step 6: Translating packages...
  Translated: 1623 packages
  Untranslated: 47 packages
  Skipped: 177 packages

Step 7: Preparing configuration translations...
  12 configuration operations prepared

Step 8: Preserving user data...
  ✓ User data backed up

Step 9: Executing migration...
  [####################################] 100/100
  Migration complete

═══════════════════════════════════════════════
✅ Transformation complete!

Next steps:
  1. Review the changes
  2. Reboot your system
  3. If issues occur, use: eshu-shapeshifter revert snapshot_1234567890

⚠️  IMPORTANT: Reboot required for changes to take effect!
```

```bash
# 4. Reboot
sudo reboot
```

### After Reboot

Your system is now running Arch Linux with:
- All your files intact
- Your user account preserved
- Most applications reinstalled
- Configurations translated

### If Something Goes Wrong

```bash
# Revert to Ubuntu
sudo eshu-shapeshifter revert
sudo reboot
```

---

## Example 2: Arch to Fedora

### Scenario
You're on Arch Linux and want to try Fedora's newer packages and SELinux.

```bash
# Quick migration
sudo eshu-shapeshifter scan
sudo eshu-shapeshifter list
sudo eshu-shapeshifter shapeshift fedora

# After testing, revert if needed
sudo eshu-shapeshifter revert
```

---

## Example 3: Testing Multiple Distributions

### Scenario
You want to try several distributions to find your favorite.

```bash
# Start with Ubuntu
# ... your current system ...

# Try Arch
sudo eshu-shapeshifter shapeshift arch
sudo reboot
# ... test for a few days ...

# Try Fedora
sudo eshu-shapeshifter shapeshift fedora
sudo reboot
# ... test for a few days ...

# Try openSUSE
sudo eshu-shapeshifter shapeshift opensuse
sudo reboot
# ... test for a few days ...

# Go back to your favorite
sudo eshu-shapeshifter snapshots  # List all snapshots
sudo eshu-shapeshifter revert snapshot_arch_123456
sudo reboot
```

---

## Example 4: Server Migration

### Scenario
Migrating a development server from Ubuntu to Debian.

```bash
# 1. Backup everything first!
sudo tar -czf /backup/server-backup.tar.gz /home /var/www /etc

# 2. Scan and validate
sudo eshu-shapeshifter scan
sudo eshu-shapeshifter validate debian

# 3. Schedule maintenance window
# 4. Perform migration
sudo eshu-shapeshifter shapeshift debian

# 5. Reboot
sudo reboot

# 6. Verify services
sudo systemctl status nginx
sudo systemctl status postgresql
sudo systemctl status redis

# 7. Test applications
curl http://localhost
# ... run your test suite ...

# 8. If issues, revert
sudo eshu-shapeshifter revert
```

---

## Example 5: Custom Package Mappings

### Scenario
You have custom packages that need special mapping.

```bash
# Create a custom mapping script
cat > add_mappings.sh << 'EOF'
#!/bin/bash

# Add custom package mappings
sqlite3 /var/lib/eshu-shapeshifter/package_mappings.db << SQL
INSERT OR REPLACE INTO package_mappings 
(source_family, source_package, target_family, target_package, confidence)
VALUES 
('Debian', 'my-custom-app', 'Arch', 'my-custom-app-git', 1.0),
('Debian', 'company-tool', 'Arch', 'company-tool-bin', 1.0);
SQL

echo "Custom mappings added"
EOF

chmod +x add_mappings.sh
sudo ./add_mappings.sh

# Now migrate with custom mappings
sudo eshu-shapeshifter shapeshift arch
```

---

## Example 6: Minimal Migration

### Scenario
You want to migrate only essential packages, then manually install others.

```bash
# 1. Create a minimal package list
cat > minimal_packages.txt << EOF
vim
git
tmux
htop
curl
wget
EOF

# 2. Scan system
sudo eshu-shapeshifter scan

# 3. Edit the migration to only include minimal packages
# (This would require code modification or a future feature)

# 4. Migrate
sudo eshu-shapeshifter shapeshift arch

# 5. Manually install additional packages
sudo pacman -S firefox chromium vscode
```

---

## Example 7: Checking Migration Status

### Scenario
Monitor the migration process and history.

```bash
# Check current status
sudo eshu-shapeshifter status

# List all snapshots
sudo eshu-shapeshifter snapshots

# View transformation history
cat /var/lib/eshu-shapeshifter/history.json | jq

# Check logs
sudo tail -f /var/log/eshu-shapeshifter/migration.log
```

**Output:**
```json
[
  {
    "from_distro": "Ubuntu",
    "to_distro": "Arch Linux",
    "timestamp": "2024-01-15T10:30:00Z",
    "snapshot_id": "snapshot_1234567890"
  },
  {
    "from_distro": "Arch Linux",
    "to_distro": "Fedora",
    "timestamp": "2024-01-20T14:45:00Z",
    "snapshot_id": "snapshot_1234567999"
  }
]
```

---

## Example 8: Automated Testing in VM

### Scenario
Test migrations automatically in a VM before running on real hardware.

```bash
#!/bin/bash
# test_migration.sh

# Create VM
virt-install \
  --name test-migration \
  --ram 4096 \
  --disk path=/var/lib/libvirt/images/test.qcow2,size=50 \
  --vcpus 2 \
  --os-type linux \
  --os-variant ubuntu22.04 \
  --network bridge=virbr0 \
  --graphics none \
  --console pty,target_type=serial \
  --location 'http://archive.ubuntu.com/ubuntu/dists/jammy/main/installer-amd64/' \
  --extra-args 'console=ttyS0,115200n8 serial'

# Wait for installation
sleep 600

# SSH into VM
ssh root@test-vm << 'ENDSSH'
  # Install eshu-shapeshifter
  curl -sSL https://raw.githubusercontent.com/yourusername/eshu-shapeshifter/main/scripts/install.sh | bash
  
  # Run migration
  eshu-shapeshifter scan
  eshu-shapeshifter shapeshift arch
  
  # Reboot
  reboot
ENDSSH

# Wait for reboot
sleep 120

# Verify
ssh root@test-vm << 'ENDSSH'
  # Check if Arch
  cat /etc/os-release | grep "Arch Linux"
  
  # Check packages
  pacman -Q | wc -l
  
  # Check services
  systemctl list-units --state=running
ENDSSH

echo "Migration test complete!"
```

---

## Example 9: Rollback After Issues

### Scenario
You migrated but encountered issues and need to rollback.

```bash
# List available snapshots
sudo eshu-shapeshifter snapshots
```

**Output:**
```
📸 Available Snapshots:
═══════════════════════════════════════════════════════════

ID: snapshot_1234567890
  Date: 2024-01-15 10:30:00
  Distro: Ubuntu 22.04
  Description: Before migration to Arch Linux rolling
  Type: Rsync

ID: snapshot_1234567800
  Date: 2024-01-10 08:15:00
  Distro: Ubuntu 22.04
  Description: Before migration to Fedora 39
  Type: Rsync
```

```bash
# Revert to specific snapshot
sudo eshu-shapeshifter revert snapshot_1234567890

# Or interactive selection
sudo eshu-shapeshifter revert
# (Shows menu to select snapshot)

# Reboot
sudo reboot
```

---

## Example 10: Enterprise Deployment

### Scenario
Deploy standardized Arch Linux across multiple Ubuntu workstations.

```bash
#!/bin/bash
# deploy_arch.sh - Run on each workstation

set -e

# Configuration
SNAPSHOT_BACKUP="/backup/snapshots"
LOG_FILE="/var/log/migration-$(date +%Y%m%d-%H%M%S).log"

# Function to log
log() {
    echo "[$(date +'%Y-%m-%d %H:%M:%S')] $*" | tee -a "$LOG_FILE"
}

# Pre-flight checks
log "Starting pre-flight checks..."

# Check disk space
AVAILABLE=$(df / | tail -1 | awk '{print $4}')
if [ "$AVAILABLE" -lt 10485760 ]; then  # 10GB
    log "ERROR: Insufficient disk space"
    exit 1
fi

# Backup current state
log "Creating backup..."
sudo eshu-shapeshifter scan

# Validate migration
log "Validating migration..."
if ! sudo eshu-shapeshifter validate arch; then
    log "ERROR: Validation failed"
    exit 1
fi

# Perform migration
log "Starting migration..."
sudo eshu-shapeshifter shapeshift arch

# Copy snapshot to backup location
log "Backing up snapshot..."
SNAPSHOT_ID=$(ls -t /var/lib/eshu-shapeshifter/snapshots/*.json | head -1 | xargs basename -s .json)
cp -r "/var/lib/eshu-shapeshifter/snapshots/$SNAPSHOT_ID" "$SNAPSHOT_BACKUP/"

log "Migration complete. Reboot required."
log "Snapshot backed up to: $SNAPSHOT_BACKUP/$SNAPSHOT_ID"

# Schedule reboot
log "Scheduling reboot in 5 minutes..."
shutdown -r +5 "System will reboot for migration completion"
```

---

## Tips and Tricks

### Quick Status Check
```bash
alias eshu-status='sudo eshu-shapeshifter status'
alias eshu-snapshots='sudo eshu-shapeshifter snapshots'
```

### Pre-Migration Checklist
```bash
#!/bin/bash
# pre_migration_check.sh

echo "Pre-Migration Checklist"
echo "======================="
echo ""

# Disk space
echo "1. Disk Space:"
df -h / | tail -1

# Backup status
echo ""
echo "2. Recent Backups:"
ls -lh /backup/*.tar.gz 2>/dev/null | tail -3

# Important services
echo ""
echo "3. Critical Services:"
systemctl is-active nginx postgresql redis 2>/dev/null

# Network
echo ""
echo "4. Network:"
ping -c 1 8.8.8.8 > /dev/null && echo "✓ Internet OK" || echo "✗ No Internet"

# Filesystem
echo ""
echo "5. Filesystem:"
findmnt -n -o FSTYPE /

echo ""
echo "Ready to migrate? (y/N)"
```

### Post-Migration Verification
```bash
#!/bin/bash
# post_migration_verify.sh

echo "Post-Migration Verification"
echo "==========================="
echo ""

# Check OS
echo "1. Operating System:"
cat /etc/os-release | grep PRETTY_NAME

# Check packages
echo ""
echo "2. Package Count:"
if command -v pacman &> /dev/null; then
    pacman -Q | wc -l
elif command -v dpkg &> /dev/null; then
    dpkg -l | grep ^ii | wc -l
fi

# Check services
echo ""
echo "3. Running Services:"
systemctl list-units --type=service --state=running | wc -l

# Check users
echo ""
echo "4. User Accounts:"
cat /etc/passwd | grep -E ":[0-9]{4,}:" | cut -d: -f1

# Check network
echo ""
echo "5. Network:"
ip addr show | grep "inet " | grep -v 127.0.0.1

echo ""
echo "Verification complete!"
```
