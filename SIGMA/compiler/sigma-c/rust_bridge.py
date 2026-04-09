import re

def rust_to_sigma(rust_code):
    # 1. Map Variables to Axis-Ankors
    variables = re.findall(r'let\s+(\w+)\s*:\s*\w+\s*=\s*([\d\.]+);', rust_code)
    
    sigma_code = ["# SIGMA RUST-NTI BRIDGE (V4: RESONANCE ACTIVE)\n"]
    
    for i, (name, val) in enumerate(variables):
        angle = i * 90
        sigma_code.append(f"axis float32 {name} @ {angle}deg : r0;")

    # 2. Add NTI Watchdog for intelligent state monitoring
    # This replaces rigid if-else with a kinetic resonance node
    sigma_code.append("\nnti_node Rust_Watchdog @ 180deg weight 0.98;")

    sigma_code.append("\nfn main_logic() {")
    for name, val in variables:
        sigma_code.append(f"    step@0({name}) -> p {{ p.point(|val| val = {val}); }}")
    
    # Static torque push to simulate activity
    sigma_code.append("    step@90(GlobalTorque) -> p { p.point(|val| val = 100.0); }")
    sigma_code.append("}")

    sigma_code.append("\nspiral (axis i = 0; i < 20; i++) {")
    sigma_code.append("    sync main_logic();")
    sigma_code.append("}")
    
    return "\n".join(sigma_code)
