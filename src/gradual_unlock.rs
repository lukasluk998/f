/// Gradual Feature Unlock System
/// 
/// This is the ULTIMATE behavioral safety feature.
/// 
/// Instead of enabling all cheat features immediately (suspicious spike in performance),
/// this system gradually unlocks features over 12+ days to mimic natural player improvement.
/// 
/// Why this is critical:
/// - New player with cheat: 0.5 K/D → 5.0 K/D overnight = OBVIOUS CHEAT
/// - Natural player: 0.5 K/D → slowly improves to 2.5 K/D over weeks = NORMAL
/// 
/// This makes the cheat virtually undetectable from behavioral analysis.

use std::fs;
use std::io::{Read, Write};
use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH, Duration};
use serde::{Deserialize, Serialize};

/// Feature unlock schedule configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnlockSchedule {
    /// Days until ESP is fully enabled
    pub esp_full_days: u32,
    
    /// Days until recoil helper is enabled
    pub recoil_helper_days: u32,
    
    /// Days until full recoil compensation
    pub recoil_full_days: u32,
    
    /// Days until aimbot enabled (if supported)
    pub aimbot_days: u32,
    
    /// Days until all optimizations enabled
    pub optimizations_days: u32,
}

impl Default for UnlockSchedule {
    fn default() -> Self {
        UnlockSchedule {
            esp_full_days: 3,          // ESP basic → full in 3 days
            recoil_helper_days: 5,     // Recoil helper after 5 days
            recoil_full_days: 12,      // Full recoil comp after 12 days
            aimbot_days: 14,           // Aimbot after 2 weeks (if ever)
            optimizations_days: 1,     // Optimizations after 1 day
        }
    }
}

/// Persistent state tracking first run time
#[derive(Debug, Clone, Serialize, Deserialize)]
struct UnlockState {
    /// Unix timestamp of first run
    first_run_timestamp: u64,
    
    /// Total runs (for statistics)
    total_runs: u32,
    
    /// Last run timestamp
    last_run_timestamp: u64,
}

pub struct GradualUnlock {
    state: UnlockState,
    schedule: UnlockSchedule,
    state_file: PathBuf,
}

impl GradualUnlock {
    /// Create new gradual unlock system
    /// 
    /// State is persisted in a hidden file to track usage across sessions
    pub fn new(schedule: UnlockSchedule) -> Self {
        let state_file = Self::get_state_file_path();
        
        let state = if state_file.exists() {
            Self::load_state(&state_file).unwrap_or_else(|| Self::create_new_state())
        } else {
            let new_state = Self::create_new_state();
            let _ = Self::save_state(&state_file, &new_state);
            new_state
        };
        
        GradualUnlock {
            state,
            schedule,
            state_file,
        }
    }
    
    pub fn with_defaults() -> Self {
        Self::new(UnlockSchedule::default())
    }
    
    /// Get state file path (hidden in system temp or appdata)
    fn get_state_file_path() -> PathBuf {
        // Use Windows temp directory with obfuscated name
        let temp = std::env::temp_dir();
        
        // Obfuscated filename (looks like Windows system file)
        temp.join(".winsvc_1a3f2b.dat")
    }
    
    /// Create new state for first run
    fn create_new_state() -> UnlockState {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
        
        UnlockState {
            first_run_timestamp: now,
            total_runs: 1,
            last_run_timestamp: now,
        }
    }
    
    /// Load state from file
    fn load_state(path: &Path) -> Option<UnlockState> {
        let mut file = fs::File::open(path).ok()?;
        let mut contents = Vec::new();
        file.read_to_end(&mut contents).ok()?;
        
        // Decrypt state (simple XOR)
        let decrypted: Vec<u8> = contents.iter().map(|&b| b ^ 0x42).collect();
        
        serde_json::from_slice(&decrypted).ok()
    }
    
    /// Save state to file
    fn save_state(path: &Path, state: &UnlockState) -> std::io::Result<()> {
        let json = serde_json::to_vec(state)?;
        
        // Encrypt state (simple XOR)
        let encrypted: Vec<u8> = json.iter().map(|&b| b ^ 0x42).collect();
        
        let mut file = fs::File::create(path)?;
        file.write_all(&encrypted)?;
        
        Ok(())
    }
    
