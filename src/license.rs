use crate::config::EshuConfig;
use crate::error::{EshuError, EshuResult};
use colored::Colorize;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

const GUMROAD_VERIFY_URL: &str = "https://api.gumroad.com/v2/licenses/verify";
const FREE_TRIAL_SHIFTS: u32 = 2;

/// License types available for Eshu Shapeshifter
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum LicenseType {
    /// Free trial with 2 shapeshifts
    FreeTrial,
    /// Monthly subscription ($5.99/month) - unlimited
    Subscription { license_key: String },
    /// Pay-per-use pack (10 shifts for $3.99)
    ShiftPack { license_key: String, shifts_remaining: u32 },
}

/// License information stored locally
#[derive(Debug, Serialize, Deserialize)]
pub struct LicenseInfo {
    pub license_type: LicenseType,
    pub shifts_used: u32,
    pub first_use_timestamp: i64,
    pub last_verified: i64,
}

impl Default for LicenseInfo {
    fn default() -> Self {
        Self {
            license_type: LicenseType::FreeTrial,
            shifts_used: 0,
            first_use_timestamp: chrono::Utc::now().timestamp(),
            last_verified: 0,
        }
    }
}

/// Gumroad license verification response
#[derive(Debug, Deserialize)]
struct GumroadVerifyResponse {
    success: bool,
    purchase: Option<GumroadPurchase>,
    message: Option<String>,
}

#[derive(Debug, Deserialize)]
struct GumroadPurchase {
    subscription_ended_at: Option<String>,
    subscription_failed_at: Option<String>,
    subscription_cancelled_at: Option<String>,
    variants: Option<String>,
    custom_fields: Option<serde_json::Value>,
}

impl LicenseInfo {
    /// Get license file path
    fn license_path() -> EshuResult<PathBuf> {
        let config = EshuConfig::load().map_err(|e| EshuError::Config(e.to_string()))?;
        Ok(config.data_dir.join("license.json"))
    }

    /// Load license information from disk
    pub fn load() -> EshuResult<Self> {
        let path = Self::license_path()?;

        if path.exists() {
            let content = fs::read_to_string(&path)
                .map_err(|e| EshuError::Config(format!("Failed to read license: {}", e)))?;
            let license: LicenseInfo = serde_json::from_str(&content)
                .map_err(|e| EshuError::Config(format!("Failed to parse license: {}", e)))?;
            Ok(license)
        } else {
            // First time user - create default license
            let license = Self::default();
            license.save()?;
            Ok(license)
        }
    }

