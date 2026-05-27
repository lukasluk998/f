# SAFETY GUIDE - Maximum Undetected Usage

## ⚠️ Detection Risk Levels

### HIGH RISK (Ban within hours)
- ❌ Memory writes without driver
- ❌ Obvious stats (100% headshots)
- ❌ Continuous no recoil on official servers
- ❌ Blatant ESP usage (staring through walls)

### MEDIUM RISK (Days to weeks)
- ⚠️ Kernel driver with memory writes
- ⚠️ Pattern-based no recoil
- ⚠️ Driver signature detected
- ⚠️ Player reports

### LOW RISK (Weeks to months)
- ✅ Read-only ESP (external)
- ✅ Macro-based recoil (hardware simulation)
- ✅ Humanized behavior
- ✅ "Legit" play style

### MINIMAL RISK (Rarely detected)
- ✅ Audio radar (sound analysis)
- ✅ External crosshair overlay
- ✅ No game memory access

---

## 🛡️ RECOMMENDED SETUP: LEGIT MODE

This is the SAFEST way to use the cheat:

### What It Does
- ✅ **ESP only** (read-only, external)
- ✅ **NO memory writes** (zero chance of write detection)
- ✅ **Macro recoil** (Logitech/Razer mouse software)
- ✅ **Humanized delays** (looks like real player)
- ✅ **Works WITHOUT driver** (no kernel signature)

### How to Enable

Edit `src/main.rs`:

```rust
// LEGIT MODE - Maximum safety
const LEGIT_MODE: bool = true;  // Change to true

// Features
const ENABLE_ESP: bool = true;         // Read-only, safe
const ENABLE_NO_RECOIL: bool = false;  // Disable memory writes
const ENABLE_AIMBOT: bool = false;     // Disable (too obvious)

// Humanization
const ESP_UPDATE_DELAY_MS: u64 = 500;  // Slower updates = less sus
const RANDOMIZE_DELAYS: bool = true;   // Random timing
```

**Why This Works:**
- EAC can't detect external memory **reads**
- Only **writes** trigger detection
- Macro recoil = physical mouse movement = undetectable
- ESP data never touches game memory

---

## 🎯 SAFEST RECOIL METHODS

### Method 1: Logitech Macro (RECOMMENDED)
**Detection Risk: MINIMAL**

1. Use Logitech G HUB or Ghub software
2. Record mouse movement pattern
3. Trigger on mouse button hold
4. **100% undetectable** (hardware-level)

See `macro_norecoil.md` for scripts.

**Why Safe:**
- Physical mouse movement
- No memory access
- No software hooks
- EAC can't detect

### Method 2: MAKCU Hardware
**Detection Risk: ZERO**

- Arduino-based device
- Physically moves mouse
- Cost: ~$30-50
- See `src/makcu_interface.rs`

**Why Safe:**
- External hardware
- No software footprint
- Literally impossible to detect

### Method 3: Kernel Driver (CURRENT)
**Detection Risk: MEDIUM**

- Memory writes via driver
- Works but signature gets blacklisted
- Re-compile weekly

**Why Risky:**
- Driver signature database
- Heuristic detection of writes
- Pattern-based flagging

---

## 📊 RUNTIME DUMPER SAFETY

**Q: Is runtime dumper detectable?**

**A: NO - It's 100% safe to use.**

**Why:**
- External process (not injected)
- Only reads memory (like Cheat Engine)
- Run it ONCE after patch, then close
- EAC doesn't scan external processes

**Best Practice:**
1. Run dumper to get offsets
2. Close dumper
3. Run cheat (now has correct offsets)
4. Dumper never runs during play

**Detection:**
- ✅ Reading memory = **SAFE**
- ❌ Writing memory = **RISKY**
- ❌ Hooks/injection = **INSTANT BAN**

---

## 🎮 SAFE GAMEPLAY GUIDELINES

### 1. Stats Management
**EAC uses behavioral analysis:**

```
RED FLAGS (auto-ban):
- 100% headshot ratio
- Perfect recoil control (0 deviation)
- Instant 180° snap shots
- Tracking through walls
- Impossible reaction times (<100ms)

SAFE BEHAVIOR:
- 30-50% headshot ratio
- Miss some shots intentionally
- Look around naturally
- Check common spots (like real player)
- 200-300ms reaction time
```

### 2. Server Selection

**AVOID:**
- ❌ Official Facepunch servers (strictest AC)
- ❌ High-pop servers (more reports)
- ❌ Streamer servers (manual review)

**PREFER:**
- ✅ Community servers (relaxed AC)
- ✅ Low-pop servers (<50 players)
- ✅ Modded servers (already chaotic)
- ✅ PVE servers (practice without bans)

### 3. Play Style

**BLATANT (instant ban):**
- Spin bot
- Flying/teleport
- Speed hacks
- One-tapping everyone

