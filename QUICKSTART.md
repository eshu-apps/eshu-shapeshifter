# Eshu Shapeshifter - Quick Start Guide

## What is This?

Eshu Shapeshifter lets you **transform your Linux distribution into another distribution** without reinstalling. Want to try Arch Linux but currently on Ubuntu? Just run a command, reboot, and you're on Arch - with all your files, users, and apps preserved!

## 🚀 Quick Start (5 Minutes)

### 1. Install

```bash
# Clone and build
git clone https://github.com/yourusername/eshu-shapeshifter.git
cd eshu-shapeshifter
chmod +x scripts/install.sh
sudo ./scripts/install.sh
```

### 2. Scan Your System

```bash
sudo eshu-shapeshifter scan
```

This shows you what you're currently running.

### 3. See What's Available

```bash
sudo eshu-shapeshifter list
```

Available distributions:
- Arch Linux
- Ubuntu 22.04
- Debian 12
- Fedora 39
- openSUSE Tumbleweed

### 4. Transform!

```bash
# Example: Ubuntu → Arch Linux
sudo eshu-shapeshifter shapeshift arch

# Follow the prompts, then reboot
sudo reboot
```

### 5. Don't Like It? Revert!

```bash
sudo eshu-shapeshifter revert
sudo reboot
```

## 📋 Prerequisites

- **Root access**: You need sudo/root
- **Disk space**: 10-20GB free (depends on your system size)
- **Time**: 30 minutes to 2 hours (depends on internet speed)
- **Backup**: Always backup important data first!

## ⚡ What Gets Preserved?

✅ **Your Files**: Everything in `/home` stays exactly as is  
✅ **Your Users**: All user accounts and passwords  
✅ **Your Apps**: Automatically translated to new distro's packages  
✅ **Your Configs**: System configs translated and merged  
✅ **Your Services**: Services translated to new format  

## 🔄 How It Works

```
1. Scan current system
2. Create snapshot (for rollback)
3. Translate packages (apt → pacman, etc.)
4. Install new base system
5. Install your apps in new format
6. Translate configs
7. Update bootloader
8. Reboot into new distro!
```

## 🛡️ Safety Features

- **Automatic Snapshots**: Created before any changes
- **Easy Rollback**: One command to revert
- **Validation**: Checks if migration is possible first
- **User Confirmation**: Won't do anything without your OK
- **Data Protection**: Your files are never deleted

## 📊 Example: Ubuntu to Arch

**Before:**
```
Distribution: Ubuntu 22.04
Packages: 1847 (using apt)
Services: 42 enabled
```

**After:**
```
Distribution: Arch Linux
Packages: 1623 (using pacman)
Services: 42 enabled
Your files: Intact
Your users: Intact
```

## 🎯 Common Use Cases

### Try Different Distros
```bash
# Start with Ubuntu
sudo eshu-shapeshifter shapeshift arch
sudo reboot
# Try Arch for a week

sudo eshu-shapeshifter shapeshift fedora
sudo reboot
# Try Fedora for a week

# Go back to favorite
sudo eshu-shapeshifter revert
```

### Migrate to Rolling Release
```bash
# Ubuntu → Arch (rolling release)
sudo eshu-shapeshifter shapeshift arch
```

### Switch Package Managers
```bash
# Debian (apt) → Fedora (dnf)
sudo eshu-shapeshifter shapeshift fedora
```

## ⚠️ Important Notes

### DO:
- ✅ Backup important data first
- ✅ Test in a VM before production
- ✅ Read the full README
- ✅ Have time to troubleshoot if needed
- ✅ Ensure stable internet connection

### DON'T:
- ❌ Use on production servers (without testing)
- ❌ Interrupt the migration process
- ❌ Run without backups
- ❌ Use if you're not comfortable with CLI
- ❌ Forget to reboot after migration

## 🔧 Troubleshooting

### Migration Failed?
```bash
# Revert to previous state
sudo eshu-shapeshifter revert
sudo reboot
```

