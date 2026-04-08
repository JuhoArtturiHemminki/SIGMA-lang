pub enum VortexError {
    PhaseCollision(u32, String),
    GhostPhaseAccess(String),
}

impl VortexError {
    pub fn emit(&self) {
        match self {
            VortexError::PhaseCollision(angle, name) => {
                println!("\x1b[1;31m[Vortex Error]: Phase 
Collision\x1b[0m");
                println!(" -> Angle {}° is occupied by '{}'.", angle, 
name);
            }
            VortexError::GhostPhaseAccess(name) => {
                println!("\x1b[1;31m[Vortex Error]: Ghost Phase 
Access\x1b[0m");
                println!(" -> Segment '{}' was consumed.", name);
            }
        }
    }
}

