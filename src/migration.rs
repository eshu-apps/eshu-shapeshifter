use crate::config::{EshuConfig, DistroProfile};
use crate::distro;
use crate::error::{EshuError, EshuResult};
use crate::package::{PackageTranslator, get_essential_packages};
use crate::repository;
use crate::scanner;
use crate::snapshot;
use crate::translation::{ConfigTranslator, preserve_home_directories};
use colored::Colorize;
use dialoguer::Confirm;
use indicatif::{ProgressBar, ProgressStyle};
use std::fs;
use std::path::PathBuf;
use std::process::Command;

/// Main shapeshift function - transform the system to a different distro
pub async fn shapeshift(target: String, custom_iso: Option<String>) -> anyhow::Result<()> {
    println!("{}", "\n🔮 Eshu Shapeshifter - System Transformation".cyan().bold());
    println!("{}", "═══════════════════════════════════════════════".cyan());

    // Step 1: Scan current system
    println!("\n{}", "Step 1: Scanning current system...".yellow().bold());
    let current_state = scanner::collect_system_state().await?;
    println!("  Current: {} {} ({:?})", 
        current_state.distro.green(),
        current_state.version,
        current_state.family
    );

    // Step 2: Get target distro profile
    println!("\n{}", "Step 2: Loading target distribution profile...".yellow().bold());
    let target_profile = if let Some(iso_path) = custom_iso {
        println!("  Using custom ISO: {}", iso_path);
        load_custom_iso_profile(&iso_path)?
    } else {
        repository::get_distro_profile(&target)
            .ok_or_else(|| anyhow::anyhow!("Distribution '{}' not found in repository", target))?
    };
    println!("  Target: {} {} ({:?})", 
        target_profile.name.green(),
        target_profile.version,
        target_profile.family
    );

    // Step 3: Validate migration
    println!("\n{}", "Step 3: Validating migration...".yellow().bold());
    validate_migration_internal(&current_state, &target_profile)?;
    println!("  {}", "✓ Migration is possible".green());

    // Step 4: Confirm with user
    println!("\n{}", "⚠️  WARNING: This will transform your system!".red().bold());
    println!("From: {} {}", current_state.distro, current_state.version);
    println!("To:   {} {}", target_profile.name, target_profile.version);
    println!("\n{}", "✅ A snapshot will be created for rollback.".green());
    println!("{}", "✅ You can revert anytime with: eshu-shapeshifter revert".green());

    let confirmed = Confirm::new()
        .with_prompt("Do you want to continue?")
        .default(false)
        .interact()?;

    if !confirmed {
        println!("{}", "Transformation cancelled.".yellow());
        return Ok(());
    }

    // Step 5: Create snapshot (with validation)
    println!("\n{}", "Step 5: Creating system snapshot...".yellow().bold());
    let snapshot_result = snapshot::create_snapshot(
        format!("Before migration to {} {}", target_profile.name, target_profile.version)
    ).await;

    let snapshot = match snapshot_result {
        Ok(snap) => {
            println!("  {}", format!("✓ Snapshot created: {}", snap.id).green());
            
            // Validate snapshot was actually created
            if !snap.path.exists() {
                eprintln!("  {}", "⚠️  Warning: Snapshot path doesn't exist!".yellow());
                eprintln!("  {}", "Continuing anyway, but rollback may not work.".yellow());
            } else {
                println!("  {}", format!("✓ Snapshot validated at: {}", snap.path.display()).green());
            }
            
            Some(snap)
        }
        Err(e) => {
            eprintln!("  {}", format!("⚠️  Warning: Snapshot creation failed: {}", e).yellow());
            eprintln!("  {}", "This means you won't be able to automatically rollback!".red().bold());
            
            let continue_anyway = Confirm::new()
                .with_prompt("Continue without snapshot? (NOT RECOMMENDED)")
                .default(false)
                .interact()?;
            
            if !continue_anyway {
                println!("{}", "Transformation cancelled for safety.".yellow());
                return Ok(());
            }
            
            eprintln!("  {}", "⚠️  Proceeding without snapshot protection!".red().bold());
            None
        }
    };

    // Step 6: Prepare package translations
    println!("\n{}", "Step 6: Translating packages...".yellow().bold());
    let config = EshuConfig::load()?;
    let db_path = config.data_dir.join("package_mappings.db");
    let translator = PackageTranslator::new(db_path.to_str().unwrap())?;
    
    let translation_result = translator.translate_packages(
        &current_state.family,
        &target_profile.family,
        &current_state.installed_packages,
    )?;

    println!("  Translated: {} packages", translation_result.translated.len());
    println!("  Untranslated: {} packages", translation_result.untranslated.len());
    println!("  Skipped: {} packages", translation_result.skipped.len());

    if !translation_result.untranslated.is_empty() {
        println!("\n  {}", "⚠️  Some packages could not be translated:".yellow());
        for pkg in translation_result.untranslated.iter().take(10) {
            println!("    - {}", pkg);
        }
        if translation_result.untranslated.len() > 10 {
            println!("    ... and {} more", translation_result.untranslated.len() - 10);
        }
    }

    // Step 7: Prepare configuration translations
    println!("\n{}", "Step 7: Preparing configuration translations...".yellow().bold());
    let mut config_translator = ConfigTranslator::new();
    config_translator.build_rules(&current_state.family, &target_profile.family);
    
    let backup_dir = config.data_dir.join("config_backup");
    fs::create_dir_all(&backup_dir)?;
    
    let config_ops = config_translator.translate_configs(&backup_dir)?;
    println!("  {} configuration operations prepared", config_ops.len());

    // Step 8: Preserve user data
    println!("\n{}", "Step 8: Preserving user data...".yellow().bold());
    let user_backup_dir = config.data_dir.join("user_backup");
    fs::create_dir_all(&user_backup_dir)?;
    preserve_home_directories(&current_state, &user_backup_dir)?;
    println!("  {}", "✓ User data backed up".green());

    // Step 9: Execute migration (with error handling)
    println!("\n{}", "Step 9: Executing migration...".yellow().bold());
    let migration_result = execute_migration(&current_state, &target_profile, &translation_result, &config_ops).await;

    match migration_result {
        Ok(_) => {
            println!("  {}", "✓ Migration completed successfully".green());
        }
        Err(e) => {
            eprintln!("\n{}", "❌ Migration failed!".red().bold());
            eprintln!("Error: {}", e);
            
            if let Some(ref snap) = snapshot {
                eprintln!("\n{}", "🔄 You can rollback with:".yellow().bold());
                eprintln!("  sudo eshu-shapeshifter revert {}", snap.id);
            } else {
                eprintln!("\n{}", "⚠️  No snapshot available for automatic rollback!".red().bold());
                eprintln!("You may need to manually restore your system.");
            }
            
            return Err(e);
        }
    }

    // Step 10: Record transformation
    if let Some(ref snap) = snapshot {
        record_transformation(&current_state.distro, &target_profile.name, &snap.id)?;
    } else {
        record_transformation(&current_state.distro, &target_profile.name, "no-snapshot")?;
    }

    println!("\n{}", "═══════════════════════════════════════════════".cyan());
    println!("{}", "✅ Transformation complete!".green().bold());
    println!("\n{}", "Next steps:".yellow().bold());
    println!("  1. Review the changes");
    println!("  2. Reboot your system");
    
    if let Some(ref snap) = snapshot {
        println!("  3. If issues occur, use: eshu-shapeshifter revert {}", snap.id);
    }
    
    println!("\n{}", "⚠️  IMPORTANT: Reboot required for changes to take effect!".red().bold());

    Ok(())
}

