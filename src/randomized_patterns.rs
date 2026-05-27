// Randomized Read Patterns - Make memory reads unpredictable
// 100% SAFE - Just changes read order/timing to avoid detection patterns
// EAC cannot detect consistent patterns if we randomize everything

use rand::prelude::*;
use std::time::{Duration, Instant};

pub struct RandomizedPatterns {
    rng: ThreadRng,
    last_read_time: Instant,
    
    // Randomization settings
    min_delay_ms: u64,
    max_delay_ms: u64,
    skip_chance: f32,
    
    // Anti-pattern tracking
    read_count: u64,
    last_delay: u64,
}

impl RandomizedPatterns {
    /// Create new randomized pattern generator
    pub fn new() -> Self {
        Self {
            rng: rand::thread_rng(),
            last_read_time: Instant::now(),
            min_delay_ms: 50,
            max_delay_ms: 150,
            skip_chance: 0.15, // 15% chance to skip
            read_count: 0,
            last_delay: 0,
        }
    }
    
    /// Create with custom settings
    pub fn with_settings(min_delay_ms: u64, max_delay_ms: u64, skip_chance: f32) -> Self {
        Self {
            rng: rand::thread_rng(),
            last_read_time: Instant::now(),
            min_delay_ms,
            max_delay_ms,
            skip_chance: skip_chance.clamp(0.0, 1.0),
            read_count: 0,
            last_delay: 0,
        }
    }
    
    /// Get random delay between reads
    /// Returns delay in milliseconds
    /// Never returns the same delay twice in a row (anti-pattern)
    pub fn get_random_delay(&mut self) -> u64 {
        let mut delay = self.rng.gen_range(self.min_delay_ms..=self.max_delay_ms);
        
        // Avoid repeating same delay (pattern detection)
        if delay == self.last_delay {
            delay = if delay < self.max_delay_ms {
                delay + 10
            } else {
                delay - 10
            };
        }
        
        self.last_delay = delay;
        delay
    }
    
    /// Wait random delay
    pub fn random_delay(&mut self) {
        let delay = self.get_random_delay();
        std::thread::sleep(Duration::from_millis(delay));
        self.last_read_time = Instant::now();
    }
    
    /// Should we skip this read? (random skipping)
    /// Returns true if should skip (15% chance by default)
    pub fn should_skip(&mut self) -> bool {
        self.rng.gen::<f32>() < self.skip_chance
    }
    
    /// Shuffle player list (randomize read order)
    /// EAC can detect if you always read players in same order (1, 2, 3, 4...)
    /// This shuffles the order every time: (3, 7, 1, 4, 2...)
    pub fn shuffle_players<T>(&mut self, players: &mut Vec<T>) {
        players.shuffle(&mut self.rng);
    }
    
    /// Get random subset of players (skip some randomly)
    /// Returns indices of players to read
    pub fn get_random_subset(&mut self, player_count: usize) -> Vec<usize> {
        let mut indices: Vec<usize> = (0..player_count).collect();
        
        // Shuffle order
        indices.shuffle(&mut self.rng);
        
        // Keep only subset (skip some)
        let keep_count = ((player_count as f32) * (1.0 - self.skip_chance)) as usize;
        indices.truncate(keep_count.max(1));
        
        indices
    }
    
    /// Get random read interval (changes every call)
    /// Use this for dynamic update rates instead of fixed intervals
    pub fn get_random_interval_ms(&mut self) -> u64 {
        self.rng.gen_range(100..=300)
    }
    
    /// Randomize offset order (when reading multiple fields)
    /// Instead of always reading: health, position, name...
    /// Randomize to: position, name, health... or name, health, position...
    pub fn shuffle_offsets(&mut self, offsets: &mut Vec<usize>) {
        offsets.shuffle(&mut self.rng);
    }
    
    /// Get random jitter (small random value)
    /// Add to delays for extra randomness: delay + jitter
    pub fn get_jitter_ms(&mut self) -> i64 {
        self.rng.gen_range(-20..=20)
    }
    
    /// Should we take a break? (random longer pauses)
    /// Occasionally take longer breaks to look more human
    /// Returns Some(duration) if should take break
    pub fn should_take_break(&mut self) -> Option<Duration> {
        self.read_count += 1;
        
        // Every 50-100 reads, take a longer break (1-3 seconds)
        if self.read_count % self.rng.gen_range(50..=100) == 0 {
            let break_duration = self.rng.gen_range(1000..=3000);
            return Some(Duration::from_millis(break_duration));
        }
        
        None
    }
    
    /// Get random batch size (for memory batching)
    /// Don't always read same number of players
    /// Sometimes read 5, sometimes 8, sometimes 3...
    pub fn get_random_batch_size(&mut self, min: usize, max: usize) -> usize {
        self.rng.gen_range(min..=max)
    }
    
    /// Randomize update priority
    /// Don't always update same players first
    /// Returns shuffled priority list (0 = highest priority)
    pub fn randomize_priorities(&mut self, count: usize) -> Vec<usize> {
        let mut priorities: Vec<usize> = (0..count).collect();
        priorities.shuffle(&mut self.rng);
        priorities
    }
    
