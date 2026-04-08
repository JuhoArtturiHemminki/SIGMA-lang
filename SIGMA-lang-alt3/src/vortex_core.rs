pub struct PhaseSync {
    pub current_angle: u32,
    pub axis_locked: bool,
    pub active_sectors: Vec<(u32, u32)>,
}

impl PhaseSync {
    pub fn new() -> Self {
        PhaseSync {
            current_angle: 0,
            axis_locked: true,
            active_sectors: Vec::new(),
        }
    }

    pub fn lock_sector(&mut self, start: u32, end: u32) -> bool {
        for (s, e) in &self.active_sectors {
            if (start >= *s && start < *e) || (end > *s && end <= *e) {
                return false;
            }
        }
        self.active_sectors.push((start, end));
        true
    }

    pub fn rotate_to(&mut self, angle: u32) {
        if self.axis_locked {
            self.current_angle = angle % 360;
            println!("\x1b[1;33m[CORE]\x1b[0m Axis at {}°", 
self.current_angle);
        }
    }
}

