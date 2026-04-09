import os
import sys
import subprocess

# Lisätään polku kääntäjän kansioon
sys.path.append(os.path.join(os.getcwd(), 'compiler', 'sigma-c'))

def run_pipeline(rust_file):
    print(f"--- STARTING RUST-VORTEX PIPELINE: {rust_file} ---")
    
    try:
        # 1. Rust -> Sigma (Transpilation)
        print("[1/3] Transpiling Rust to Sigma Syntax...")
        import rust_bridge
        
        with open(rust_file, "r") as f:
            rust_code = f.read()
            sigma_code = rust_bridge.rust_to_sigma(rust_code)
            with open("temp.sigma", "w") as s:
                s.write(sigma_code)

        # 2. Sigma -> Executable Python (Compilation)
        print("[2/3] Compiling Sigma to Kinetic Runtime...")
        import transpiler
        
        with open("temp.sigma", "r") as s:
            runtime_code = transpiler.transpile(s.read())
            with open("temp_runtime.py", "w") as r:
                r.write(runtime_code)

        # 3. Execution (Vortex Emulation)
        print("[3/3] Launching Vortex Engine...")
        subprocess.run(["python3", "temp_runtime.py"])

    except Exception as e:
        print(f"ERROR in Pipeline: {e}")
    
    finally:
        # Siivous
        if os.path.exists("temp.sigma"): os.remove("temp.sigma")
        if os.path.exists("temp_runtime.py"): os.remove("temp_runtime.py")
        print("--- PIPELINE EXECUTION COMPLETE ---")

if __name__ == "__main__":
    if len(sys.argv) > 1:
        run_pipeline(sys.argv[1])
    else:
        print("Usage: python3 vortex_run.py demo.rs")
