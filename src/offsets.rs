// Rust game offsets - Unity IL2CPP
// These are EXAMPLE offsets - you need to update them with actual game offsets
// Use a tool like Il2CppDumper or ReClass.NET to find current offsets

pub struct RustOffsets {
    // GameAssembly.dll patterns and offsets
    pub game_object_manager: usize,      // Pattern to find
    pub base_networkable: usize,
    pub base_player: usize,
    
    // BasePlayer offsets
    pub player_model: usize,
    pub player_input: usize,
    pub player_inventory: usize,
    pub model_state: usize,
    pub life_state: usize,
    pub health: usize,
    pub max_health: usize,
    
    // Transform/Position offsets
    pub transform: usize,
    pub visual_state: usize,
    pub position: usize,
    
    // PlayerInput offsets
    pub body_angles: usize,
    pub view_angles: usize,
    pub recoil_angles: usize,
    
    // PlayerModel offsets
    pub new_velocity: usize,
    
    // BaseNetworkable offsets
    pub entity_realm: usize,
    pub entity_list: usize,
    
    // Weapon offsets
    pub held_entity: usize,
    pub active_item: usize,
    pub weapon_recoil: usize,
    pub recoil_properties: usize,
}

impl RustOffsets {
    pub fn new() -> Self {
        Self {
            // Base pointers (find via pattern scanning)
            game_object_manager: 0x0,  // Find via "48 8B 0D ? ? ? ? 48 85 C9 74 ? 48 8B 49 ? 48 85 C9"
            base_networkable: 0x0,
            base_player: 0x0,
            
            // BasePlayer class offsets (update from IL2CPP dump)
            player_model: 0x4B0,
            player_input: 0x4D8,
            player_inventory: 0x610,
            model_state: 0x570,
            life_state: 0x224,
            health: 0x1F0,
            max_health: 0x1F4,
            
            // Transform
            transform: 0x30,
            visual_state: 0x38,
            position: 0x90,
            
            // PlayerInput
            body_angles: 0x3C,
            view_angles: 0x44,
            recoil_angles: 0x4C,
            
            // PlayerModel
            new_velocity: 0x1D4,
            
            // Networking
            entity_realm: 0x58,
            entity_list: 0x28,
            
            // Weapon
            held_entity: 0x5F8,
            active_item: 0x28,
            weapon_recoil: 0x2E8,
            recoil_properties: 0x2F0,
        }
    }
}

// Common data structures
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct Vec2 {
    pub x: f32,
    pub y: f32,
}

impl Vec3 {
    pub fn distance(&self, other: &Vec3) -> f32 {
        let dx = self.x - other.x;
        let dy = self.y - other.y;
        let dz = self.z - other.z;
        (dx * dx + dy * dy + dz * dz).sqrt()
    }
}
