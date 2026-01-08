# Frequently Asked Questions

## General Questions

### What is Eshu Shapeshifter?

Eshu Shapeshifter is a tool that allows you to transform your Linux distribution into another distribution without reinstalling. It preserves your data, applications, and configurations while changing the underlying system.

### Why would I want to use this?

- **Try before you commit**: Test different distributions without reinstalling
- **Easy migration**: Move to a new distro while keeping your setup
- **Learning**: Understand how different distributions work
- **Flexibility**: Switch between distros based on your needs
- **Safety**: Revert back if you don't like the new distro

### Is this safe?

Eshu Shapeshifter includes multiple safety features:
- Automatic snapshots before any changes
- Validation checks before migration
- Easy rollback capability
- User data preservation
- Extensive testing on common configurations

However, it modifies core system files, so always backup important data first.

### What's the difference between this and a dual-boot?

- **Dual-boot**: Two separate installations, switch by rebooting
- **Eshu**: Transform your current system, keep all your data and configs

Eshu is more like "morphing" your system rather than installing a new one.

## Technical Questions

### How does package translation work?

Eshu maintains a database of package mappings between distributions. For example:
- Ubuntu's `python3-pip` → Arch's `python-pip`
- Debian's `libssl-dev` → Arch's `openssl`

If a direct mapping doesn't exist, it uses fuzzy matching to find similar packages.

### What happens to my files?

- **User files** (`/home`): Completely preserved
- **System configs** (`/etc`): Translated and merged
- **Applications**: Reinstalled using target distro's packages
- **Services**: Translated to target distro's format

### Can I customize the migration?

Yes! You can:
- Add custom package mappings
- Create custom distro profiles
- Add pre/post migration hooks
- Skip certain packages or configs

### What if a package doesn't exist in the target distro?

Eshu will:
1. Try to find a similar package
2. Log it as "untranslated"
3. Continue with the migration
4. You can manually install alternatives later

### How long does a migration take?

Depends on:
- **Snapshot method**: Btrfs (instant), LVM (seconds), Rsync (minutes-hours)
- **Number of packages**: More packages = longer time
- **Internet speed**: For downloading packages
- **System specs**: CPU, disk speed

Typical migration: 30 minutes to 2 hours

### Can I migrate back to my original distro?

Yes! Two ways:
1. **Revert snapshot**: `sudo eshu-shapeshifter revert`
2. **Transform again**: `sudo eshu-shapeshifter shapeshift <original-distro>`

## Compatibility Questions

### Which distributions are supported?

**Fully Supported**:
- Arch Linux
- Ubuntu 22.04+
- Debian 12+
- Fedora 39+
- openSUSE Tumbleweed

**Coming Soon**:
- Manjaro
- EndeavourOS
- Pop!_OS
- Linux Mint
- Rocky Linux

### Can I use this on a server?

Yes, but be cautious:
- Test in a VM first
- Have console access (not just SSH)
- Ensure you can reboot safely
- Consider the downtime

### Does it work with custom kernels?

Partially. The migration will:
- Install the target distro's default kernel
- You may need to reinstall custom kernel modules
- DKMS modules should rebuild automatically

### What about proprietary drivers (NVIDIA, etc.)?

Proprietary drivers may need reconfiguration:
- NVIDIA: May need to reinstall driver for new kernel
- AMD: Usually works out of the box
- WiFi: Check if firmware packages are translated

### Can I migrate from/to NixOS?

NixOS is fundamentally different (declarative, immutable). Migration to/from NixOS is not currently supported and would require a complete redesign.

## Filesystem Questions

### Do I need btrfs?

No, but it's recommended:
- **With btrfs**: Instant snapshots, fast rollback
- **Without btrfs**: Rsync backups (slower but works)

### How much disk space do I need?

Depends on snapshot method:
- **Btrfs**: Minimal (copy-on-write)
- **LVM**: ~10GB for snapshot
- **Rsync**: Equal to your used space

Plus space for new packages (~5-10GB typically).

### Can I use this on an encrypted system?

Yes, but:
- Ensure you have the encryption password
- Bootloader may need reconfiguration
- Test in a VM first

## Troubleshooting

### Migration failed, what do I do?

