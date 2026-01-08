#!/bin/bash
# Eshu Shapeshifter Installation Script

set -e

INSTALL_DIR="/usr/local/bin"
CONFIG_DIR="/etc/eshu-shapeshifter"
DATA_DIR="/var/lib/eshu-shapeshifter"
CACHE_DIR="/var/cache/eshu-shapeshifter"

echo "🔮 Eshu Shapeshifter Installation"
echo "=================================="
echo ""

# Check if running as root
if [ "$EUID" -ne 0 ]; then 
    echo "❌ Please run as root (use sudo)"
    exit 1
fi

# Check for Rust/Cargo
if ! command -v cargo &> /dev/null; then
    echo "❌ Cargo not found. Please install Rust first:"
    echo "   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh"
    exit 1
fi

echo "✓ Rust/Cargo found"

# Build the project
echo ""
echo "📦 Building Eshu Shapeshifter..."
cargo build --release

if [ ! -f "target/release/eshu-shapeshifter" ]; then
    echo "❌ Build failed"
    exit 1
fi

echo "✓ Build successful"

# Install binary
echo ""
echo "📥 Installing binary to $INSTALL_DIR..."
cp target/release/eshu-shapeshifter "$INSTALL_DIR/"
chmod +x "$INSTALL_DIR/eshu-shapeshifter"
echo "✓ Binary installed"

# Create directories
echo ""
echo "📁 Creating directories..."
mkdir -p "$CONFIG_DIR"
mkdir -p "$DATA_DIR"
mkdir -p "$DATA_DIR/snapshots"
mkdir -p "$CACHE_DIR"
mkdir -p "$CACHE_DIR/profiles"
echo "✓ Directories created"

# Create default config
echo ""
echo "⚙️  Creating default configuration..."
cat > "$CONFIG_DIR/config.toml" << EOF
version = "0.1.0"
data_dir = "$DATA_DIR"
snapshot_dir = "$DATA_DIR/snapshots"
repository_url = "https://raw.githubusercontent.com/eshu-shapeshifter/distro-profiles/main"
cache_dir = "$CACHE_DIR"
EOF
echo "✓ Configuration created"

# Set permissions
echo ""
echo "🔒 Setting permissions..."
chown -R root:root "$CONFIG_DIR"
chown -R root:root "$DATA_DIR"
chown -R root:root "$CACHE_DIR"
chmod 755 "$CONFIG_DIR"
chmod 755 "$DATA_DIR"
chmod 755 "$CACHE_DIR"
echo "✓ Permissions set"

# Check for btrfs
echo ""
echo "🔍 Checking filesystem..."
ROOT_FS=$(findmnt -n -o FSTYPE /)
if [ "$ROOT_FS" = "btrfs" ]; then
    echo "✓ Btrfs detected - fast snapshots available!"
else
    echo "⚠️  Non-btrfs filesystem ($ROOT_FS) - snapshots will use rsync"
    echo "   Consider using btrfs for better performance"
fi

# Installation complete
echo ""
echo "═══════════════════════════════════════"
echo "✅ Installation complete!"
echo "═══════════════════════════════════════"
echo ""
echo "Quick start:"
echo "  1. Scan your system:     sudo eshu-shapeshifter scan"
echo "  2. List distributions:   sudo eshu-shapeshifter list"
echo "  3. Transform:            sudo eshu-shapeshifter shapeshift <distro>"
echo ""
echo "Documentation: https://github.com/yourusername/eshu-shapeshifter"
echo ""
echo "⚠️  IMPORTANT: Always backup important data before transforming!"
echo ""