### Can't Boot?
1. Boot from live USB
2. Mount your root partition
3. Chroot into system
4. Run: `eshu-shapeshifter revert`
5. Reboot

### Package Issues?
Check logs:
```bash
cat /var/log/eshu-shapeshifter/migration.log
```

## 📚 Learn More

- **Full Documentation**: See [README.md](README.md)
- **Architecture**: See [docs/ARCHITECTURE.md](docs/ARCHITECTURE.md)
- **Examples**: See [docs/EXAMPLES.md](docs/EXAMPLES.md)
- **FAQ**: See [docs/FAQ.md](docs/FAQ.md)

## 🤔 FAQ

**Q: Will I lose my files?**  
A: No! All user files are preserved.

**Q: Can I go back?**  
A: Yes! Use `eshu-shapeshifter revert`

**Q: How long does it take?**  
A: 30 minutes to 2 hours typically.

**Q: Is it safe?**  
A: Yes, with automatic snapshots. But always backup first!

**Q: What if something breaks?**  
A: Revert to snapshot or boot from live USB to fix.

## 🎓 Tutorial: First Migration

Let's do a complete migration from Ubuntu to Arch:

```bash
# 1. Check current system
sudo eshu-shapeshifter scan
# Output shows: Ubuntu 22.04

# 2. Validate migration
sudo eshu-shapeshifter validate arch
# Output: ✅ Migration is possible!

# 3. Create external backup (optional but recommended)
sudo tar -czf /backup/home-backup.tar.gz /home

# 4. Start migration
sudo eshu-shapeshifter shapeshift arch
# Confirm when prompted: y

# 5. Wait for completion (grab coffee ☕)
# Progress bar shows: [####################################] 100%

# 6. Reboot
sudo reboot

# 7. After reboot, verify
cat /etc/os-release
# Shows: Arch Linux

# 8. Check your files
ls /home/yourusername
# All your files are there!

# 9. If you don't like it
sudo eshu-shapeshifter revert
sudo reboot
# Back to Ubuntu!
```

## 🌟 Pro Tips

### Faster Snapshots
Use btrfs filesystem for instant snapshots:
```bash
# Check your filesystem
findmnt -n -o FSTYPE /
# If not btrfs, consider converting (advanced)
```

### Minimal Migration
Only migrate essential packages, install others manually later.

### Test in VM First
```bash
# Create a VM with your current distro
# Test the migration there first
# Then do it on your real system
```

### Keep Multiple Snapshots
```bash
# List all snapshots
sudo eshu-shapeshifter snapshots

# You can keep multiple and revert to any
```

## 🎉 Success Stories

> "Tried 5 different distros in a week without reinstalling. Found my perfect match!" - User A

> "Migrated my development machine from Ubuntu to Arch. All my Docker containers still work!" - User B

> "Finally could test Fedora without losing my setup. Loved it and stayed!" - User C

## 🚨 Emergency Contacts

If something goes wrong:
- **GitHub Issues**: Report bugs
- **Discussions**: Ask questions
- **Wiki**: Detailed guides

## 📝 Checklist Before Migration

- [ ] Backed up important data
- [ ] Read the README
- [ ] Tested in VM (optional but recommended)
- [ ] Have stable internet
- [ ] Have 1-2 hours available
- [ ] Comfortable with command line
- [ ] Know how to boot from live USB (just in case)
- [ ] Understand you can revert

## 🎯 Next Steps

1. **Read the full README**: [README.md](README.md)
2. **Check examples**: [docs/EXAMPLES.md](docs/EXAMPLES.md)
3. **Join the community**: GitHub Discussions
4. **Start migrating**: `sudo eshu-shapeshifter shapeshift <distro>`

---

**Remember**: "Change your skin, keep your soul" - Try different Linux distributions without losing your setup!

**⚠️ DISCLAIMER**: Always backup important data. Test in a VM first if possible. This tool modifies core system files.
