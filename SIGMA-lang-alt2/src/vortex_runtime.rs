use crate::vortex_core::PhaseSync;
use std::collections::HashMap;

pub struct VortexRuntime {
    pub core: PhaseSync,
    pub heap: HashMap<String, f64>,
}

impl VortexRuntime {
    pub fn new() -> Self {
        VortexRuntime { 
            core: PhaseSync::new(),
            heap: HashMap::new(),
        }
    }

    pub fn request_access(&mut self, angle: u32) {
        let width = 45;
        let start = if angle >= width { angle - width } else { 0 };
        let end = angle + width;

        if self.core.lock_sector(start, end) {
            println!("\x1b[1;32m  [GEOM]\x1b[0m Sector {}°-{}° Locked.", 
start, end);
            self.core.rotate_to(angle);
        } else {
            println!("\x1b[1;31m  [RECOVERY]\x1b[0m Resolving Collision at 
{}°...", angle);
        }
    }

    pub fn mirror_process(&mut self, data: &str) {
        let reversed: String = data.chars().rev().collect();
        println!("\x1b[1;35m  [MIRROR]\x1b[0m {} <=> {}", data, reversed);
        self.heap.insert(data.to_string(), 1.0);
    }
}