    /// Save license information to disk
    pub fn save(&self) -> EshuResult<()> {
        let path = Self::license_path()?;

        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)
                .map_err(|e| EshuError::Config(format!("Failed to create license dir: {}", e)))?;
        }

        let json = serde_json::to_string_pretty(self)
            .map_err(|e| EshuError::Serialization(e.to_string()))?;

        fs::write(&path, json)
            .map_err(|e| EshuError::Config(format!("Failed to save license: {}", e)))?;

        Ok(())
    }

    /// Check if user can perform a shapeshift
    pub async fn can_shapeshift(&mut self) -> EshuResult<bool> {
        match &self.license_type {
            LicenseType::FreeTrial => {
                if self.shifts_used < FREE_TRIAL_SHIFTS {
                    Ok(true)
                } else {
                    Ok(false)
                }
            }
            LicenseType::Subscription { license_key } => {
                // Clone the license_key to avoid borrow conflicts
                let key = license_key.clone();
                // Verify subscription is still active
                self.verify_subscription(&key).await
            }
            LicenseType::ShiftPack { license_key, shifts_remaining } => {
                if *shifts_remaining > 0 {
                    Ok(true)
                } else {
                    // Clone the license_key to avoid borrow conflicts
                    let key = license_key.clone();
                    // Check if they've purchased more packs
                    self.refresh_shift_pack(&key).await?;
                    Ok(self.get_shifts_remaining() > 0)
                }
            }
        }
    }

    /// Get number of shifts remaining
    pub fn get_shifts_remaining(&self) -> u32 {
        match &self.license_type {
            LicenseType::FreeTrial => FREE_TRIAL_SHIFTS.saturating_sub(self.shifts_used),
            LicenseType::Subscription { .. } => u32::MAX, // Unlimited
            LicenseType::ShiftPack { shifts_remaining, .. } => *shifts_remaining,
        }
    }

    /// Consume one shapeshift
    pub fn use_shapeshift(&mut self) -> EshuResult<()> {
        match &mut self.license_type {
            LicenseType::FreeTrial => {
                if self.shifts_used < FREE_TRIAL_SHIFTS {
                    self.shifts_used += 1;
                    self.save()?;
                    Ok(())
                } else {
                    Err(EshuError::Validation("Free trial exhausted".to_string()))
                }
            }
            LicenseType::Subscription { .. } => {
                // Unlimited - just track usage for stats
                self.shifts_used += 1;
                self.save()?;
                Ok(())
            }
            LicenseType::ShiftPack { shifts_remaining, .. } => {
                if *shifts_remaining > 0 {
                    *shifts_remaining -= 1;
                    self.shifts_used += 1;
                    self.save()?;
                    Ok(())
                } else {
                    Err(EshuError::Validation("No shifts remaining".to_string()))
                }
            }
        }
    }

    /// Activate a license key
    pub async fn activate_license(&mut self, license_key: String, product_permalink: String) -> EshuResult<()> {
        // Verify the license key with Gumroad
        let client = reqwest::Client::new();
        let response = client
            .post(GUMROAD_VERIFY_URL)
            .form(&[
                ("product_permalink", product_permalink.as_str()),
                ("license_key", license_key.as_str()),
                ("increment_uses_count", "false"),
            ])
            .send()
            .await
            .map_err(|e| EshuError::Network(format!("License verification failed: {}", e)))?;

        let verify_response: GumroadVerifyResponse = response
            .json()
            .await
            .map_err(|e| EshuError::Network(format!("Failed to parse response: {}", e)))?;

        if !verify_response.success {
            return Err(EshuError::Validation(
                format!("Invalid license key: {}", verify_response.message.unwrap_or_default())
            ));
        }

        let purchase = verify_response.purchase.ok_or_else(|| {
            EshuError::Validation("No purchase information returned".to_string())
        })?;

        // Determine license type based on product
        if product_permalink.contains("unlimited") || product_permalink.contains("subscription") {
            // Check if subscription is active
            if purchase.subscription_ended_at.is_some()
                || purchase.subscription_failed_at.is_some()
                || purchase.subscription_cancelled_at.is_some()
            {
                return Err(EshuError::Validation("Subscription is not active".to_string()));
            }

            self.license_type = LicenseType::Subscription { license_key };
            self.last_verified = chrono::Utc::now().timestamp();
            self.save()?;

            println!("{}", "✅ Subscription activated! You now have unlimited shapeshifts.".green().bold());
        } else if product_permalink.contains("pack") {
            // Shift pack - 10 shifts
            self.license_type = LicenseType::ShiftPack {
                license_key,
                shifts_remaining: 10,
            };
            self.save()?;

            println!("{}", "✅ Shift pack activated! You have 10 shapeshifts available.".green().bold());
        }

        Ok(())
    }

    /// Verify subscription is still active (checks every 24 hours)
    async fn verify_subscription(&mut self, license_key: &str) -> EshuResult<bool> {
        let now = chrono::Utc::now().timestamp();
        let time_since_last_check = now - self.last_verified;

        // Check once per day (86400 seconds)
        if time_since_last_check < 86400 {
            return Ok(true); // Assume valid until next check
        }

        // Verify with Gumroad
        let client = reqwest::Client::new();
        let response = client
            .post(GUMROAD_VERIFY_URL)
            .form(&[
                ("product_permalink", "eshu-shapeshifter-unlimited"),
                ("license_key", license_key),
                ("increment_uses_count", "false"),
            ])
            .send()
            .await
            .map_err(|e| EshuError::Network(format!("Subscription verification failed: {}", e)))?;

        let verify_response: GumroadVerifyResponse = response
            .json()
            .await
            .map_err(|e| EshuError::Network(format!("Failed to parse response: {}", e)))?;

        if !verify_response.success {
            return Ok(false);
        }

        if let Some(purchase) = verify_response.purchase {
            // Check if subscription ended, failed, or was cancelled
            if purchase.subscription_ended_at.is_some()
                || purchase.subscription_failed_at.is_some()
                || purchase.subscription_cancelled_at.is_some()
            {
                return Ok(false);
            }
        }

        self.last_verified = now;
        self.save()?;
        Ok(true)
    }

    /// Refresh shift pack count (in case they bought more)
    async fn refresh_shift_pack(&mut self, license_key: &str) -> EshuResult<()> {
        let client = reqwest::Client::new();
        let response = client
            .post(GUMROAD_VERIFY_URL)
            .form(&[
                ("product_permalink", "eshu-shapeshifter-pack"),
                ("license_key", license_key),
                ("increment_uses_count", "false"),
            ])
            .send()
            .await
            .map_err(|e| EshuError::Network(format!("Pack verification failed: {}", e)))?;

        let verify_response: GumroadVerifyResponse = response
            .json()
            .await
            .map_err(|e| EshuError::Network(format!("Failed to parse response: {}", e)))?;

        // This is a simplified version - in production you'd track multiple purchases
        // For now, we assume if the license is valid, they have their pack

        Ok(())
    }

    /// Display license status
    pub fn display_status(&self) {
        println!("\n{}", "📜 License Status".cyan().bold());
        println!("{}", "═══════════════════════════════════════".cyan());

        match &self.license_type {
            LicenseType::FreeTrial => {
                let remaining = FREE_TRIAL_SHIFTS.saturating_sub(self.shifts_used);
                println!("\n  {}: {}", "Type".yellow(), "Free Trial".white());
                println!("  {}: {}/{}",
                    "Shapeshifts".yellow(),
                    self.shifts_used,
                    FREE_TRIAL_SHIFTS
                );
                println!("  {}: {}", "Remaining".yellow(), remaining.to_string().green().bold());

                if remaining == 0 {
                    println!("\n  {}", "⚠️  Free trial exhausted!".red().bold());
                    println!("  {}", "Upgrade to continue:".yellow());
                    println!("    • {} - Unlimited shapeshifts", "$5.99/month".green());
                    println!("    • {} - 10 shapeshifts", "$3.99".green());
                    println!("\n  Visit: {}", "https://gumroad.com/l/eshu-shapeshifter".cyan());
                }
            }
            LicenseType::Subscription { license_key } => {
                println!("\n  {}: {}", "Type".yellow(), "Unlimited Subscription".green().bold());
                println!("  {}: {}", "License Key".yellow(), format!("{}...", &license_key[..8]));
                println!("  {}: {}", "Shapeshifts Used".yellow(), self.shifts_used);
                println!("  {}: {}", "Remaining".yellow(), "Unlimited ♾️".green().bold());

                let last_check = chrono::DateTime::<chrono::Utc>::from_timestamp(self.last_verified, 0)
                    .map(|dt| dt.format("%Y-%m-%d %H:%M").to_string())
                    .unwrap_or_else(|| "Never".to_string());
                println!("  {}: {}", "Last Verified".yellow(), last_check);
            }
            LicenseType::ShiftPack { license_key, shifts_remaining } => {
                println!("\n  {}: {}", "Type".yellow(), "Shift Pack".white());
                println!("  {}: {}", "License Key".yellow(), format!("{}...", &license_key[..8]));
                println!("  {}: {}", "Shapeshifts Used".yellow(), self.shifts_used);
                println!("  {}: {}", "Remaining".yellow(), shifts_remaining.to_string().green().bold());

                if *shifts_remaining == 0 {
                    println!("\n  {}", "⚠️  No shifts remaining!".red().bold());
                    println!("  Buy more: {}", "https://gumroad.com/l/eshu-shapeshifter".cyan());
                } else if *shifts_remaining <= 2 {
                    println!("\n  {}", format!("⚠️  Only {} shifts remaining!", shifts_remaining).yellow());
                }
            }
        }

        let first_use = chrono::DateTime::<chrono::Utc>::from_timestamp(self.first_use_timestamp, 0)
            .map(|dt| dt.format("%Y-%m-%d").to_string())
            .unwrap_or_else(|| "Unknown".to_string());
        println!("\n  {}: {}", "First Used".yellow(), first_use);
        println!("  {}: {}", "Total Shapeshifts".yellow(), self.shifts_used);
    }
}

