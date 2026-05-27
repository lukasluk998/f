// Library exports for bin tools
pub mod memory;
pub mod scanner;
pub mod runtime_dumper;
pub mod esp_optimizer;
pub mod recoil_helper;
pub mod external_overlay;
pub mod randomized_patterns;
pub mod screenshot_detector;
pub mod offsets;

// Re-export commonly used types
pub use memory::Process;
pub use scanner::PatternScanner;
pub use runtime_dumper::{RuntimeDumper, DumpedOffsets};
pub use esp_optimizer::{ESPOptimizer, DetailLevel, CachedPlayer};
pub use recoil_helper::{RecoilHelper, WeaponRecoil};
pub use external_overlay::ExternalOverlay;
pub use randomized_patterns::RandomizedPatterns;
pub use screenshot_detector::{UnifiedScreenshotDetector, DetectionStrategy};
pub use offsets::{RustOffsets, Vec3, Vec2};

