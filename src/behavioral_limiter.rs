use std::time::{Duration, Instant};
use serde::{Deserialize, Serialize};

/// Behavioral Stats Limiter
/// 
/// This module tracks player statistics and ensures they stay within "natural" ranges
/// to avoid statistical detection by anti-cheat systems.
/// 
/// EAC and similar systems track:
/// - K/D ratio (kills per death)
/// - Headshot percentage
/// - Accuracy (hits per shots)
/// - Session length (bot detection)
/// - Win rate
/// 
/// Players with outlier statistics get flagged for manual review or automatic bans.

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BehavioralLimits {
    /// Maximum K/D ratio (e.g., 3.0 = good player, 10.0 = obvious cheat)
    pub max_kd_ratio: f32,
    
    /// Maximum headshot percentage (e.g., 0.35 = 35% headshots)
    pub max_headshot_percentage: f32,
    
    /// Maximum accuracy (e.g., 0.65 = 65% shots hit)
    pub max_accuracy: f32,
    
    /// Maximum session length in hours before forced break
    pub max_session_hours: f32,
    
    /// Emergency K/D threshold that forces intentional death
    pub emergency_kd_threshold: f32,
    
    /// Minimum deaths per hour to maintain natural ratio
    pub min_deaths_per_hour: f32,
}

impl Default for BehavioralLimits {
    fn default() -> Self {
        BehavioralLimits {
            max_kd_ratio: 3.5,              // Top 5% players: 3-4 K/D
            max_headshot_percentage: 0.40,   // Good players: 30-40% HS
            max_accuracy: 0.70,              // Good players: 60-70% accuracy
            max_session_hours: 4.0,          // 4 hours max per session
            emergency_kd_threshold: 5.0,     // Force death if K/D > 5.0
            min_deaths_per_hour: 3.0,        // At least 3 deaths/hour
        }
    }
}

#[derive(Debug)]
pub struct SessionStats {
    pub kills: u32,
    pub deaths: u32,
    pub headshots: u32,
    pub shots_fired: u32,
    pub shots_hit: u32,
}

impl Default for SessionStats {
    fn default() -> Self {
        SessionStats {
            kills: 0,
            deaths: 0,
            headshots: 0,
            shots_fired: 0,
            shots_hit: 0,
        }
    }
}

pub struct BehavioralLimiter {
    session_start: Instant,
    stats: SessionStats,
    limits: BehavioralLimits,
    
    // State flags
    play_worse_mode: bool,
    last_warning_time: Option<Instant>,
}

impl BehavioralLimiter {
    pub fn new(limits: BehavioralLimits) -> Self {
        BehavioralLimiter {
            session_start: Instant::now(),
            stats: SessionStats::default(),
            limits,
            play_worse_mode: false,
            last_warning_time: None,
        }
    }
    
    pub fn with_defaults() -> Self {
        Self::new(BehavioralLimits::default())
    }
    
    /// Register a kill
    pub fn register_kill(&mut self, was_headshot: bool) {
        self.stats.kills += 1;
        if was_headshot {
            self.stats.headshots += 1;
        }
        
        // Check if we need to trigger "play worse" mode
        self.update_play_mode();
    }
    
    /// Register a death
    pub fn register_death(&mut self) {
        self.stats.deaths += 1;
        
        // Death resets some pressure
        if self.play_worse_mode {
            if self.get_current_kd() < self.limits.max_kd_ratio {
                self.play_worse_mode = false;
                println!("[Behavioral] Stats normalized - resuming normal play");
            }
        }
    }
    
    /// Register shots fired
    pub fn register_shots(&mut self, fired: u32, hit: u32) {
        self.stats.shots_fired += fired;
        self.stats.shots_hit += hit;
        
        self.update_play_mode();
    }
    
    /// Get current K/D ratio
    pub fn get_current_kd(&self) -> f32 {
        if self.stats.deaths == 0 {
            return self.stats.kills as f32;
        }
        self.stats.kills as f32 / self.stats.deaths as f32
    }
    
    /// Get current headshot percentage
    pub fn get_headshot_percentage(&self) -> f32 {
        if self.stats.kills == 0 {
            return 0.0;
        }
        self.stats.headshots as f32 / self.stats.kills as f32
    }
    
    /// Get current accuracy
    pub fn get_accuracy(&self) -> f32 {
        if self.stats.shots_fired == 0 {
            return 0.0;
        }
        self.stats.shots_hit as f32 / self.stats.shots_fired as f32
    }
    