**LEGIT (long survival):**
- Slow playstyle
- Use cover
- Miss shots
- React slowly
- Check corners
- Die sometimes

### 4. Session Length

**Pattern Detection:**
- Playing 12+ hours straight with 10+ K/D = flagged
- Winning every fight = flagged
- Never dying = flagged

**Safe Pattern:**
- Play 2-4 hour sessions
- Die occasionally (even on purpose)
- K/D around 2-3 (good but not insane)
- Log off after suspect plays

---

## 🔧 TECHNICAL IMPROVEMENTS

### For LEGIT MODE

Edit `src/main.rs`:

```rust
impl RustCheat {
    fn new(process: Process) -> Option<Self> {
        // LEGIT: Don't load driver at all
        let driver = None;  // Skip driver entirely
        let use_kernel_driver = false;
        
        // Skip EAC bypass techniques
        // We're read-only, don't need them
        
        Some(RustCheat {
            process,
            driver: None,
            no_recoil_enabled: false,  // DISABLED
            ...
        })
    }
    
    // Remove all write operations
    fn apply_no_recoil(&self) {
        // DISABLED in legit mode
        return;
    }
}
```

### Humanized ESP Updates

```rust
fn main() {
    loop {
        // Random delay between updates
        let delay = 300 + (rand::random::<u64>() % 200);  // 300-500ms
        thread::sleep(Duration::from_millis(delay));
        
        // Only update if we're "looking" that direction
        if should_update_esp() {
            let players = cheat.get_players();
            cheat.draw_esp(&players);
        }
    }
}

fn should_update_esp() -> bool {
    // Random skip (looks more human)
    rand::random::<f32>() > 0.1  // 10% chance to skip frame
}
```

---

## 📋 DETECTION METHODS EAC USES

### 1. Memory Write Detection
**How:** Scans for suspicious writes to player data
**Bypass:** Use kernel driver OR don't write at all

### 2. Driver Signature Scanning
**How:** Database of known cheat drivers
**Bypass:** Recompile weekly with new signature

### 3. Behavioral Analysis
**How:** Stats (K/D, accuracy, reaction time)
**Bypass:** Play "legit" - miss shots, die sometimes

### 4. Pattern Detection
**How:** Perfect recoil = 0 deviation = flagged
**Bypass:** Use macro (adds natural variance)

### 5. Hook Detection
**How:** Scans for hooked functions
**Bypass:** Don't hook (we don't)

### 6. Injection Detection
**How:** Scans for injected DLLs
**Bypass:** External cheat (we are)

### 7. Process Enumeration
**How:** Lists suspicious processes
**Bypass:** Rename executable to something normal

### 8. HWID Tracking
**How:** Hardware IDs linked to bans
**Bypass:** HWID spoofer (we have)

---

## ✅ RECOMMENDED CONFIGURATION

For **longest survival** without ban:

```toml
[safety]
mode = "legit"                    # Legit mode
driver_enabled = false             # No driver needed
memory_writes = false              # Read-only

[features]
esp_enabled = true                 # Safe (read-only)
no_recoil_type = "macro"          # Logitech macro (undetectable)
aimbot_enabled = false             # Too obvious
item_esp = false                   # Too obvious (staring at loot)

[humanization]
random_delays = true               # Randomize timing
miss_shots_chance = 0.15          # Miss 15% of shots
reaction_delay_ms = [200, 400]    # Human reaction time
esp_update_rate = [300, 500]      # Slower, randomized

[gameplay]
max_kd_ratio = 3.0                # Stay under 3.0 K/D
intentional_deaths = true          # Die sometimes
check_corners = true               # Look natural
max_session_hours = 4              # Limit playtime
```

Save as `config.toml` and load in cheat.

---

## 🎯 SUMMARY: What To Do

### For Maximum Safety:

1. **Use LEGIT MODE**
   - ESP only (read-only)
   - Macro recoil (Logitech)
   - No driver needed
   - No memory writes

2. **Play Smart**
   - 30-50% accuracy
   - 2-3 K/D ratio
   - Miss shots intentionally
   - Community servers only

3. **Runtime Dumper**
   - ✅ 100% safe to use
   - Run once per patch
   - Close before playing
   - Updates offsets automatically

4. **Test Account First**
   - Never use main account
   - Test on alt for 1 week
   - If no ban → probably safe
   - If banned → tweak settings

---

## 📞 Quick Decision Tree

**Want to cheat with minimal risk?**
→ Use LEGIT MODE (ESP + macro recoil)

**Want full features?**
→ Use kernel driver but expect 1-2 week lifespan

**Just testing?**
→ Use alt account, full features, don't care about ban

**Main account, long-term?**
→ LEGIT MODE only, play smart, 3+ months survival

---

**Remember:** No cheat is 100% undetectable forever. Play smart, use alts, and don't get attached to accounts.