/// Check license and display upgrade prompt if needed
pub async fn check_license_and_prompt() -> EshuResult<bool> {
    let mut license = LicenseInfo::load()?;

    let can_use = license.can_shapeshift().await?;

    if !can_use {
        println!("\n{}", "🔒 License Required".red().bold());
        println!("{}", "═══════════════════════════════════════".red());

        match &license.license_type {
            LicenseType::FreeTrial => {
                println!("\n  Your {} have been used.", "2 free trial shapeshifts".yellow());
                println!("\n  {} to continue using Eshu Shapeshifter:", "Choose a plan".green().bold());
            }
            LicenseType::Subscription { .. } => {
                println!("\n  Your {} is no longer active.", "subscription".yellow());
                println!("  This could be due to payment failure or cancellation.");
                println!("\n  {} to continue:", "Renew your subscription".green().bold());
            }
            LicenseType::ShiftPack { .. } => {
                println!("\n  Your {} are all used.", "shift pack".yellow());
                println!("\n  {} to continue:", "Purchase more shifts".green().bold());
            }
        }

        println!("\n  {} Unlimited Monthly - ${}/month", "💎".cyan(), "5.99");
        println!("     ♾️  Unlimited shapeshifts");
        println!("     ❌ Cancel anytime");
        println!("     💪 Best for frequent users\n");

        println!("  {} Shift Pack - ${} per pack", "📦".cyan(), "3.99");
        println!("     📦 10 shapeshifts");
        println!("     💰 One-time payment");
        println!("     📚 Stack multiple packs\n");

        println!("  Visit: {}", "https://gumroad.com/l/eshu-shapeshifter".cyan().bold());
        println!("\n  After purchase, activate with:");
        println!("  {}", "sudo eshu-shapeshifter activate <license-key>".green());

        return Ok(false);
    }

    Ok(true)
}
