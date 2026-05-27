// Screenshot Detector - Detect when EAC takes screenshots
// 100% SAFE - Detection only, no blocking
// Hides overlay when screenshot detected to avoid visual detection

use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::time::{Duration, Instant};

pub struct ScreenshotDetector {
    screenshot_detected: Arc<AtomicBool>,
    last_detection: Instant,
    cooldown: Duration,
    
    // Detection methods
    check_gdi_calls: bool,
    check_bitblt: bool,
    check_printwindow: bool,
}

impl ScreenshotDetector {
    /// Create new screenshot detector
    pub fn new() -> Self {
        Self {
            screenshot_detected: Arc::new(AtomicBool::new(false)),
            last_detection: Instant::now(),
            cooldown: Duration::from_secs(2),
            check_gdi_calls: true,
            check_bitblt: true,
            check_printwindow: true,
        }
    }
    
    /// Check if screenshot is happening RIGHT NOW
    /// Returns true if screenshot detected
    pub fn is_screenshot_happening(&mut self) -> bool {
        // Cooldown period (don't spam detection)
        if self.last_detection.elapsed() < Duration::from_millis(100) {
            return self.screenshot_detected.load(Ordering::Relaxed);
        }
        
        let mut detected = false;
        
        // Method 1: Check for BitBlt calls (most common screenshot method)
        if self.check_bitblt {
            if self.detect_bitblt() {
                detected = true;
            }
        }
        
        // Method 2: Check for PrintWindow calls (another screenshot method)
        if self.check_printwindow {
            if self.detect_printwindow() {
                detected = true;
            }
        }
        
        // Method 3: Check for suspicious GDI operations
        if self.check_gdi_calls {
            if self.detect_suspicious_gdi() {
                detected = true;
            }
        }
        
        if detected {
            self.screenshot_detected.store(true, Ordering::Relaxed);
            self.last_detection = Instant::now();
            println!("[!] Screenshot detected - Hiding overlay");
        } else {
            // Clear flag after cooldown
            if self.last_detection.elapsed() >= self.cooldown {
                self.screenshot_detected.store(false, Ordering::Relaxed);
            }
        }
        
        detected
    }
    
    /// Detect BitBlt calls (common screenshot method)
    /// BitBlt copies pixels from one DC to another
    fn detect_bitblt(&self) -> bool {
        // In real implementation, you would:
        // 1. Hook or monitor BitBlt calls
        // 2. Check if source is game window
        // 3. Check if destination is memory DC or file
        
        // For now, this is a placeholder
        // You can implement actual detection by:
        // - Hooking GDI32.dll!BitBlt with MinHook
        // - Monitoring GetDC/CreateCompatibleDC patterns
        // - Checking for memory DC creation spikes
        
        false // Placeholder
    }
    
    /// Detect PrintWindow calls (another screenshot method)
    fn detect_printwindow(&self) -> bool {
        // PrintWindow is specifically designed to capture window content
        // EAC might use this instead of BitBlt
        
        // Similar to BitBlt detection:
        // - Hook USER32.dll!PrintWindow
        // - Check if target is game window
        
        false // Placeholder
    }
    
    /// Detect suspicious GDI operations (generic detection)
    fn detect_suspicious_gdi(&self) -> bool {
        // Look for patterns that indicate screenshot:
        // - Multiple CreateCompatibleDC calls in short time
        // - CreateCompatibleBitmap + SelectObject + BitBlt sequence
        // - GetDC followed by memory allocation
        
        false // Placeholder
    }
    
    /// Should overlay be hidden? (public API)
    pub fn should_hide_overlay(&self) -> bool {
        self.screenshot_detected.load(Ordering::Relaxed)
    }
    
    /// Get time since last detection
    pub fn time_since_detection(&self) -> Duration {
        self.last_detection.elapsed()
    }
    
    /// Force hide overlay for duration
    pub fn force_hide(&mut self, duration: Duration) {
        self.screenshot_detected.store(true, Ordering::Relaxed);
        self.last_detection = Instant::now();
        self.cooldown = duration;
    }
    
    /// Reset detector
    pub fn reset(&mut self) {
        self.screenshot_detected.store(false, Ordering::Relaxed);
        self.last_detection = Instant::now();
        self.cooldown = Duration::from_secs(2);
    }
}

impl Default for ScreenshotDetector {
    fn default() -> Self {
        Self::new()
    }
}

/// Advanced screenshot detection (using hooks)
/// This requires MinHook or similar hooking library
pub struct AdvancedScreenshotDetector {
    detector: ScreenshotDetector,
    hooks_installed: bool,
}

impl AdvancedScreenshotDetector {
    pub fn new() -> Self {
        Self {
            detector: ScreenshotDetector::new(),
            hooks_installed: false,
        }
    }
    
    /// Install hooks for GDI functions
    /// Requires MinHook library (not included in this basic version)
    pub fn install_hooks(&mut self) -> Result<(), ()> {
        // TODO: Implement with MinHook
        // 1. Hook GDI32.dll!BitBlt
        // 2. Hook GDI32.dll!StretchBlt
        // 3. Hook USER32.dll!PrintWindow
        // 4. Hook GDI32.dll!CreateCompatibleDC
        // 5. Set callbacks to detect screenshot patterns
        
        // For now, just return Ok (hooks not implemented)
        println!("[!] Advanced screenshot detection not implemented yet");
        println!("[!] Using basic detection only");
        
        self.hooks_installed = false;
        Ok(())
    }
    
