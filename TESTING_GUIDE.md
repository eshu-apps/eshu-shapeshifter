# 🧪 Eshu Shapeshifter - Testing Guide

## Quick Test Commands

### 1. Basic Functionality Test
```bash
# Check if app runs
sudo eshu-shapeshifter --help

# Check status
sudo eshu-shapeshifter status

# Scan system
sudo eshu-shapeshifter scan

# List available distros
sudo eshu-shapeshifter list

# Validate a migration (doesn't actually migrate)
sudo eshu-shapeshifter validate fedora
```

### 2. Snapshot Testing
```bash
# List existing snapshots
sudo eshu-shapeshifter snapshots

# The snapshot creation is tested during actual shapeshift
# To test snapshot creation in isolation, you'd need to:
# 1. Run a shapeshift (it creates snapshot automatically)
# 2. Cancel when prompted
```

### 3. License Testing
```bash
# Check license status
sudo eshu-shapeshifter license

# Test activation (requires valid key)
sudo eshu-shapeshifter activate YOUR_LICENSE_KEY eshu-shapeshifter
```

### 4. Full Migration Test (Safe - with cancellation)
```bash
# Start a migration but cancel when prompted
sudo eshu-shapeshifter shapeshift fedora
# Press 'n' when asked to confirm
```

---

## Test Scenarios

### Scenario A: Normal Operation
**Goal:** Verify everything works when conditions are ideal

```bash
# 1. Check you have enough disk space (20GB+ free)
df -h /

# 2. Scan system
sudo eshu-shapeshifter scan

# 3. Validate migration
sudo eshu-shapeshifter validate ubuntu

# 4. Check license
sudo eshu-shapeshifter license

# Expected: All commands succeed, clear output
```

---

### Scenario B: Low Disk Space Warning
**Goal:** Verify disk space checking works

```bash
# Check current space
df -h /

# Run validation
sudo eshu-shapeshifter validate fedora

# Expected: Warning if < 10GB free for migration
# Expected: Warning if < 20GB free for rsync snapshots
```

---

### Scenario C: Snapshot Creation
**Goal:** Verify snapshot is created and validated

```bash
# Start a migration (will create snapshot)
sudo eshu-shapeshifter shapeshift fedora

# When prompted, say YES to continue
# When prompted about transformation, say NO to cancel

# Check if snapshot was created
sudo eshu-shapeshifter snapshots

# Expected: New snapshot listed with current timestamp
# Expected: Status shows "✓ Available"
```

---

### Scenario D: Snapshot Revert (Btrfs)
**Goal:** Verify btrfs revert provides clear instructions

**Prerequisites:** You must be on a btrfs filesystem

```bash
# Create a snapshot first (see Scenario C)

# Try to revert
sudo eshu-shapeshifter revert

# Select the snapshot
# Read the instructions provided

# Expected: Clear instructions for 3 revert methods
# Expected: Option to use rsync fallback
```

---

### Scenario E: Snapshot Revert (Non-Btrfs)
**Goal:** Verify rsync revert works

**Prerequisites:** You must NOT be on btrfs/LVM

```bash
# Create a snapshot first (see Scenario C)

# Try to revert
sudo eshu-shapeshifter revert

# Select the snapshot
# Confirm when prompted

# Expected: Rsync restore process
# Expected: Success message
```

---

### Scenario F: License Exhaustion
**Goal:** Verify behavior when free trial is exhausted

```bash
# Check current license status
sudo eshu-shapeshifter license

# If you have free shifts remaining, use them:
# (This will actually start migrations, so be careful!)

# Try to shapeshift after exhausting free trial
sudo eshu-shapeshifter shapeshift fedora

# Expected: Clear upgrade prompt with pricing
# Expected: Gumroad link provided
# Expected: Activation instructions
```

---

### Scenario G: Invalid License Key
**Goal:** Verify license validation works

```bash
# Try to activate with fake key
sudo eshu-shapeshifter activate FAKE-KEY-12345 eshu-shapeshifter

# Expected: "Invalid license key" error
# Expected: Clear error message
```

---

### Scenario H: Network Failure During License Check
**Goal:** Verify graceful handling of network issues

```bash
# Disconnect network
sudo systemctl stop NetworkManager

# Try to activate license
sudo eshu-shapeshifter activate SOME-KEY eshu-shapeshifter

# Reconnect network
sudo systemctl start NetworkManager

# Expected: Network error message
# Expected: No crash or hang
```

---

### Scenario I: Snapshot Path Missing
**Goal:** Verify detection of broken snapshots

```bash
# Create a snapshot
sudo eshu-shapeshifter shapeshift fedora
# Cancel when prompted

# Manually delete snapshot directory
sudo rm -rf /var/lib/eshu/snapshots/snapshot_*

# List snapshots
sudo eshu-shapeshifter snapshots

# Expected: Status shows "⚠️ Path missing"
```

---

### Scenario J: Permission Denied
**Goal:** Verify proper error handling for permission issues

```bash
# Try to run without sudo
eshu-shapeshifter scan

# Expected: Clear error about needing root privileges
# Expected: Suggestion to use sudo
```

---

## Automated Test Script

Save this as `test_eshu.sh`:

