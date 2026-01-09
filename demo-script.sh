#!/bin/bash
# ESHU Shapeshifter Demo Recording Script
# Shows dramatic distro transformation

set -e

GREEN='\033[0;32m'
BLUE='\033[0;34m'
YELLOW='\033[1;33m'
PURPLE='\033[0;35m'
NC='\033[0m'

# Install required tools
if ! command -v asciinema &> /dev/null; then
    echo -e "${YELLOW}Installing asciinema...${NC}"
    sudo pacman -S asciinema --noconfirm || sudo apt install asciinema -y
fi

if ! command -v expect &> /dev/null; then
    echo -e "${YELLOW}Installing expect...${NC}"
    sudo pacman -S expect --noconfirm || sudo apt install expect -y
fi

# Create demo script
cat > /tmp/shapeshifter-demo.exp << 'EOF'
#!/usr/bin/expect -f

set timeout -1
set send_human {0.05 0.1 0.5 0.01 0.2}

spawn asciinema rec --overwrite /tmp/shapeshifter-demo.cast

sleep 2

# === SCENE 1: Introduction ===
send -h "# 🔮 ESHU Shapeshifter: Transform Your Linux Distro\r"
sleep 2
send -h "# Switch distros WITHOUT reinstalling!\r"
sleep 2

# === SCENE 2: Check Current System ===
send -h "\r\r"
send -h "# First, let's see what we have now\r"
sleep 1
send -h "sudo eshu-shapeshifter status\r"
sleep 4
send -h "\r"
send -h "# Running: Arch Linux\r"
send -h "# Packages: 847\r"
send -h "# Services: 23 enabled\r"
sleep 3

# === SCENE 3: Scan System ===
send -h "\r\r"
send -h "# Scan system before transformation\r"
sleep 1
send -h "sudo eshu-shapeshifter scan\r"
sleep 4
send -h "# 📦 Analyzing installed packages...\r"
sleep 2
send -h "# ⚙️  Detecting services...\r"
sleep 2
send -h "# 👥 Scanning user configurations...\r"
sleep 2
send -h "# ✅ System scan complete!\r"
sleep 2

# === SCENE 4: List Available Distros ===
send -h "\r\r"
send -h "# What distros can we transform into?\r"
sleep 1
send -h "sudo eshu-shapeshifter list\r"
sleep 4
send -h "\r"
send -h "# Available transformations:\r"
send -h "#  • Arch → Debian\r"
send -h "#  • Arch → Fedora\r"
send -h "#  • Arch → Ubuntu\r"
send -h "#  • Arch → openSUSE\r"
send -h "#  • ...and more!\r"
sleep 3

# === SCENE 5: Licensing (FREE TRIAL + PREMIUM) ===
send -h "\r\r"
send -h "# Check license status\r"
sleep 1
send -h "sudo eshu-shapeshifter license-status\r"
sleep 3
send -h "\r"
send -h "# License: Free Trial\r"
send -h "# Shapeshifts: 2/2 remaining\r"
send -h "\r"
send -h "# 💎 Want unlimited? Subscribe for \$5.99/month\r"
send -h "# 📦 Or buy shift packs: 10 for \$3.99\r"
sleep 4

# === SCENE 6: THE TRANSFORMATION! ===
send -h "\r\r"
send -h "# Let's transform Arch → Fedora!\r"
sleep 2
send -h "sudo eshu-shapeshifter shapeshift fedora\r"
sleep 2

# Dramatic transformation sequence
send -h "\r"
send -h "# 🔍 Analyzing transformation...\r"
sleep 2
send -h "# 📸 Creating snapshot 'arch-backup-2026-01-09'...\r"
sleep 2
send -h "# ✅ Snapshot created!\r"
sleep 1

send -h "\r"
send -h "# 🔄 Translating packages...\r"
sleep 2
send -h "#   pacman → dnf\r"
send -h "#   firefox (arch) → firefox (fedora)\r"
send -h "#   python (arch) → python3 (fedora)\r"
send -h "#   ...translating 847 packages\r"
sleep 4

send -h "\r"
send -h "# 📦 Installing Fedora base system...\r"
sleep 3
send -h "# ⚙️  Migrating services...\r"
sleep 2
send -h "# 👥 Preserving user data...\r"
sleep 2
send -h "# 🎨 Updating configurations...\r"
sleep 3

send -h "\r"
send -h "# ✨ TRANSFORMATION COMPLETE! ✨\r"
sleep 2

# === SCENE 7: Verify New System ===
send -h "\r\r"
send -h "# Let's verify the transformation\r"
sleep 1
send -h "cat /etc/os-release | grep PRETTY_NAME\r"
sleep 2
send -h "# PRETTY_NAME=\"Fedora Linux 39\"\r"
sleep 2

send -h "\r"
send -h "sudo eshu-shapeshifter status\r"
sleep 3
send -h "# Now running: Fedora 39\r"
send -h "# Packages: 847 (all preserved!)\r"
send -h "# Services: 23 enabled (all migrated!)\r"
sleep 3

# === SCENE 8: Rollback Option ===
send -h "\r\r"
send -h "# Changed your mind? Roll back instantly!\r"
sleep 2
send -h "# sudo eshu-shapeshifter rollback arch-backup-2026-01-09\r"
sleep 3

# === FINALE ===
send -h "\r\r"
send -h "# 🔮 ESHU Shapeshifter: Switch distros like changing clothes\r"
sleep 2
send -h "#    • Safe: Auto-snapshots before transformation\r"
send -h "#    • Smart: AI-powered package translation\r"
send -h "#    • Complete: Keeps all your data & settings\r"
sleep 3
send -h "\r"
send -h "# Get it: paru -S eshu-shapeshifter\r"
sleep 2
send -h "# Visit: https://eshu-apps.com\r"
sleep 2
send -h "\r"

send "\x04"
sleep 2

EOF

chmod +x /tmp/shapeshifter-demo.exp

echo -e "${GREEN}Starting ESHU Shapeshifter demo recording...${NC}"
echo -e "${PURPLE}This will show a simulated distro transformation!${NC}"
echo ""

/tmp/shapeshifter-demo.exp

if command -v agg &> /dev/null; then
    echo -e "${GREEN}Converting to GIF...${NC}"
    agg /tmp/shapeshifter-demo.cast /tmp/shapeshifter-demo.gif --speed 1.5
    echo -e "${GREEN}GIF created: /tmp/shapeshifter-demo.gif${NC}"
fi

echo -e "${GREEN}Demo complete!${NC}"
echo -e "${BLUE}Recording: /tmp/shapeshifter-demo.cast${NC}"
echo -e "${BLUE}View: asciinema play /tmp/shapeshifter-demo.cast${NC}"
echo ""
echo -e "${YELLOW}To convert to video for website:${NC}"
echo -e "${BLUE}  1. Upload .cast to asciinema.org${NC}"
echo -e "${BLUE}  2. Or use: svg-term --in /tmp/shapeshifter-demo.cast --out demo.svg${NC}"
echo -e "${BLUE}  3. Or record screen while playing: asciinema play /tmp/shapeshifter-demo.cast${NC}"
