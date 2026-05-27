// Read-Only Recoil Helper - Visual compensation guide
// 100% SAFE - No memory writes, only reads recoil pattern and displays visual guide

use crate::offsets::{Vec2, Vec3};
use crate::memory::Process;
use std::collections::HashMap;

/// Weapon recoil pattern data (read from game memory)
#[derive(Clone, Debug)]
pub struct WeaponRecoil {
    pub weapon_id: u32,
    pub weapon_name: String,
    pub recoil_pattern: Vec<Vec2>,  // X/Y offsets per shot
    pub fire_rate: f32,              // Rounds per minute
    pub current_shot: u32,           // Which shot in the pattern
    pub max_pattern_length: u32,     // Pattern repeats after this
}

pub struct RecoilHelper {
    // Known weapon patterns (read from memory or file)
    weapon_patterns: HashMap<u32, WeaponRecoil>,
    
    // Current weapon state
    current_weapon: Option<WeaponRecoil>,
    shot_count: u32,
    last_shot_time: std::time::Instant,
    
    // Visual settings
    compensation_strength: f32,  // 0.0-1.0 (how much to compensate)
    crosshair_color: u32,
    guide_enabled: bool,
}

impl RecoilHelper {
    pub fn new() -> Self {
        let mut helper = Self {
            weapon_patterns: HashMap::new(),
            current_weapon: None,
            shot_count: 0,
            last_shot_time: std::time::Instant::now(),
            compensation_strength: 1.0,
            crosshair_color: 0x00FF00, // Green
            guide_enabled: true,
        };
        
        // Load default weapon patterns
        helper.load_default_patterns();
        helper
    }
    
    /// Load weapon recoil patterns from file or hardcoded data
    fn load_default_patterns(&mut self) {
        // AK47 pattern (example - needs real data)
        let ak47 = WeaponRecoil {
            weapon_id: 1,
            weapon_name: "AK47".to_string(),
            recoil_pattern: vec![
                Vec2 { x: 0.0, y: 4.0 },     // Shot 1
                Vec2 { x: -0.5, y: 8.0 },    // Shot 2
                Vec2 { x: 1.0, y: 10.0 },    // Shot 3
                Vec2 { x: -1.5, y: 12.0 },   // Shot 4
                Vec2 { x: 2.0, y: 14.0 },    // Shot 5
                Vec2 { x: -2.0, y: 15.0 },   // Shot 6
                Vec2 { x: 2.5, y: 16.0 },    // Shot 7
                Vec2 { x: -2.5, y: 17.0 },   // Shot 8
                // ... more shots
            ],
            fire_rate: 450.0,
            current_shot: 0,
            max_pattern_length: 30,
        };
        self.weapon_patterns.insert(1, ak47);
        
        // LR300 pattern (example)
        let lr300 = WeaponRecoil {
            weapon_id: 2,
            weapon_name: "LR300".to_string(),
            recoil_pattern: vec![
                Vec2 { x: 0.0, y: 2.0 },
                Vec2 { x: -0.2, y: 4.0 },
                Vec2 { x: 0.3, y: 6.0 },
                Vec2 { x: -0.4, y: 8.0 },
                // ... more shots
            ],
            fire_rate: 600.0,
            current_shot: 0,
            max_pattern_length: 30,
        };
        self.weapon_patterns.insert(2, lr300);
        
        // MP5 pattern (example)
        let mp5 = WeaponRecoil {
            weapon_id: 3,
            weapon_name: "MP5".to_string(),
            recoil_pattern: vec![
                Vec2 { x: 0.0, y: 1.5 },
                Vec2 { x: 0.1, y: 3.0 },
                Vec2 { x: -0.1, y: 4.5 },
                // ... more shots
            ],
            fire_rate: 800.0,
            current_shot: 0,
            max_pattern_length: 30,
        };
        self.weapon_patterns.insert(3, mp5);
    }
    
    /// Read weapon recoil data from game memory (SAFE - read-only)
    pub fn read_weapon_from_memory(&mut self, process: &Process, player_addr: usize, held_entity_offset: usize, weapon_recoil_offset: usize) -> Option<WeaponRecoil> {
        // Read held entity
        let held_entity = process.read::<usize>(player_addr + held_entity_offset).ok()?;
        if held_entity == 0 {
            return None;
        }
        
        // Read weapon ID (simplified - actual offset needs to be found)
        let weapon_id = process.read::<u32>(held_entity + 0x28).ok()?;
        
        // Read recoil properties address
        let recoil_props = process.read::<usize>(held_entity + weapon_recoil_offset).ok()?;
        if recoil_props == 0 {
            return None;
        }
        
        // Read current shot count
        let current_shot = process.read::<u32>(recoil_props + 0x10).ok()?;
        
        // Get pattern from cache or create new
        let mut weapon = self.weapon_patterns.get(&weapon_id)?.clone();
        weapon.current_shot = current_shot;
        
        Some(weapon)
    }
    
    /// Update current weapon state
    pub fn update_weapon(&mut self, weapon: WeaponRecoil) {
        self.current_weapon = Some(weapon);
        self.shot_count = 0;
    }
    