/// Validate if migration is possible
pub async fn validate_migration(target: String) -> anyhow::Result<()> {
    println!("{}", "\n🔍 Validating migration...".cyan().bold());

    let current_state = scanner::collect_system_state().await?;
    let target_profile = repository::get_distro_profile(&target)
        .ok_or_else(|| anyhow::anyhow!("Distribution '{}' not found", target))?;

    match validate_migration_internal(&current_state, &target_profile) {
        Ok(_) => {
            println!("{}", "\n✅ Migration is possible!".green().bold());
            println!("From: {} {}", current_state.distro, current_state.version);
            println!("To:   {} {}", target_profile.name, target_profile.version);
        }
        Err(e) => {
            println!("{}", format!("\n❌ Migration not possible: {}", e).red().bold());
        }
    }

    Ok(())
}

fn validate_migration_internal(
    current_state: &crate::config::SystemState,
    target_profile: &DistroProfile,
) -> EshuResult<()> {
    // Check if same distro
    if current_state.family == target_profile.family 
        && current_state.distro.to_lowercase().contains(&target_profile.name.to_lowercase()) 
    {
        return Err(EshuError::Validation(
            "Already running the target distribution".to_string()
        ));
    }

    // Check filesystem compatibility
    if current_state.filesystem_type == "btrfs" {
        println!("  {}", "✓ Btrfs detected - snapshots will be fast".green());
    } else {
        println!("  {}", "⚠️  Non-btrfs filesystem - snapshots will use rsync (slower)".yellow());
    }

    // Check architecture compatibility
    if current_state.architecture != "x86_64" && current_state.architecture != "aarch64" {
        return Err(EshuError::Validation(
            format!("Unsupported architecture: {}", current_state.architecture)
        ));
    }

    // Check bootloader compatibility
    if current_state.boot_loader == "unknown" {
        println!("  {}", "⚠️  Could not detect bootloader - manual configuration may be needed".yellow());
    }

    // Check for sufficient disk space
    match check_disk_space_for_migration() {
        Ok(_) => println!("  {}", "✓ Sufficient disk space available".green()),
        Err(e) => {
            println!("  {}", format!("⚠️  Warning: {}", e).yellow());
            println!("  {}", "Migration may fail if disk space is insufficient".yellow());
        }
    }

    Ok(())
}

