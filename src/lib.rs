// Library exports for bin tools
pub mod memory;
pub mod scanner;
pub mod runtime_dumper;
pub mod config;
pub mod esp_optimizer;
pub mod recoil_helper;
pub mod external_overlay;
pub mod randomized_patterns;
pub mod screenshot_detector;
pub mod offsets;

// v3.4 Ultimate Safety modules
pub mod behavioral_limiter;
pub mod gradual_unlock;
pub mod obfuscation;
pub mod anti_debug;

// Re-export commonly used types
pub use memory::Process;
pub use scanner::PatternScanner;
pub use runtime_dumper::{RuntimeDumper, DumpedOffsets};
pub use config::{CheatConfig, CommercialConfig, SafetyMode, RecoilMethod, Humanizer};
pub use esp_optimizer::{ESPOptimizer, DetailLevel, CachedPlayer};
pub use recoil_helper::{RecoilHelper, WeaponRecoil};
pub use external_overlay::ExternalOverlay;
pub use randomized_patterns::RandomizedPatterns;
pub use screenshot_detector::{UnifiedScreenshotDetector, DetectionStrategy};
pub use offsets::{RustOffsets, Vec3, Vec2};

// v3.4 exports
pub use behavioral_limiter::{BehavioralLimiter, BehavioralLimits, SessionStats};
pub use gradual_unlock::{GradualUnlock, UnlockSchedule, FeatureType};
pub use obfuscation::StackString;
pub use anti_debug::{AntiDebug, anti_vm};

