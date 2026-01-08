mod cli;
mod config;
mod distro;
mod error;
mod license;
mod migration;
mod package;
mod scanner;
mod snapshot;
mod translation;
mod repository;

use anyhow::Result;
use clap::Parser;
use cli::{Cli, Commands};
use colored::Colorize;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize logging
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "eshu_shapeshifter=info".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    // Check for root privileges
    if !is_root() {
        eprintln!("{}", "⚠️  Eshu Shapeshifter requires root privileges to modify system files.".red().bold());
        eprintln!("Please run with: sudo eshu-shapeshifter <command>");
        std::process::exit(1);
    }

    print_banner();

    let cli = Cli::parse();

    match cli.command {
        Commands::Scan => {
            scanner::scan_system().await?;
        }
        Commands::List => {
            repository::list_available_distros().await?;
        }
        Commands::Shapeshift { target, custom_iso } => {
            // Check license before allowing shapeshift
            if !license::check_license_and_prompt().await? {
                std::process::exit(1);
            }

            migration::shapeshift(target, custom_iso).await?;

            // Record usage after successful shapeshift
            let mut license_info = license::LicenseInfo::load()?;
            license_info.use_shapeshift()?;

            // Show remaining shapeshifts
            let remaining = license_info.get_shifts_remaining();
            if remaining != u32::MAX {
                println!("\n{}", format!("💫 Shapeshifts remaining: {}", remaining).cyan());
            }
        }
        Commands::Revert { snapshot_id } => {
            snapshot::revert_snapshot(snapshot_id).await?;
        }
        Commands::Snapshots => {
            snapshot::list_snapshots().await?;
        }
        Commands::Status => {
            scanner::show_status().await?;
        }
        Commands::Validate { target } => {
            migration::validate_migration(target).await?;
        }
        Commands::Activate { license_key, product } => {
            let mut license_info = license::LicenseInfo::load()?;
            license_info.activate_license(license_key, product).await?;
        }
        Commands::License => {
            let license_info = license::LicenseInfo::load()?;
            license_info.display_status();
        }
    }

    Ok(())
}

fn is_root() -> bool {
    unsafe { libc::geteuid() == 0 }
}

fn print_banner() {
    println!("{}", r#"
    ╔═══════════════════════════════════════════════════════╗
    ║                                                       ║
    ║   ███████╗███████╗██╗  ██╗██╗   ██╗                 ║
    ║   ██╔════╝██╔════╝██║  ██║██║   ██║                 ║
    ║   █████╗  ███████╗███████║██║   ██║                 ║
    ║   ██╔══╝  ╚════██║██╔══██║██║   ██║                 ║
    ║   ███████╗███████║██║  ██║╚██████╔╝                 ║
    ║   ╚══════╝╚══════╝╚═╝  ╚═╝ ╚═════╝                  ║
    ║                                                       ║
    ║        SHAPESHIFTER - Linux Distro Transformer       ║
    ║           "Change your skin, keep your soul"         ║
    ║                                                       ║
    ╚═══════════════════════════════════════════════════════╝
    "#.cyan().bold());
}