fn check_disk_space_for_migration() -> EshuResult<()> {
    let output = Command::new("df")
        .args(&["-B1", "/"])
        .output()
        .map_err(|e| EshuError::Validation(format!("Failed to check disk space: {}", e)))?;

    if !output.status.success() {
        return Err(EshuError::Validation("Failed to check disk space".to_string()));
    }

    let stdout = String::from_utf8_lossy(&output.stdout);
    let lines: Vec<&str> = stdout.lines().collect();

    if lines.len() < 2 {
        return Err(EshuError::Validation("Could not parse disk space information".to_string()));
    }

    let parts: Vec<&str> = lines[1].split_whitespace().collect();
    if parts.len() < 4 {
        return Err(EshuError::Validation("Could not parse disk space information".to_string()));
    }

    let available_bytes: u64 = parts[3].parse()
        .map_err(|_| EshuError::Validation("Could not parse available space".to_string()))?;

    // Require at least 10GB free for migration
    const MIN_SPACE_BYTES: u64 = 10 * 1024 * 1024 * 1024; // 10GB

    if available_bytes < MIN_SPACE_BYTES {
        let available_gb = available_bytes as f64 / (1024.0 * 1024.0 * 1024.0);
        return Err(EshuError::Validation(
            format!(
                "Insufficient disk space. Need at least 10GB, but only {:.2}GB available",
                available_gb
            )
        ));
    }

    Ok(())
}

async fn execute_migration(
    current_state: &crate::config::SystemState,
    target_profile: &DistroProfile,
    translation_result: &crate::package::TranslationResult,
    config_ops: &[crate::translation::ConfigOperation],
) -> anyhow::Result<()> {
    let pb = ProgressBar::new(100);
    pb.set_style(
        ProgressStyle::default_bar()
            .template("{spinner:.green} [{bar:40.cyan/blue}] {pos}/{len} {msg}")
            .unwrap()
            .progress_chars("#>-")
    );

    // Phase 1: Setup target package manager (10%)
    pb.set_message("Setting up target package manager...");
    setup_target_package_manager(target_profile)?;
    pb.set_position(10);

    // Phase 2: Install base packages (30%)
    pb.set_message("Installing base packages...");
    install_base_packages(target_profile)?;
    pb.set_position(40);

    // Phase 3: Install translated packages (40%)
    pb.set_message("Installing translated packages...");
    install_translated_packages(target_profile, translation_result)?;
    pb.set_position(80);

    // Phase 4: Apply configuration translations (10%)
    pb.set_message("Applying configuration translations...");
    for op in config_ops {
        op.execute()?;
    }
    pb.set_position(90);

    // Phase 5: Run post-migration hooks (10%)
    pb.set_message("Running post-migration hooks...");
    run_hooks(&target_profile.post_migration_hooks)?;
    pb.set_position(100);

    pb.finish_with_message("Migration complete");

    Ok(())
}