```bash
#!/bin/bash

echo "🧪 Eshu Shapeshifter Test Suite"
echo "================================"

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Test counter
PASSED=0
FAILED=0

# Test function
test_command() {
    local name="$1"
    local command="$2"
    local expected_exit="$3"
    
    echo -n "Testing: $name ... "
    
    eval "$command" > /dev/null 2>&1
    local exit_code=$?
    
    if [ $exit_code -eq $expected_exit ]; then
        echo -e "${GREEN}✓ PASS${NC}"
        ((PASSED++))
    else
        echo -e "${RED}✗ FAIL${NC} (exit code: $exit_code, expected: $expected_exit)"
        ((FAILED++))
    fi
}

# Run tests
echo ""
echo "Running tests..."
echo ""

test_command "Help command" "eshu-shapeshifter --help" 0
test_command "Version command" "eshu-shapeshifter --version" 0
test_command "Status without sudo" "eshu-shapeshifter status" 1
test_command "Status with sudo" "sudo eshu-shapeshifter status" 0
test_command "Scan with sudo" "sudo eshu-shapeshifter scan" 0
test_command "List distros" "sudo eshu-shapeshifter list" 0
test_command "List snapshots" "sudo eshu-shapeshifter snapshots" 0
test_command "License status" "sudo eshu-shapeshifter license" 0
test_command "Validate migration" "sudo eshu-shapeshifter validate fedora" 0

# Summary
echo ""
echo "================================"
echo "Test Results:"
echo -e "${GREEN}Passed: $PASSED${NC}"
echo -e "${RED}Failed: $FAILED${NC}"
echo "================================"

if [ $FAILED -eq 0 ]; then
    echo -e "${GREEN}All tests passed! ✓${NC}"
    exit 0
else
    echo -e "${RED}Some tests failed! ✗${NC}"
    exit 1
fi
```

Run it:
```bash
chmod +x test_eshu.sh
./test_eshu.sh
```

---

## Manual Verification Checklist

After running tests, manually verify:

- [ ] Help text is clear and complete
- [ ] Status shows correct system information
- [ ] Scan detects packages correctly
- [ ] List shows available distros
- [ ] Snapshots list is formatted nicely
- [ ] License status is accurate
- [ ] Error messages are helpful
- [ ] Colors render correctly in terminal
- [ ] Progress bars work smoothly
- [ ] Confirmations prompt correctly
- [ ] Warnings are visible and clear

---

## Performance Testing

### Snapshot Creation Speed
```bash
# Time snapshot creation
time sudo eshu-shapeshifter shapeshift fedora
# Cancel when prompted

# Expected times:
# - Btrfs: < 5 seconds
# - LVM: < 30 seconds  
# - Rsync: 2-10 minutes (depends on system size)
```

### Migration Speed
```bash
# Full migration timing
# (Only do this in a VM or test system!)
time sudo eshu-shapeshifter shapeshift fedora

# Expected: 10-30 minutes depending on:
# - Number of packages
# - Network speed
# - Disk speed
```

---

## Debugging

### Enable Verbose Logging
```bash
RUST_LOG=debug sudo eshu-shapeshifter scan
```

### Check Log Files
```bash
# Check system logs
sudo journalctl -u eshu-shapeshifter

# Check application data
ls -la /var/lib/eshu/
cat /var/lib/eshu/license.json
cat /var/lib/eshu/current_state.json
cat /var/lib/eshu/history.json
```

### Inspect Snapshots
```bash
# List snapshot directories
ls -la /var/lib/eshu/snapshots/

# Check snapshot metadata
cat /var/lib/eshu/snapshots/snapshot_*.json

# Check snapshot size
du -sh /var/lib/eshu/snapshots/*
```

---

## Cleanup After Testing

```bash
# Remove test snapshots
sudo rm -rf /var/lib/eshu/snapshots/*

# Reset license (for testing)
sudo rm /var/lib/eshu/license.json

# Clear history
sudo rm /var/lib/eshu/history.json

# Or remove everything
sudo rm -rf /var/lib/eshu/
```

---

## CI/CD Integration

For automated testing in CI/CD:

```yaml
# .github/workflows/test.yml
name: Test Eshu Shapeshifter

on: [push, pull_request]

jobs:
  test:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v2
      - name: Install Rust
        uses: actions-rs/toolchain@v1
        with:
          toolchain: stable
      - name: Build
        run: cargo build --release
      - name: Run tests
        run: cargo test
      - name: Run integration tests
        run: ./test_eshu.sh
```

---

## Reporting Issues

When reporting issues, include:

1. **Command run:**
   ```bash
   sudo eshu-shapeshifter shapeshift fedora
   ```

2. **Full output:**
   ```
   [paste complete terminal output]
   ```

3. **System info:**
   ```bash
   uname -a
   cat /etc/os-release
   df -h
   ```

4. **Eshu status:**
   ```bash
   sudo eshu-shapeshifter status
   sudo eshu-shapeshifter license
   ```

5. **Logs:**
   ```bash
   RUST_LOG=debug sudo eshu-shapeshifter [command] 2>&1 | tee eshu-debug.log
   ```

---

## Success Criteria

✅ **All tests pass**
✅ **No crashes or panics**
✅ **Clear error messages**
✅ **Snapshots create successfully**
✅ **Revert works correctly**
✅ **License validation works**
✅ **User prompts are clear**
✅ **Progress indicators work**
✅ **Colors render properly**
✅ **Help text is complete**

---

*Happy Testing! 🚀*
