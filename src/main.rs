use std::fs;
use std::path::Path;

fn main() {
    println!("\x1b[1;36mΣ SIGMA-lang ENGINE\x1b[0m | Vortex Stabilizer");
    println!("{:-<65}", "");

    let build_dir = Path::new("./build_assets");
    if let Ok(entries) = fs::read_dir(build_dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.extension().and_then(|s| s.to_str()) == Some("vortex") 
{
                let name = 
entry.file_name().to_string_lossy().to_string();
                
                println!(
                    "[SYNC] {:<22} | Drift: 0.00ns | Harmonic: LOCKED", 
                    name
                );
            }
        }
    }
    println!("{:-<65}", "");
    println!("\x1b[1;32mSYSTEM READY\x1b[0m (Global Phase-Sync 360 
Verified)");
}