fn setup_target_package_manager(profile: &DistroProfile) -> EshuResult<()> {
    // This is a simplified version - in production, you'd need to:
    // 1. Download and install the target package manager
    // 2. Configure repositories
    // 3. Initialize package database

    println!("  Setting up {} package manager...", profile.package_manager.name);

    // Run pre-migration hooks
    run_hooks(&profile.pre_migration_hooks)?;

    Ok(())
}

fn install_base_packages(profile: &DistroProfile) -> EshuResult<()> {
    println!("  Installing base packages for {}...", profile.name);

    for package in &profile.base_packages {
        println!("    Installing {}...", package);
        
        let output = Command::new("sh")
            .arg("-c")
            .arg(format!("{} {}", profile.package_manager.install_cmd, package))
            .output()
            .map_err(|e| EshuError::PackageManager(format!("Failed to install {}: {}", package, e)))?;

        if !output.status.success() {
            eprintln!("    ⚠️  Warning: Failed to install {}", package);
        }
    }

    Ok(())
}

fn install_translated_packages(
    profile: &DistroProfile,
    translation_result: &crate::package::TranslationResult,
) -> EshuResult<()> {
    println!("  Installing {} translated packages...", translation_result.translated.len());

    // Install in batches to avoid command line length limits
    let batch_size = 50;
    for chunk in translation_result.translated.chunks(batch_size) {
        let packages: Vec<String> = chunk.iter()
            .filter(|m| m.confidence > 0.5)
            .map(|m| m.target.clone())
            .collect();

        if packages.is_empty() {
            continue;
        }

        let package_list = packages.join(" ");
        
        let output = Command::new("sh")
            .arg("-c")
            .arg(format!("{} {}", profile.package_manager.install_cmd, package_list))
            .output()
            .map_err(|e| EshuError::PackageManager(format!("Failed to install packages: {}", e)))?;

        if !output.status.success() {
            eprintln!("    ⚠️  Warning: Some packages failed to install");
        }
    }

    Ok(())
}

fn run_hooks(hooks: &[String]) -> EshuResult<()> {
    for hook in hooks {
        println!("    Running: {}", hook);
        
        let output = Command::new("sh")
            .arg("-c")
            .arg(hook)
            .output()
            .map_err(|e| EshuError::Migration(format!("Hook failed: {}", e)))?;

        if !output.status.success() {
            eprintln!("    ⚠️  Warning: Hook failed: {}", hook);
            eprintln!("    {}", String::from_utf8_lossy(&output.stderr));
        }
    }

    Ok(())
}

fn load_custom_iso_profile(iso_path: &str) -> EshuResult<DistroProfile> {
    // This would need to:
    // 1. Mount the ISO
    // 2. Detect the distro from ISO contents
    // 3. Extract package lists
    // 4. Build a profile
    
    // For now, return an error
    Err(EshuError::UnsupportedDistro(
        "Custom ISO support not yet implemented. Use curated distros for now.".to_string()
    ))
}

fn record_transformation(from: &str, to: &str, snapshot_id: &str) -> anyhow::Result<()> {
    let config = EshuConfig::load()?;
    let history_path = config.data_dir.join("history.json");

    let mut history = if history_path.exists() {
        let content = fs::read_to_string(&history_path)?;
        serde_json::from_str(&content).unwrap_or_else(|_| Vec::new())
    } else {
        Vec::new()
    };

    let record = serde_json::json!({
        "from_distro": from,
        "to_distro": to,
        "timestamp": chrono::Utc::now().to_rfc3339(),
        "snapshot_id": snapshot_id,
    });

    history.push(record);

    let json = serde_json::to_string_pretty(&history)?;
    fs::write(history_path, json)?;

    Ok(())
}