    /// Get session duration in hours
    pub fn get_session_hours(&self) -> f32 {
        self.session_start.elapsed().as_secs_f32() / 3600.0
    }
    
    /// Get deaths per hour rate
    pub fn get_deaths_per_hour(&self) -> f32 {
        let hours = self.get_session_hours();
        if hours < 0.1 {
            return 0.0;
        }
        self.stats.deaths as f32 / hours
    }
    
    /// Update play mode based on current stats
    fn update_play_mode(&mut self) {
        let kd = self.get_current_kd();
        let hs_pct = self.get_headshot_percentage();
        let accuracy = self.get_accuracy();
        let deaths_per_hour = self.get_deaths_per_hour();
        
        // Check if any stat is out of bounds
        let was_worse_mode = self.play_worse_mode;
        
        self.play_worse_mode = 
            kd > self.limits.max_kd_ratio ||
            hs_pct > self.limits.max_headshot_percentage ||
            accuracy > self.limits.max_accuracy ||
            (self.get_session_hours() > 0.5 && deaths_per_hour < self.limits.min_deaths_per_hour);
        
        // Print warning when entering "play worse" mode
        if self.play_worse_mode && !was_worse_mode {
            let now = Instant::now();
            if self.last_warning_time.is_none() || 
               now.duration_since(self.last_warning_time.unwrap()) > Duration::from_secs(60) {
                println!("\n[Behavioral] ⚠️  STATS TOO HIGH - PLAY WORSE MODE ACTIVATED");
                println!("[Behavioral] Current stats:");
                println!("    K/D: {:.2} (max: {:.2})", kd, self.limits.max_kd_ratio);
                println!("    Headshots: {:.1}% (max: {:.1}%)", hs_pct * 100.0, self.limits.max_headshot_percentage * 100.0);
                println!("    Accuracy: {:.1}% (max: {:.1}%)", accuracy * 100.0, self.limits.max_accuracy * 100.0);
                println!("    Deaths/hour: {:.1} (min: {:.1})", deaths_per_hour, self.limits.min_deaths_per_hour);
                println!("[Behavioral] Reducing cheat effectiveness to normalize stats...\n");
                
                self.last_warning_time = Some(now);
            }
        }
    }
    
    /// Check if player should intentionally play worse
    /// Returns true if stats are too good
    pub fn should_play_worse(&self) -> bool {
        self.play_worse_mode
    }
    
    /// Check if player should be forced to die (emergency)
    /// This prevents extreme outlier stats
    pub fn should_force_death(&self) -> bool {
        let kd = self.get_current_kd();
        
        // Emergency threshold: K/D way too high
        if kd > self.limits.emergency_kd_threshold {
            println!("[Behavioral] ⚠️  EMERGENCY: K/D {:.2} too high! Need death ASAP", kd);
            return true;
        }
        
        // Not enough deaths per hour
        let session_hours = self.get_session_hours();
        if session_hours > 0.5 {
            let deaths_per_hour = self.get_deaths_per_hour();
            if deaths_per_hour < self.limits.min_deaths_per_hour * 0.5 {
                println!("[Behavioral] ⚠️  EMERGENCY: Only {:.1} deaths/hour! Need death", deaths_per_hour);
                return true;
            }
        }
        
        false
    }
    
    /// Check if session should be ended (too long)
    pub fn should_end_session(&self) -> bool {
        let hours = self.get_session_hours();
        
        if hours > self.limits.max_session_hours {
            println!("\n[Behavioral] ⚠️  SESSION TOO LONG ({:.1} hours)", hours);
            println!("[Behavioral] Take a break! Long sessions = bot detection");
            println!("[Behavioral] Recommended: Exit and restart in 30+ minutes\n");
            return true;
        }
        
        false
    }
    
    /// Get recommendation for reducing effectiveness
    /// Returns multiplier for cheat features (0.0 = disabled, 1.0 = full power)
    pub fn get_effectiveness_multiplier(&self) -> f32 {
        if !self.play_worse_mode {
            return 1.0; // Full power
        }
        
        // Calculate how far over limits we are
        let kd = self.get_current_kd();
        let kd_ratio = kd / self.limits.max_kd_ratio;
        
        let hs_pct = self.get_headshot_percentage();
        let hs_ratio = hs_pct / self.limits.max_headshot_percentage;
        
        let accuracy = self.get_accuracy();
        let acc_ratio = accuracy / self.limits.max_accuracy;
        
        // Find worst offender
        let max_ratio = kd_ratio.max(hs_ratio).max(acc_ratio);
        
        if max_ratio > 2.0 {
            return 0.3; // Severe reduction (70% less effective)
        } else if max_ratio > 1.5 {
            return 0.5; // Major reduction (50% less effective)
        } else if max_ratio > 1.2 {
            return 0.7; // Moderate reduction (30% less effective)
        } else {
            return 0.85; // Minor reduction (15% less effective)
        }
    }
    