    /// Check if screenshot happening (with hooks)
    pub fn is_screenshot_happening(&mut self) -> bool {
        self.detector.is_screenshot_happening()
    }
    
    /// Should hide overlay?
    pub fn should_hide_overlay(&self) -> bool {
        self.detector.should_hide_overlay()
    }
}

/// Simple screenshot detection based on timing heuristics
/// Detects screenshot by monitoring suspicious patterns
pub struct HeuristicScreenshotDetector {
    detector: ScreenshotDetector,
    
    // Heuristics
    last_gdi_call_time: Instant,
    gdi_call_count: u32,
    suspicious_pattern_detected: bool,
}

impl HeuristicScreenshotDetector {
    pub fn new() -> Self {
        Self {
            detector: ScreenshotDetector::new(),
            last_gdi_call_time: Instant::now(),
            gdi_call_count: 0,
            suspicious_pattern_detected: false,
        }
    }
    
    /// Update heuristics
    pub fn update(&mut self) {
        // Reset counter after 1 second
        if self.last_gdi_call_time.elapsed() > Duration::from_secs(1) {
            self.gdi_call_count = 0;
            self.suspicious_pattern_detected = false;
        }
        
        // Detect suspicious pattern: many GDI calls in short time
        if self.gdi_call_count > 10 {
            self.suspicious_pattern_detected = true;
            self.detector.force_hide(Duration::from_secs(3));
        }
    }
    
    /// Notify of GDI call (call this when you detect GDI activity)
    pub fn on_gdi_call(&mut self) {
        self.gdi_call_count += 1;
        self.last_gdi_call_time = Instant::now();
    }
    
    /// Check if should hide overlay
    pub fn should_hide_overlay(&mut self) -> bool {
        self.update();
        self.detector.should_hide_overlay() || self.suspicious_pattern_detected
    }
}

/// Screenshot detection strategy
#[derive(Debug, Clone, Copy)]
pub enum DetectionStrategy {
    /// No detection (always visible)
    None,
    
    /// Basic detection (no hooks, just heuristics)
    Basic,
    
    /// Advanced detection (requires hooks)
    Advanced,
    
    /// Paranoid mode (hide overlay periodically)
    Paranoid,
}

/// Unified screenshot detector with multiple strategies
pub struct UnifiedScreenshotDetector {
    strategy: DetectionStrategy,
    basic_detector: ScreenshotDetector,
    heuristic_detector: HeuristicScreenshotDetector,
    
    // Paranoid mode settings
    paranoid_hide_interval: Duration,
    paranoid_hide_duration: Duration,
    last_paranoid_hide: Instant,
}

impl UnifiedScreenshotDetector {
    pub fn new(strategy: DetectionStrategy) -> Self {
        Self {
            strategy,
            basic_detector: ScreenshotDetector::new(),
            heuristic_detector: HeuristicScreenshotDetector::new(),
            paranoid_hide_interval: Duration::from_secs(60), // Hide every 60 seconds
            paranoid_hide_duration: Duration::from_secs(2),   // Hide for 2 seconds
            last_paranoid_hide: Instant::now(),
        }
    }
    
    /// Check if should hide overlay (unified check)
    pub fn should_hide_overlay(&mut self) -> bool {
        match self.strategy {
            DetectionStrategy::None => false,
            
            DetectionStrategy::Basic => {
                self.basic_detector.is_screenshot_happening()
            }
            
            DetectionStrategy::Advanced => {
                // Use heuristic detection as fallback for advanced
                self.heuristic_detector.should_hide_overlay()
            }
            
            DetectionStrategy::Paranoid => {
                // Hide overlay periodically as precaution
                if self.last_paranoid_hide.elapsed() >= self.paranoid_hide_interval {
                    self.last_paranoid_hide = Instant::now();
                    true
                } else if self.last_paranoid_hide.elapsed() < self.paranoid_hide_duration {
                    true
                } else {
                    // Also use basic detection
                    self.basic_detector.is_screenshot_happening()
                }
            }
        }
    }
    
    /// Get current strategy
    pub fn get_strategy(&self) -> DetectionStrategy {
        self.strategy
    }
    
    /// Change strategy
    pub fn set_strategy(&mut self, strategy: DetectionStrategy) {
        self.strategy = strategy;
        println!("[*] Screenshot detection strategy: {:?}", strategy);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_screenshot_detector_creation() {
        let detector = ScreenshotDetector::new();
        assert!(!detector.should_hide_overlay());
    }
    
    #[test]
    fn test_force_hide() {
        let mut detector = ScreenshotDetector::new();
        
        detector.force_hide(Duration::from_millis(500));
        assert!(detector.should_hide_overlay());
        
        std::thread::sleep(Duration::from_millis(600));
        // Should still be hidden (cooldown not elapsed)
        assert!(detector.should_hide_overlay());
    }
    
    #[test]
    fn test_detection_strategies() {
        let mut detector_none = UnifiedScreenshotDetector::new(DetectionStrategy::None);
        assert!(!detector_none.should_hide_overlay());
        
        let mut detector_basic = UnifiedScreenshotDetector::new(DetectionStrategy::Basic);
        // Basic should not hide by default (no screenshot detected)
        assert!(!detector_basic.should_hide_overlay());
    }
}