1. Don't panic - your snapshot is safe
2. Check logs: `/var/log/eshu-shapeshifter/migration.log`
3. Revert: `sudo eshu-shapeshifter revert`
4. Report the issue with logs

### System won't boot after migration

1. Boot from live USB
2. Mount your root partition
3. Chroot into the system
4. Run: `eshu-shapeshifter revert`
5. Reboot

### Some applications don't work

This can happen if:
- Package wasn't translated correctly
- Dependencies are missing
- Config files need adjustment

Check which packages failed to install and manually install equivalents.

### Network doesn't work after migration

Network configs may need adjustment:
- Check NetworkManager is enabled: `systemctl enable NetworkManager`
- Verify network config files in `/etc`
- Restart network: `systemctl restart NetworkManager`

### Bootloader issues

If GRUB doesn't work:
1. Boot from live USB
2. Chroot into system
3. Reinstall GRUB: `grub-install /dev/sdX`
4. Update config: `grub-mkconfig -o /boot/grub/grub.cfg`

## Best Practices

### Should I test in a VM first?

**Absolutely!** Always test in a VM before running on your main system:
1. Create a VM with your current distro
2. Set it up similarly to your main system
3. Run the migration
4. Verify everything works
5. Then try on your main system

### What should I backup before migrating?

Even with snapshots, backup:
- Important documents
- Project files
- Configuration files you've customized
- SSH keys, GPG keys
- Browser profiles
- Database dumps

### How do I prepare for migration?

1. **Update your system**: Ensure all packages are up to date
2. **Clean up**: Remove unused packages
3. **Backup**: External backup of important data
4. **Document**: Note any custom configurations
5. **Test**: Try in a VM first
6. **Time**: Do it when you have time to troubleshoot

### When should I NOT use this?

Don't use Eshu if:
- You're running critical production systems
- You don't have time to troubleshoot
- You can't afford any downtime
- You're not comfortable with the command line
- You haven't backed up your data

## Advanced Usage

### Can I automate migrations?

Yes, but not recommended for production:
```bash
# Non-interactive mode (use with caution)
sudo eshu-shapeshifter shapeshift arch --yes
```

### Can I create custom distro profiles?

Yes! Create a TOML file:
```toml
name = "My Custom Distro"
version = "1.0"
family = "Arch"
# ... rest of profile
```

Then use: `sudo eshu-shapeshifter shapeshift --custom-profile my-distro.toml`

### Can I migrate multiple systems?

Yes, but each system needs its own migration. You could:
1. Create a custom profile with your preferred packages
2. Use the same profile on multiple systems
3. Script the process (carefully!)

### How do I contribute package mappings?

1. Fork the repository
2. Add mappings to `src/package.rs`
3. Test the mappings
4. Submit a pull request

## Performance

### How can I speed up migrations?

- Use btrfs for instant snapshots
- Have a fast internet connection
- Use a local package mirror
- Migrate fewer packages (minimal install)
- Use SSD instead of HDD

### Does it affect system performance after migration?

No, once migrated, your system runs at native performance of the target distribution.

## Support

### Where can I get help?

- GitHub Issues: Bug reports and feature requests
- GitHub Discussions: Questions and community help
- Wiki: Detailed guides and tutorials
- IRC/Discord: Real-time community support (coming soon)

### How do I report a bug?

1. Check if it's already reported
2. Gather information:
   - Source and target distros
   - Error messages
   - Log files
   - System specs
3. Create a GitHub issue with details

### Can I request a new distribution?

Yes! Open a feature request with:
- Distribution name and version
- Why you want it supported
- Willingness to help test

## Philosophy

### Why "Eshu Shapeshifter"?

Eshu is a Yoruba deity associated with crossroads, choices, and transformation. The name reflects the tool's purpose: helping you navigate the crossroads of Linux distributions and transform your system.

### Is this production-ready?

Eshu is in active development. While it includes extensive safety features:
- ✅ Use for personal systems
- ✅ Use for testing/development
- ⚠️ Use caution on servers
- ❌ Not recommended for critical production systems (yet)

### What's the long-term vision?

- Support for all major distributions
- GUI interface for easier use
- Cloud backup integration
- Container-based testing
- Community-driven package mappings
- Enterprise support options