    /// Detect if player is shooting (read-only check)
    pub fn is_shooting(&self, process: &Process, player_input_addr: usize) -> bool {
        // Read attack button state from PlayerInput
        // Offset needs to be found in memory
        if let Ok(attack_state) = process.read::<bool>(player_input_addr + 0x18) {
            return attack_state;
        }
        false
    }
    
    /// Get compensated aim position (where to aim to counter recoil)
    pub fn get_compensation_offset(&self) -> Vec2 {
        if let Some(ref weapon) = self.current_weapon {
            let shot_index = (weapon.current_shot as usize) % weapon.recoil_pattern.len();
            
            if shot_index < weapon.recoil_pattern.len() {
                let recoil = weapon.recoil_pattern[shot_index];
                
                // Invert recoil for compensation
                return Vec2 {
                    x: -recoil.x * self.compensation_strength,
                    y: -recoil.y * self.compensation_strength,
                };
            }
        }
        
        Vec2 { x: 0.0, y: 0.0 }
    }
    
    /// Get visual guide position for overlay
    pub fn get_compensated_crosshair(&self, screen_center: Vec2) -> Vec2 {
        if !self.guide_enabled {
            return screen_center;
        }
        
        let offset = self.get_compensation_offset();
        
        Vec2 {
            x: screen_center.x + offset.x,
            y: screen_center.y + offset.y,
        }
    }
    
    /// Increment shot counter (called when player fires)
    pub fn on_shot_fired(&mut self) {
        // Check if this is a new spray or continued spray
        let time_since_last = self.last_shot_time.elapsed().as_millis();
        
        if let Some(ref weapon) = self.current_weapon {
            let shot_interval = 60000.0 / weapon.fire_rate; // ms between shots
            
            if time_since_last > shot_interval as u128 * 2 {
                // New spray - reset counter
                self.shot_count = 0;
            }
        }
        
        self.shot_count += 1;
        self.last_shot_time = std::time::Instant::now();
    }
    
    /// Reset shot counter (called when player stops shooting)
    pub fn reset_spray(&mut self) {
        self.shot_count = 0;
    }
    
    /// Get recoil pattern visualization data
    pub fn get_pattern_visualization(&self) -> Vec<Vec2> {
        if let Some(ref weapon) = self.current_weapon {
            // Return full pattern for display
            weapon.recoil_pattern.clone()
        } else {
            Vec::new()
        }
    }
    
    /// Set compensation strength (0.0 = none, 1.0 = full)
    pub fn set_compensation_strength(&mut self, strength: f32) {
        self.compensation_strength = strength.clamp(0.0, 1.0);
    }
    
    /// Enable/disable visual guide
    pub fn set_guide_enabled(&mut self, enabled: bool) {
        self.guide_enabled = enabled;
    }
    
    /// Get current weapon info for display
    pub fn get_weapon_info(&self) -> Option<String> {
        if let Some(ref weapon) = self.current_weapon {
            Some(format!(
                "{} | Shot {}/{} | RPM: {:.0}",
                weapon.weapon_name,
                weapon.current_shot + 1,
                weapon.max_pattern_length,
                weapon.fire_rate
            ))
        } else {
            None
        }
    }
    
    /// Calculate perfect aim point for target (with bullet drop compensation)
    pub fn calculate_aim_with_compensation(&self, target_pos: Vec3, camera_pos: Vec3, bullet_speed: f32, gravity: f32) -> Vec3 {
        let distance = ((target_pos.x - camera_pos.x).powi(2) + 
                       (target_pos.y - camera_pos.y).powi(2) + 
                       (target_pos.z - camera_pos.z).powi(2)).sqrt();
        
        let time_to_target = distance / bullet_speed;
        
        // Compensate for bullet drop (gravity)
        let drop = 0.5 * gravity * time_to_target * time_to_target;
        
        // Get recoil compensation
        let recoil_offset = self.get_compensation_offset();
        
        Vec3 {
            x: target_pos.x - recoil_offset.x,
            y: target_pos.y + drop - recoil_offset.y,
            z: target_pos.z,
        }
    }
}

/// Helper function to load weapon patterns from JSON file
pub fn load_weapon_patterns_from_file(path: &str) -> Result<HashMap<u32, WeaponRecoil>, Box<dyn std::error::Error>> {
    use std::fs;
    
    let content = fs::read_to_string(path)?;
    
    // Parse JSON (requires serde_json)
    // For now, return empty HashMap
    Ok(HashMap::new())
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_recoil_compensation() {
        let helper = RecoilHelper::new();
        
        // Should have loaded default patterns
        assert!(helper.weapon_patterns.len() > 0);
    }
    
    #[test]
    fn test_compensation_offset() {
        let mut helper = RecoilHelper::new();
        
        // Set AK47 as current weapon
        if let Some(ak47) = helper.weapon_patterns.get(&1).cloned() {
            helper.update_weapon(ak47);
            
            // Get compensation for first shot
            let offset = helper.get_compensation_offset();
            
            // Should be opposite of recoil
            assert!(offset.y < 0.0); // Recoil goes up, compensation goes down
        }
    }
}