    /// Update state on new run
    pub fn register_run(&mut self) {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
        
        self.state.total_runs += 1;
        self.state.last_run_timestamp = now;
        
        let _ = Self::save_state(&self.state_file, &self.state);
    }
    
    /// Get number of days since first run
    pub fn days_since_first_run(&self) -> u32 {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
        
        let seconds_elapsed = now.saturating_sub(self.state.first_run_timestamp);
        let days = seconds_elapsed / 86400; // 86400 seconds per day
        
        days as u32
    }
    
    /// Get hours since first run (for more granular control)
    pub fn hours_since_first_run(&self) -> u32 {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
        
        let seconds_elapsed = now.saturating_sub(self.state.first_run_timestamp);
        let hours = seconds_elapsed / 3600;
        
        hours as u32
    }
    
    /// Check if ESP is unlocked
    pub fn is_esp_unlocked(&self) -> bool {
        // ESP available immediately, but limited at first
        true
    }
    
    /// Get ESP distance multiplier (gradually increases)
    /// Day 0: 50m max
    /// Day 1: 100m max
    /// Day 2: 200m max
    /// Day 3+: 300m+ max
    pub fn esp_distance_multiplier(&self) -> f32 {
        let days = self.days_since_first_run();
        
        match days {
            0 => 0.17,      // 50m  (50/300)
            1 => 0.33,      // 100m (100/300)
            2 => 0.67,      // 200m (200/300)
            _ => 1.0,       // 300m+ full distance
        }
    }
    
    /// Check if ESP shows full info (or minimal info)
    pub fn esp_show_full_info(&self) -> bool {
        self.days_since_first_run() >= self.schedule.esp_full_days
    }
    
    /// Check if recoil helper is unlocked
    pub fn is_recoil_helper_unlocked(&self) -> bool {
        self.days_since_first_run() >= self.schedule.recoil_helper_days
    }
    
    /// Get recoil compensation strength multiplier
    /// Gradually increases from 0% to 100% over 12 days
    pub fn recoil_strength_multiplier(&self) -> f32 {
        let days = self.days_since_first_run();
        
        if days < self.schedule.recoil_helper_days {
            return 0.0; // Not unlocked yet
        }
        
        let days_since_unlock = days - self.schedule.recoil_helper_days;
        let days_to_full = self.schedule.recoil_full_days - self.schedule.recoil_helper_days;
        
        if days_since_unlock >= days_to_full {
            return 1.0; // Fully unlocked
        }
        
        // Gradual increase
        // Day 5: 20%
        // Day 7: 40%
        // Day 9: 60%
        // Day 11: 80%
        // Day 12+: 100%
        let progress = days_since_unlock as f32 / days_to_full as f32;
        progress.min(1.0)
    }
    
    /// Check if aimbot is unlocked (for future features)
    pub fn is_aimbot_unlocked(&self) -> bool {
        self.days_since_first_run() >= self.schedule.aimbot_days
    }
    
    /// Get aimbot FOV multiplier (gradually increases)
    pub fn aimbot_fov_multiplier(&self) -> f32 {
        if !self.is_aimbot_unlocked() {
            return 0.0;
        }
        
        let days = self.days_since_first_run() - self.schedule.aimbot_days;
        
        // Start with very small FOV (5°) and gradually increase to 20° over a week
        match days {
            0..=1 => 0.25,   // 5° FOV
            2..=3 => 0.50,   // 10° FOV
            4..=5 => 0.75,   // 15° FOV
            _ => 1.0,        // 20° FOV
        }
    }
    
    /// Check if all optimizations are unlocked
    pub fn are_optimizations_unlocked(&self) -> bool {
        self.days_since_first_run() >= self.schedule.optimizations_days
    }
    
