// Configuration for cheat safety modes
// v3.4: Enhanced with commercial features, presets, and licensing stub
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum SafetyMode {
    /// Maximum safety - ESP only, no memory writes
    /// Detection risk: LOW
    /// Expected survival: 1-3+ months
    Legit,
    
    /// Balanced - ESP + driver-based no recoil
    /// Detection risk: MEDIUM
    /// Expected survival: 1-2 weeks
    Rage,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum RecoilMethod {
    /// No recoil control (safest)
    None,
    
    /// Logitech/Razer macro (undetectable)
    Macro,
    
    /// MAKCU hardware device (undetectable)
    Hardware,
    
    /// Kernel driver memory writes (detectable)
    MemoryPatch,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CheatConfig {
    // Safety mode
    pub mode: SafetyMode,
    
    // Features
    pub esp_enabled: bool,
    pub no_recoil_method: RecoilMethod,
    pub aimbot_enabled: bool,
    pub item_esp_enabled: bool,
    
    // ESP Optimization (v3.2)
    pub max_esp_distance: f32,
    pub esp_distance_lod: bool,
    pub esp_fov_culling: bool,
    pub esp_fov_angle: f32,
    pub esp_caching: bool,
    
    // Advanced Safety Features (v3.3)
    pub external_overlay_enabled: bool,     // Use external window overlay (SAFER)
    pub randomized_reads: bool,             // Randomize read patterns (SAFER)
    pub memory_batching: bool,              // Batch reads for efficiency (SAFER)
    pub screenshot_protection: bool,        // Hide overlay during screenshots (SAFER)
    pub screenshot_detection_strategy: String, // "None", "Basic", "Advanced", "Paranoid"
    
    // Recoil Helper (v3.2 - Read-only)
    pub recoil_helper_enabled: bool,
    pub recoil_compensation_strength: f32,
    pub recoil_show_weapon_info: bool,
    pub recoil_show_pattern: bool,
    pub recoil_crosshair_color: u32,
    
    // Driver
    pub use_kernel_driver: bool,
    pub driver_path: String,
    
    // Humanization (makes behavior look natural)
    pub humanization_enabled: bool,
    pub random_delays: bool,
    pub miss_shot_chance: f32,              // 0.0 - 1.0 (0.15 = miss 15%)
    pub reaction_delay_ms: (u64, u64),      // (min, max) reaction time
    pub esp_update_interval_ms: (u64, u64), // (min, max) ESP update rate
    
    // Gameplay limits
    pub max_kd_ratio: f32,           // Stop being obvious above this
    pub intentional_deaths: bool,     // Die sometimes on purpose
    pub check_corners_naturally: bool, // Look around like real player
    pub max_session_hours: u32,       // Auto-exit after X hours
    
    // Server preferences
    pub avoid_official_servers: bool,
    pub prefer_community_servers: bool,
    pub max_server_population: u32,
}

impl Default for CheatConfig {
    fn default() -> Self {
        Self::legit_mode()
    }
}

impl CheatConfig {
    /// LEGIT MODE - Maximum safety, minimal features
    /// Best for main accounts, long-term usage
    pub fn legit_mode() -> Self {
        Self {
            mode: SafetyMode::Legit,
            
            // Features - Read-only ESP only
            esp_enabled: true,
            no_recoil_method: RecoilMethod::Macro,  // Use Logitech macro
            aimbot_enabled: false,                  // Too obvious
            item_esp_enabled: false,                // Too obvious
            
            // ESP Optimization (v3.2)
            max_esp_distance: 300.0,
            esp_distance_lod: true,
            esp_fov_culling: true,
            esp_fov_angle: 90.0,
            esp_caching: true,
            
            // Advanced Safety Features (v3.3)
            external_overlay_enabled: true,      // External overlay = SAFER
            randomized_reads: true,              // Randomized patterns = SAFER
            memory_batching: true,               // Batch reads = SAFER + FASTER
            screenshot_protection: true,         // Hide during screenshots = SAFER
            screenshot_detection_strategy: "Basic".to_string(),
            
            // Recoil Helper (v3.2)
            recoil_helper_enabled: true,
            recoil_compensation_strength: 0.8,
            recoil_show_weapon_info: true,
            recoil_show_pattern: false,
            recoil_crosshair_color: 0x00FF00,
            
            // Driver - Not needed for read-only
            use_kernel_driver: false,
            driver_path: String::new(),
            
            // Humanization - Max humanization
            humanization_enabled: true,
            random_delays: true,
            miss_shot_chance: 0.15,            // Miss 15% of shots
            reaction_delay_ms: (200, 400),     // 200-400ms reaction (human-like)
            esp_update_interval_ms: (300, 500), // Slower updates
            
            // Gameplay limits
            max_kd_ratio: 3.0,                 // Stay reasonable
            intentional_deaths: true,           // Die sometimes
            check_corners_naturally: true,      // Look natural
            max_session_hours: 4,               // Limit session length
            
            // Server preferences
            avoid_official_servers: true,
            prefer_community_servers: true,
            max_server_population: 100,
        }
    }
    
    /// RAGE MODE - Full features, higher detection risk
    /// Best for alt accounts, short-term fun
    pub fn rage_mode() -> Self {
        Self {
            mode: SafetyMode::Rage,
            
            // Features - Everything enabled
            esp_enabled: true,
            no_recoil_method: RecoilMethod::MemoryPatch,
            aimbot_enabled: false,  // Still disabled (too obvious)
            item_esp_enabled: true,
            
            // ESP Optimization (v3.2) - Less strict
            max_esp_distance: 500.0,
            esp_distance_lod: false,  // Show full detail always
            esp_fov_culling: false,   // Show all players
            esp_fov_angle: 180.0,
            esp_caching: true,
            
            // Advanced Safety Features (v3.3) - Still use them for safety
            external_overlay_enabled: true,      // Still use external overlay
            randomized_reads: true,              // Still randomize
            memory_batching: true,               // Still batch
            screenshot_protection: true,         // Still protect
            screenshot_detection_strategy: "Paranoid".to_string(), // Extra paranoid
            
            // Recoil Helper (v3.2) - Not needed with memory patch
            recoil_helper_enabled: false,
            recoil_compensation_strength: 1.0,
            recoil_show_weapon_info: false,
            recoil_show_pattern: false,
            recoil_crosshair_color: 0xFF0000,
            
            // Driver - Required for memory writes
            use_kernel_driver: true,
            driver_path: "C:\\Windows\\System32\\drivers\\RustDriver.sys".to_string(),
            
            // Humanization - Minimal
            humanization_enabled: true,
            random_delays: true,
            miss_shot_chance: 0.05,            // Still miss some
            reaction_delay_ms: (100, 200),     // Faster reactions
            esp_update_interval_ms: (100, 200), // Faster updates
            
            // Gameplay limits - Less restrictive
            max_kd_ratio: 5.0,
            intentional_deaths: false,
            check_corners_naturally: false,
            max_session_hours: 8,
            
            // Server preferences
            avoid_official_servers: true,  // Still avoid official
            prefer_community_servers: false,
            max_server_population: 200,
        }
    }
    

    /// Load config from file, or use default
    pub fn load() -> Self {
        match std::fs::read_to_string("config.toml") {
            Ok(contents) => {
                toml::from_str(&contents).unwrap_or_else(|_| Self::default())
            },
            Err(_) => Self::default(),
        }
    }
    
    /// Save config to file
    pub fn save(&self) -> std::io::Result<()> {
        let toml = toml::to_string_pretty(self)
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;
        std::fs::write("config.toml", toml)
    }
}

/// Helper for humanized behavior
pub struct Humanizer {
    config: CheatConfig,
}

impl Humanizer {
    pub fn new(config: CheatConfig) -> Self {
        Self { config }
    }
    
    /// Random delay within configured range
    pub fn random_delay(&self, base_ms: (u64, u64)) {
        if !self.config.random_delays {
            std::thread::sleep(std::time::Duration::from_millis(base_ms.0));
            return;
        }
        
        let (min, max) = base_ms;
        let delay = min + (rand::random::<u64>() % (max - min));
        std::thread::sleep(std::time::Duration::from_millis(delay));
    }
    
    /// Should we miss this shot? (humanization)
    pub fn should_miss_shot(&self) -> bool {
        if !self.config.humanization_enabled {
            return false;
        }
        rand::random::<f32>() < self.config.miss_shot_chance
    }
    
    /// Reaction delay (simulate human reaction time)
    pub fn reaction_delay(&self) {
        self.random_delay(self.config.reaction_delay_ms);
    }
    
    /// ESP update delay (don't update too frequently)
    pub fn esp_update_delay(&self) {
        self.random_delay(self.config.esp_update_interval_ms);
    }
    
    /// Should we skip this ESP frame? (looks more human)
    pub fn should_skip_esp_frame(&self) -> bool {
        if !self.config.humanization_enabled {
            return false;
        }
        rand::random::<f32>() < 0.1  // 10% skip rate
    }
}

// ============================================================================
// v3.4: COMMERCIAL FEATURES
// ============================================================================

/// License type for commercial distribution
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum LicenseType {
    /// Free/trial version (limited features)
    Free,
    
    /// Basic paid license ($30/month)
    Basic,
    
    /// Pro license ($50/month)
    Pro,
    
    /// Lifetime license ($200 one-time)
    Lifetime,
    
    /// Developer/beta tester license
    Developer,
}

/// License information (for commercial version)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct License {
    /// License key (would be validated against server in real impl)
    pub key: String,
    
    /// License type
    pub license_type: LicenseType,
    
    /// Expiration timestamp (Unix epoch)
    /// None = lifetime
    pub expires_at: Option<u64>,
    
    /// HWID binding (simple impl - real version would use proper HWID)
    pub hwid: String,
    
    /// Maximum concurrent sessions
    pub max_sessions: u32,
}

impl Default for License {
    fn default() -> Self {
        License {
            key: "FREE-TRIAL".to_string(),
            license_type: LicenseType::Free,
            expires_at: None,
            hwid: String::new(),
            max_sessions: 1,
        }
    }
}

impl License {
    /// Check if license is valid
    pub fn is_valid(&self) -> bool {
        // In commercial version, this would:
        // 1. Validate key format
        // 2. Check against license server
        // 3. Verify HWID match
        // 4. Check expiration
        
        if let Some(expires_at) = self.expires_at {
            let now = std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs();
            
            if now > expires_at {
                return false; // Expired
            }
        }
        
        true
    }
    
    /// Get remaining days
    pub fn days_remaining(&self) -> Option<u32> {
        if let Some(expires_at) = self.expires_at {
            let now = std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs();
            
            if expires_at > now {
                let seconds_left = expires_at - now;
                let days_left = seconds_left / 86400;
                return Some(days_left as u32);
            } else {
                return Some(0);
            }
        }
        
        None // Lifetime
    }
    
    /// Get feature access level
    pub fn can_use_feature(&self, feature: &str) -> bool {
        match self.license_type {
            LicenseType::Free => {
                // Free version: Basic ESP only
                matches!(feature, "esp_basic" | "optimizations")
            },
            LicenseType::Basic => {
                // Basic: ESP + Recoil Helper
                matches!(feature, 
                    "esp_basic" | "esp_advanced" | "recoil_helper" | 
                    "optimizations" | "screenshot_protection")
            },
            LicenseType::Pro | LicenseType::Lifetime | LicenseType::Developer => {
                // Pro/Lifetime: All features
                true
            },
        }
    }
}

/// Configuration preset system
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConfigPreset {
    pub name: String,
    pub description: String,
    pub config: CheatConfig,
}

/// Preset manager
pub struct PresetManager {
    presets: HashMap<String, ConfigPreset>,
}

impl PresetManager {
    pub fn new() -> Self {
        let mut manager = PresetManager {
            presets: HashMap::new(),
        };
        
        manager.load_default_presets();
        manager
    }
    
    /// Load default presets
    fn load_default_presets(&mut self) {
        // Preset 1: Ultra Safe (Main Account)
        self.presets.insert(
            "ultra_safe".to_string(),
            ConfigPreset {
                name: "Ultra Safe".to_string(),
                description: "Maximum safety for main account. 6-12+ months survival.".to_string(),
                config: CheatConfig::legit_mode(),
            },
        );
        
        // Preset 2: Balanced (Alt Account)
        let mut balanced = CheatConfig::legit_mode();
        balanced.max_esp_distance = 400.0;
        balanced.recoil_compensation_strength = 0.9;
        balanced.miss_shot_chance = 0.10;
        self.presets.insert(
            "balanced".to_string(),
            ConfigPreset {
                name: "Balanced".to_string(),
                description: "Good balance of features and safety. 3-6 months survival.".to_string(),
                config: balanced,
            },
        );
        
        // Preset 3: Rage (Burner Account)
        self.presets.insert(
            "rage".to_string(),
            ConfigPreset {
                name: "Rage Mode".to_string(),
                description: "Maximum features. Use on burner accounts only. 2-4 weeks survival.".to_string(),
                config: CheatConfig::rage_mode(),
            },
        );
        
        // Preset 4: Stealth Stream (for streaming)
        let mut stealth = CheatConfig::legit_mode();
        stealth.external_overlay_enabled = false; // No overlay visible
        stealth.recoil_helper_enabled = false;    // No visual indicators
        stealth.screenshot_detection_strategy = "Paranoid".to_string();
        self.presets.insert(
            "stealth_stream".to_string(),
            ConfigPreset {
                name: "Stealth Stream".to_string(),
                description: "For streaming without visual indicators. Console-only ESP.".to_string(),
                config: stealth,
            },
        );
    }
    
    /// Get preset by name
    pub fn get_preset(&self, name: &str) -> Option<&ConfigPreset> {
        self.presets.get(name)
    }
    
    /// List all presets
    pub fn list_presets(&self) -> Vec<&ConfigPreset> {
        self.presets.values().collect()
    }
}

/// Enhanced config with commercial features
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommercialConfig {
    /// Base cheat config
    #[serde(flatten)]
    pub cheat: CheatConfig,
    
    /// License information
    pub license: License,
    
    /// v3.4 Ultimate Safety Features
    pub behavioral_limiter_enabled: bool,
    pub anti_debug_enabled: bool,
    pub anti_debug_auto_exit: bool,
    pub gradual_unlock_enabled: bool,
    pub string_obfuscation_enabled: bool,
    
    /// Statistics tracking
    pub track_statistics: bool,
    pub upload_anonymous_stats: bool,
    
    /// Auto-update system (stub)
    pub auto_update_check: bool,
    pub update_channel: String, // "stable", "beta", "dev"
    
    /// Telemetry (for commercial version)
    pub telemetry_enabled: bool,
    pub crash_reports: bool,
}

impl Default for CommercialConfig {
    fn default() -> Self {
        CommercialConfig {
            cheat: CheatConfig::default(),
            license: License::default(),
            
            // v3.4 features - ALL ENABLED by default
            behavioral_limiter_enabled: true,
            anti_debug_enabled: true,
            anti_debug_auto_exit: true,
            gradual_unlock_enabled: true,
            string_obfuscation_enabled: true,
            
            // Statistics
            track_statistics: true,
            upload_anonymous_stats: false,
            
            // Updates
            auto_update_check: true,
            update_channel: "stable".to_string(),
            
            // Telemetry
            telemetry_enabled: false,
            crash_reports: false,
        }
    }
}

impl CommercialConfig {
    /// Load from file
    pub fn load() -> Self {
        match std::fs::read_to_string("config.toml") {
            Ok(contents) => {
                toml::from_str(&contents).unwrap_or_else(|_| Self::default())
            },
            Err(_) => Self::default(),
        }
    }
    
    /// Save to file
    pub fn save(&self) -> std::io::Result<()> {
        let toml = toml::to_string_pretty(self)
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;
        std::fs::write("config.toml", toml)
    }
    
    /// Validate license and check features
    pub fn validate(&self) -> Result<(), String> {
        if !self.license.is_valid() {
            return Err("License invalid or expired".to_string());
        }
        
        // Check feature permissions
        if self.cheat.aimbot_enabled && !self.license.can_use_feature("aimbot") {
            return Err("Aimbot requires Pro license or higher".to_string());
        }
        
        if self.cheat.item_esp_enabled && !self.license.can_use_feature("item_esp") {
            return Err("Item ESP requires Basic license or higher".to_string());
        }
        
        Ok(())
    }
    
    /// Print license info
    pub fn print_license_info(&self) {
        println!("\n╔════════════════════════════════════════════╗");
        println!("║          LICENSE INFORMATION               ║");
        println!("╚════════════════════════════════════════════╝");
        println!();
        println!("License Type: {:?}", self.license.license_type);
        println!("Key: {}", 
            if self.license.key.len() > 10 {
                format!("{}...{}", &self.license.key[..5], &self.license.key[self.license.key.len()-5..])
            } else {
                self.license.key.clone()
            }
        );
        
        match self.license.days_remaining() {
            Some(0) => println!("Status: ⚠️  EXPIRED"),
            Some(days) if days <= 7 => println!("Status: ⚠️  Expires in {} days", days),
            Some(days) => println!("Status: ✓ Valid ({} days remaining)", days),
            None => println!("Status: ✓ Lifetime"),
        }
        
        println!();
        println!("Available Features:");
        let features = [
            ("Basic ESP", "esp_basic"),
            ("Advanced ESP", "esp_advanced"),
            ("Recoil Helper", "recoil_helper"),
            ("Optimizations", "optimizations"),
            ("Screenshot Protection", "screenshot_protection"),
            ("Aimbot", "aimbot"),
            ("Item ESP", "item_esp"),
        ];
        
        for (name, feature) in &features {
            let available = self.license.can_use_feature(feature);
            println!("  {} {}", if available { "✓" } else { "✗" }, name);
        }
        
        println!();
    }
}

/// Version information
pub const VERSION: &str = env!("CARGO_PKG_VERSION");
pub const BUILD_DATE: &str = "2026-05-27"; // Build date
pub const GIT_HASH: &str = "v3.4.0"; // Version tag

pub fn print_version_info() {
    println!("╔══════════════════════════════════════════════╗");
    println!("║   Rust EAC Bypass Cheat v{}              ║", VERSION);
    println!("║   Commercial Edition                        ║");
    println!("╚══════════════════════════════════════════════╝");
    println!();
    println!("Version: {}", VERSION);
    println!("Build: {}", GIT_HASH);
    println!();
}