    /// Check if enough time passed since last read
    /// Use random intervals instead of fixed
    pub fn should_read_now(&mut self) -> bool {
        let random_interval = self.get_random_interval_ms();
        self.last_read_time.elapsed() >= Duration::from_millis(random_interval)
    }
    
    /// Get statistics (for debugging)
    pub fn get_stats(&self) -> RandomizedStats {
        RandomizedStats {
            read_count: self.read_count,
            avg_delay_ms: (self.min_delay_ms + self.max_delay_ms) / 2,
            skip_chance: self.skip_chance,
        }
    }
    
    /// Reset statistics
    pub fn reset_stats(&mut self) {
        self.read_count = 0;
    }
}

impl Default for RandomizedPatterns {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone)]
pub struct RandomizedStats {
    pub read_count: u64,
    pub avg_delay_ms: u64,
    pub skip_chance: f32,
}

/// Polymorphic sleep - never sleep same duration twice
/// Makes timing analysis harder for anti-cheat
pub fn polymorphic_sleep(base_ms: u64) {
    let mut rng = rand::thread_rng();
    let jitter = rng.gen_range(-20..=20);
    let actual_ms = (base_ms as i64 + jitter).max(1) as u64;
    std::thread::sleep(Duration::from_millis(actual_ms));
}

/// Random boolean with probability
pub fn random_chance(probability: f32) -> bool {
    rand::random::<f32>() < probability.clamp(0.0, 1.0)
}

/// Weighted random selection
/// Given list of items with weights, return random item
/// Example: [(item1, 0.7), (item2, 0.2), (item3, 0.1)]
/// item1 has 70% chance, item2 has 20% chance, item3 has 10% chance
pub fn weighted_random<T: Clone>(items: &[(T, f32)]) -> Option<T> {
    if items.is_empty() {
        return None;
    }
    
    let total_weight: f32 = items.iter().map(|(_, w)| w).sum();
    if total_weight <= 0.0 {
        return None;
    }
    
    let mut rng = rand::thread_rng();
    let mut roll = rng.gen::<f32>() * total_weight;
    
    for (item, weight) in items {
        if roll < *weight {
            return Some(item.clone());
        }
        roll -= weight;
    }
    
    // Fallback to last item (shouldn't happen with proper weights)
    items.last().map(|(item, _)| item.clone())
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_random_delay() {
        let mut randomizer = RandomizedPatterns::new();
        
        let delay1 = randomizer.get_random_delay();
        let delay2 = randomizer.get_random_delay();
        
        // Delays should be different (anti-pattern)
        assert_ne!(delay1, delay2);
        
        // Delays should be in range
        assert!(delay1 >= 50 && delay1 <= 150);
        assert!(delay2 >= 50 && delay2 <= 150);
    }
    
    #[test]
    fn test_should_skip() {
        let mut randomizer = RandomizedPatterns::new();
        
        let mut skip_count = 0;
        let total = 1000;
        
        for _ in 0..total {
            if randomizer.should_skip() {
                skip_count += 1;
            }
        }
        
        // Should skip ~15% (within reasonable margin)
        let skip_rate = skip_count as f32 / total as f32;
        assert!(skip_rate > 0.10 && skip_rate < 0.20, "Skip rate: {}", skip_rate);
    }
    
    #[test]
    fn test_shuffle_players() {
        let mut randomizer = RandomizedPatterns::new();
        let mut players = vec![1, 2, 3, 4, 5];
        let original = players.clone();
        
        randomizer.shuffle_players(&mut players);
        
        // Order should be different (with very high probability)
        assert_ne!(players, original);
        
        // But should contain same elements
        let mut sorted = players.clone();
        sorted.sort();
        assert_eq!(sorted, original);
    }
    
    #[test]
    fn test_random_subset() {
        let mut randomizer = RandomizedPatterns::new();
        let player_count = 100;
        
        let subset = randomizer.get_random_subset(player_count);
        
        // Subset should be smaller (skipped some)
        assert!(subset.len() < player_count);
        assert!(subset.len() > 0);
        
        // Should be ~85% of players (100% - 15% skip)
        let ratio = subset.len() as f32 / player_count as f32;
        assert!(ratio > 0.75 && ratio < 0.95);
    }
    
    #[test]
    fn test_polymorphic_sleep() {
        use std::time::Instant;
        
        let start = Instant::now();
        polymorphic_sleep(100);
        let duration = start.elapsed();
        
        // Should sleep around 100ms (±20ms jitter)
        let ms = duration.as_millis();
        assert!(ms >= 80 && ms <= 150);
    }
    
    #[test]
    fn test_weighted_random() {
        let items = vec![
            ("high", 0.7),
            ("medium", 0.2),
            ("low", 0.1),
        ];
        
        // Run many times and check distribution
        let mut counts = std::collections::HashMap::new();
        for _ in 0..1000 {
            if let Some(item) = weighted_random(&items) {
                *counts.entry(item).or_insert(0) += 1;
            }
        }
        
        // "high" should appear ~70% of the time
        let high_count = counts.get("high").unwrap_or(&0);
        let high_ratio = *high_count as f32 / 1000.0;
        assert!(high_ratio > 0.60 && high_ratio < 0.80, "High ratio: {}", high_ratio);
    }
}
