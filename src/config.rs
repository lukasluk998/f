// Configuration for cheat safety modes
use serde::{Deserialize, Serialize};

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
