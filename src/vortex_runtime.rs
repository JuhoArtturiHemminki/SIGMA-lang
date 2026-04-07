use crate::vortex_core::PhaseSync;
use std::collections::HashMap;

pub struct VortexRuntime {
    pub tick: u64,
    pub core: PhaseSync,
    pub heap: HashMap<String, f64>,
}

impl VortexRuntime {
    pub fn new() -> Self {
        VortexRuntime { 
            tick: 0,
            core: PhaseSync::new(),
            heap: HashMap::new(),
        }
    }

    pub fn request_access(&mut self, angle: u32) {
        let width = 45;
        let start = if angle >= width { angle - width } else { 0 };
        let end = angle + width;

        if self.core.lock_sector(start, end) {
            self.core.rotate_to(angle);
            self.draw_ui();
            self.tick += 1;
        } else {
            println!("\x1b[1;31m  [VIOLATION]\x1b[0m Sector {}°-{}° is 
LOCKED!", start, end);
        }
    }

    pub fn draw_ui(&self) {
        let angle = self.core.current_angle;
        let radar = match angle {
            0 | 360 => "[#-------]",
            90      => "[---#----]",
            180     => "[-----#--]",
            270     => "[-------#]",
            _       => "[----?---]",
        };
        println!("\x1b[1;36m┌── VORTEX STATUS ──┐\x1b[0m");
        println!("│ Phase: {:<11}│", radar);
        println!("│ Angle: {:>3}°      │", angle);
        
println!("\x1b[1;36m└───────────────────┘\x1b[0m");
    }

    pub fn stabilize(&mut self) {
        self.core.active_sectors.clear();
        println!("\x1b[1;32m[SYSTEM]\x1b[0m All sectors released.");
    }
}

