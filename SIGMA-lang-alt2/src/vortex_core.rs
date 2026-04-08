pub struct PhaseSync {
    pub current_angle: u32,
    pub active_sectors: Vec<(u32, u32)>,
    pub mirror_mode: bool,
}

impl PhaseSync {
    pub fn new() -> Self {
        PhaseSync {
            current_angle: 0,
            active_sectors: Vec::new(),
            mirror_mode: true,
        }
    }

    pub fn lock_sector(&mut self, start: u32, end: u32) -> bool {
        for (s, e) in &self.active_sectors {
            if self.mirror_mode {
                if start == *s && end == *e { return true; }
            }
            if (start >= *s && start < *e) || (end > *s && end <= *e) {
                return false;
            }
        }
        self.active_sectors.push((start, end));
        true
    }

    pub fn rotate_to(&mut self, angle: u32) {
        self.current_angle = angle % 360;
    }
}

