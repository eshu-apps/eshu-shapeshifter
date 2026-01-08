use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "eshu-shapeshifter")]
#[command(author, version, about, long_about = None)]
#[command(
    about = "Transform your Linux distribution without reinstalling",
    long_about = "Eshu Shapeshifter allows you to migrate between Linux distributions \
                  while preserving your data, applications, and configurations. \
                  Create snapshots to easily revert back to your original system."
)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Scan current system and display configuration
    Scan,

    /// List available distributions from repository
    List,

    /// Transform into a different Linux distribution
    Shapeshift {
        /// Target distribution (e.g., 'arch', 'ubuntu', 'fedora')
        target: String,

        /// Optional: Path to custom ISO file
        #[arg(short, long)]
        custom_iso: Option<String>,
    },

    /// Revert to a previous snapshot
    Revert {
        /// Snapshot ID to revert to (use 'snapshots' command to list)
        snapshot_id: Option<String>,
    },

    /// List all available snapshots
    Snapshots,

    /// Show current system status and transformation history
    Status,

    /// Validate if migration to target distro is possible
    Validate {
        /// Target distribution to validate
        target: String,
    },

    /// Activate a license key
    Activate {
        /// Gumroad license key
        license_key: String,

        /// Product permalink (e.g., 'eshu-shapeshifter-unlimited' or 'eshu-shapeshifter-pack')
        #[arg(short, long, default_value = "eshu-shapeshifter-unlimited")]
        product: String,
    },

    /// Show license status and usage
    License,
}
