// ESP Optimization - Distance-based LOD, FOV culling, frame skipping
// 100% SAFE - Read-only operations

use crate::offsets::{Vec3, Vec2};
use std::collections::HashMap;
use std::time::{Duration, Instant};

#[derive(Clone, Debug)]
pub struct CachedPlayer {
    pub address: usize,
    pub position: Vec3,
    pub health: f32,
    pub max_health: f32,
    pub distance: f32,
    pub last_update: Instant,
    pub visible: bool,
}

pub struct ESPOptimizer {
    // Cached player data (reduces memory reads)
    player_cache: HashMap<usize, CachedPlayer>,
    
    // Frame skipping counter
    frame_count: u64,
    
    // Performance settings
    max_render_distance: f32,
    fov_angle: f32,
    
    // Update rates based on distance
    close_update_rate: u32,   // Update every N frames (< 50m)
    medium_update_rate: u32,  // Update every N frames (50-150m)
    far_update_rate: u32,     // Update every N frames (150-300m)
}

impl ESPOptimizer {
    pub fn new() -> Self {
        Self {
            player_cache: HashMap::new(),
            frame_count: 0,
            max_render_distance: 300.0,
            fov_angle: 90.0,
            close_update_rate: 1,   // Every frame
            medium_update_rate: 2,  // Every 2 frames
            far_update_rate: 4,     // Every 4 frames
        }
    }
    
    /// Check if player should be rendered based on distance
    pub fn should_render_player(&self, distance: f32) -> bool {
        if distance > self.max_render_distance {
            return false;
        }
        
        // Distance-based frame skipping
        let update_rate = match distance {
            d if d < 50.0 => self.close_update_rate,
            d if d < 150.0 => self.medium_update_rate,
            d if d < 300.0 => self.far_update_rate,
            _ => return false,
        };
        
        self.frame_count % update_rate as u64 == 0
    }
    
    /// Check if position is in camera FOV (frustum culling)
    pub fn is_in_fov(&self, player_pos: Vec3, camera_pos: Vec3, camera_forward: Vec3) -> bool {
        let to_player = Vec3 {
            x: player_pos.x - camera_pos.x,
            y: player_pos.y - camera_pos.y,
            z: player_pos.z - camera_pos.z,
        };
        
        let to_player_normalized = self.normalize(to_player);
        let dot = self.dot_product(camera_forward, to_player_normalized);
        
        // Convert dot product to angle (cos(angle) = dot)
        // For 90° FOV: cos(45°) = 0.707
        let fov_threshold = (self.fov_angle / 2.0).to_radians().cos();
        
        dot >= fov_threshold
    }
    
    /// Get or update cached player data
    pub fn get_cached_player(&mut self, address: usize) -> Option<&CachedPlayer> {
        // Check if cache exists and is still valid
        if let Some(cached) = self.player_cache.get(&address) {
            if cached.last_update.elapsed() < Duration::from_millis(500) {
                return Some(cached);
            }
        }
        
        None
    }
    
    /// Update player cache
    pub fn update_cache(&mut self, address: usize, position: Vec3, health: f32, max_health: f32, distance: f32, visible: bool) {
        let cached = CachedPlayer {
            address,
            position,
            health,
            max_health,
            distance,
            last_update: Instant::now(),
            visible,
        };
        
        self.player_cache.insert(address, cached);
    }
    
    /// Clear stale cache entries (players that left)
    pub fn cleanup_cache(&mut self) {
        self.player_cache.retain(|_, player| {
            player.last_update.elapsed() < Duration::from_secs(5)
        });
    }
    
    /// Increment frame counter
    pub fn next_frame(&mut self) {
        self.frame_count = self.frame_count.wrapping_add(1);
        
        // Cleanup every 100 frames
        if self.frame_count % 100 == 0 {
            self.cleanup_cache();
        }
    }
    
    /// Get render priority (0 = highest, 3 = lowest)
    pub fn get_render_priority(&self, distance: f32) -> u8 {
        match distance {
            d if d < 50.0 => 0,    // Close: always render
            d if d < 150.0 => 1,   // Medium: render most frames
            d if d < 300.0 => 2,   // Far: render occasionally
            _ => 3,                // Very far: rarely render
        }
    }
    
    /// Calculate distance LOD (Level of Detail)
    pub fn get_detail_level(&self, distance: f32) -> DetailLevel {
        match distance {
            d if d < 50.0 => DetailLevel::High,
            d if d < 150.0 => DetailLevel::Medium,
            d if d < 300.0 => DetailLevel::Low,
            _ => DetailLevel::Minimal,
        }
    }
    
    /// Helper: Normalize vector
    fn normalize(&self, v: Vec3) -> Vec3 {
        let length = (v.x * v.x + v.y * v.y + v.z * v.z).sqrt();
        if length == 0.0 {
            return Vec3 { x: 0.0, y: 0.0, z: 0.0 };
        }
        Vec3 {
            x: v.x / length,
            y: v.y / length,
            z: v.z / length,
        }
    }
    
    /// Helper: Dot product
    fn dot_product(&self, a: Vec3, b: Vec3) -> f32 {
        a.x * b.x + a.y * b.y + a.z * b.z
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum DetailLevel {
    High,      // Full info: name, HP bar, distance, weapon
    Medium,    // Basic info: HP, distance
    Low,       // Distance only
    Minimal,   // Just a dot
}

impl DetailLevel {
    /// Get what info to display
    pub fn show_name(&self) -> bool {
        matches!(self, DetailLevel::High)
    }
    
    pub fn show_health_bar(&self) -> bool {
        matches!(self, DetailLevel::High | DetailLevel::Medium)
    }
    
    pub fn show_distance(&self) -> bool {
        !matches!(self, DetailLevel::Minimal)
    }
    
    pub fn show_weapon(&self) -> bool {
        matches!(self, DetailLevel::High)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_distance_rendering() {
        let optimizer = ESPOptimizer::new();
        
        // Close players should always render
        assert!(optimizer.should_render_player(30.0));
        
        // Very far players should not render
        assert!(!optimizer.should_render_player(400.0));
    }
    
    #[test]
    fn test_detail_levels() {
        let optimizer = ESPOptimizer::new();
        
        assert_eq!(optimizer.get_detail_level(30.0), DetailLevel::High);
        assert_eq!(optimizer.get_detail_level(100.0), DetailLevel::Medium);
        assert_eq!(optimizer.get_detail_level(250.0), DetailLevel::Low);
        assert_eq!(optimizer.get_detail_level(400.0), DetailLevel::Minimal);
    }
    
    #[test]
    fn test_fov_culling() {
        let optimizer = ESPOptimizer::new();
        
        let camera_pos = Vec3 { x: 0.0, y: 0.0, z: 0.0 };
        let camera_forward = Vec3 { x: 0.0, y: 0.0, z: 1.0 };
        
        // Player in front should be visible
        let player_front = Vec3 { x: 0.0, y: 0.0, z: 10.0 };
        assert!(optimizer.is_in_fov(player_front, camera_pos, camera_forward));
        
        // Player behind should not be visible
        let player_behind = Vec3 { x: 0.0, y: 0.0, z: -10.0 };
        assert!(!optimizer.is_in_fov(player_behind, camera_pos, camera_forward));
    }
}