    /// Get current session statistics summary
    pub fn get_stats_summary(&self) -> String {
        format!(
            "K/D: {:.2} | HS: {:.1}% | Acc: {:.1}% | Deaths/hr: {:.1} | Session: {:.1}h",
            self.get_current_kd(),
            self.get_headshot_percentage() * 100.0,
            self.get_accuracy() * 100.0,
            self.get_deaths_per_hour(),
            self.get_session_hours()
        )
    }
    
    /// Print detailed statistics report
    pub fn print_report(&self) {
        let kd = self.get_current_kd();
        let hs_pct = self.get_headshot_percentage();
        let accuracy = self.get_accuracy();
        let session_hours = self.get_session_hours();
        let deaths_per_hour = self.get_deaths_per_hour();
        
        println!("\n╔════════════════════════════════════════════╗");
        println!("║     BEHAVIORAL STATS REPORT                ║");
        println!("╚════════════════════════════════════════════╝");
        println!();
        println!("Session Statistics:");
        println!("  Kills:        {}", self.stats.kills);
        println!("  Deaths:       {}", self.stats.deaths);
        println!("  Headshots:    {}", self.stats.headshots);
        println!("  Shots Fired:  {}", self.stats.shots_fired);
        println!("  Shots Hit:    {}", self.stats.shots_hit);
        println!();
        println!("Calculated Metrics:");
        println!("  K/D Ratio:    {:.2} / {:.2} max", kd, self.limits.max_kd_ratio);
        println!("  Headshot %:   {:.1}% / {:.1}% max", hs_pct * 100.0, self.limits.max_headshot_percentage * 100.0);
        println!("  Accuracy:     {:.1}% / {:.1}% max", accuracy * 100.0, self.limits.max_accuracy * 100.0);
        println!("  Deaths/Hour:  {:.1} / {:.1} min", deaths_per_hour, self.limits.min_deaths_per_hour);
        println!("  Session Time: {:.1}h / {:.1}h max", session_hours, self.limits.max_session_hours);
        println!();
        
        if self.play_worse_mode {
            println!("Status: ⚠️  PLAY WORSE MODE ACTIVE");
            println!("  Effectiveness: {:.0}%", self.get_effectiveness_multiplier() * 100.0);
        } else {
            println!("Status: ✓ Stats within normal range");
        }
        
        if self.should_force_death() {
            println!("Warning: 🚨 EMERGENCY - NEED DEATH IMMEDIATELY");
        }
        
        if self.should_end_session() {
            println!("Warning: ⏰ SESSION TOO LONG - TAKE BREAK");
        }
        
        println!();
    }
    
    /// Reset statistics (for new session)
    pub fn reset_session(&mut self) {
        self.session_start = Instant::now();
        self.stats = SessionStats::default();
        self.play_worse_mode = false;
        self.last_warning_time = None;
        
        println!("[Behavioral] Session reset - fresh stats");
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_normal_play() {
        let mut limiter = BehavioralLimiter::with_defaults();
        
        // Simulate normal gameplay
        limiter.register_kill(false);
        limiter.register_death();
        limiter.register_kill(true);
        limiter.register_death();
        
        assert!(!limiter.should_play_worse());
        assert_eq!(limiter.get_current_kd(), 1.0);
    }
    
    #[test]
    fn test_high_kd_triggers_worse_mode() {
        let mut limiter = BehavioralLimiter::with_defaults();
        
        // Simulate too many kills without deaths
        for _ in 0..10 {
            limiter.register_kill(false);
        }
        limiter.register_death();
        
        assert!(limiter.should_play_worse());
        assert!(limiter.get_current_kd() > limiter.limits.max_kd_ratio);
    }
    
    #[test]
    fn test_emergency_death() {
        let mut limiter = BehavioralLimiter::with_defaults();
        
        // Simulate extreme K/D
        for _ in 0..20 {
            limiter.register_kill(false);
        }
        limiter.register_death();
        
        assert!(limiter.should_force_death());
    }
}