    /// Get current unlock status as human-readable string
    pub fn get_status_summary(&self) -> String {
        let days = self.days_since_first_run();
        let hours = self.hours_since_first_run();
        
        let mut status = String::new();
        status.push_str(&format!("Days since first run: {} ({} hours)\n", days, hours));
        status.push_str(&format!("Total runs: {}\n", self.state.total_runs));
        status.push_str("\nFeature Unlock Status:\n");
        
        // ESP
        if self.is_esp_unlocked() {
            let dist_mult = self.esp_distance_multiplier();
            let max_dist = 300.0 * dist_mult;
            status.push_str(&format!("  ESP: ✓ UNLOCKED (max distance: {:.0}m / 300m)\n", max_dist));
            
            if self.esp_show_full_info() {
                status.push_str("       ✓ Full info (health, distance, name)\n");
            } else {
                status.push_str("       ○ Basic info only (distance)\n");
            }
        } else {
            status.push_str("  ESP: ✗ Locked\n");
        }
        
        // Recoil Helper
        if self.is_recoil_helper_unlocked() {
            let strength = self.recoil_strength_multiplier();
            status.push_str(&format!("  Recoil Helper: ✓ UNLOCKED ({:.0}% compensation)\n", strength * 100.0));
        } else {
            let days_left = self.schedule.recoil_helper_days.saturating_sub(days);
            status.push_str(&format!("  Recoil Helper: ○ Unlocks in {} days\n", days_left));
        }
        
        // Optimizations
        if self.are_optimizations_unlocked() {
            status.push_str("  Optimizations: ✓ UNLOCKED\n");
        } else {
            let days_left = self.schedule.optimizations_days.saturating_sub(days);
            status.push_str(&format!("  Optimizations: ○ Unlocks in {} days\n", days_left));
        }
        
        status
    }
    
    /// Print detailed unlock report
    pub fn print_unlock_report(&self) {
        println!("\n╔════════════════════════════════════════════╗");
        println!("║     GRADUAL UNLOCK STATUS                  ║");
        println!("╚════════════════════════════════════════════╝");
        println!();
        println!("{}", self.get_status_summary());
        println!("This gradual unlock system mimics natural player improvement");
        println!("to avoid behavioral detection. Be patient for maximum safety!");
        println!();
    }
    
    /// Reset state (for testing or new install)
    pub fn reset(&mut self) {
        self.state = Self::create_new_state();
        let _ = Self::save_state(&self.state_file, &self.state);
        
        println!("[!] Gradual unlock state RESET");
        println!("[!] Starting fresh - all features locked");
    }
    
    /// Skip to specific day (for testing ONLY - never use in production!)
    #[cfg(debug_assertions)]
    pub fn debug_skip_to_day(&mut self, day: u32) {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
        
        // Set first run to N days ago
        self.state.first_run_timestamp = now - (day as u64 * 86400);
        let _ = Self::save_state(&self.state_file, &self.state);
        
        println!("[DEBUG] Skipped to day {}", day);
    }
}

/// Helper function to apply gradual unlock to config values
pub fn apply_unlock_limits(
    unlock: &GradualUnlock,
    base_value: f32,
    feature_type: FeatureType,
) -> f32 {
    match feature_type {
        FeatureType::EspDistance => base_value * unlock.esp_distance_multiplier(),
        FeatureType::RecoilCompensation => base_value * unlock.recoil_strength_multiplier(),
        FeatureType::AimbotFov => base_value * unlock.aimbot_fov_multiplier(),
    }
}

#[derive(Debug, Clone, Copy)]
pub enum FeatureType {
    EspDistance,
    RecoilCompensation,
    AimbotFov,
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_gradual_unlock_creation() {
        let unlock = GradualUnlock::with_defaults();
        assert!(unlock.is_esp_unlocked());
    }
    
    #[test]
    fn test_esp_distance_progression() {
        let unlock = GradualUnlock::with_defaults();
        
        // Day 0 should be limited
        assert!(unlock.esp_distance_multiplier() < 0.5);
    }
    
    #[test]
    fn test_recoil_locked_initially() {
        let unlock = GradualUnlock::with_defaults();
        
        // Recoil should be locked on day 0
        assert!(!unlock.is_recoil_helper_unlocked());
        assert_eq!(unlock.recoil_strength_multiplier(), 0.0);
    }
}
