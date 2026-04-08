use crate::vortex_error::VortexError;
use std::fs;
use std::path::Path;

pub struct Compiler {
    target: String,
}

impl Compiler {
    pub fn new(target: &str) -> Self {
        Compiler {
            target: target.to_string(),
        }
    }

    pub fn compile(&self, source: &str, file_path: &Path) -> 
Result<String, VortexError> {
        let mut signature_id = 0;
        for c in source.chars() {
            signature_id = (signature_id + c as u32) % 4096;
        }

        let binary_content = 
format!("SIGMA_VORTEX_EXE::{}\nTARGET::{}\nSTATUS::STABLE", signature_id, 
self.target);
        
        let mut output_path = file_path.to_path_buf();
        output_path.set_extension("vortex");
        
        if fs::write(output_path, binary_content).is_ok() {
            Ok(format!("SIGMA_VORTEX_EXE::{}", signature_id))
        } else {
            Err(VortexError::GhostPhaseAccess("IO_FAILURE".to_string()))
        }
    }
}

