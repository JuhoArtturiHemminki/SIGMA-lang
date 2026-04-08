use crate::vortex_core::PhaseSync;
use std::collections::HashMap;

pub struct VortexRuntime {
    pub tick: u64,
    pub core: PhaseSync,
    pub heap: HashMap<String, f64>,
    pub active_class: u32,
}

impl VortexRuntime {
    pub fn new() -> Self {
        VortexRuntime { 
            tick: 0,
            core: PhaseSync::new(),
            heap: HashMap::new(),
            active_class: 1,
        }
    }

    pub fn set_class(&mut self, class: u32) {
        self.active_class = class;
        println!("\x1b[1;35m  [CLASS]\x1b[0m Switched to Layer: {}", 
class);
    }

    pub fn mirror_check(&self, original: &str, mirrored: &str) -> bool {
        let rev: String = original.chars().rev().collect();
        rev == mirrored
    }

    pub fn request_access(&mut self, angle: u32) {
        if self.core.lock_sector(angle, angle + 45) {
            self.core.rotate_to(angle);
            self.tick += 1;
        } else {
            println!("\x1b[1;31m  [COLLISION]\x1b[0m Geometric Overlap at 
{}°!", angle);
        }
    }
}

