import re
import math

def transpile(sigma_code):
    sigma_code = re.sub(r'#.*', '', sigma_code)
    axes = re.findall(r'axis \w+ (\w+) @ (\d+)deg', sigma_code)
    # Etsitään uudet NTI-solmut: nti_node [Name] @ [Angle] weight [Value]
    nti_nodes = re.findall(r'nti_node (\w+) @ (\d+)deg weight ([\d\.]+)', sigma_code)
    steps = re.findall(r'step@(\d+)\((\w+)\)\s*->\s*\w+\s*\{(.*?)\}', sigma_code, re.DOTALL)
    spiral = re.search(r'spiral\s*\(axis \w+ = 0; \w+ < (\d+);', sigma_code)
    rotations = int(spiral.group(1)) if spiral else 1

    python_code = [
        "import math",
        "import time",
        "class SigmaFinalRuntime:",
        "    def __init__(self):",
        "        self.phase = 0.0",
        "        self.memory = " + str({name: 0 for name, angle in axes}),
        "        self.torque = 100.0",
        "        self.nodes = " + str([{'target': int(a), 'weight': float(w), 'label': n} for n, a, w in nti_nodes]),
        "        self.exhaust_vault = 0",
        "    def run_cycle(self):",
        "        self.phase = (self.phase + 1.0) % 360",
        "        self.torque *= 0.9995 # Optimized global friction",
        "        # 1. NTI RESONANCE ENGINE",
        "        for node in self.nodes:",
        "            if abs(self.phase - node['target']) < 1.0:",
        "                activation = self.torque * node['weight']",
        "                if activation > 45.0:",
        "                    print(f' [RES] {node[\"label\"]} Fired | Amplitude: {activation:.2f}')",
        "        # 2. DEAD-ANGLE SINK (DAS)",
        "        if 355 <= self.phase <= 359: self.exhaust_vault += 1"
    ]
    
    for angle, axis_name, body in steps:
        val_match = re.search(r'val\s*=\s*([\d\.]+)', body)
        value = val_match.group(1) if val_match else "1"
        python_code.append(f"        if {angle} <= self.phase < {int(angle)+2}:")
        python_code.append(f"            self.memory['{axis_name}'] = {value}")

    python_code.extend([
        "    def run(self, rotations):",
        "        print('--- SIGMA VORTEX PRODUCTION RUNTIME ACTIVE ---')",
        "        for _ in range(int(360 * rotations)): self.run_cycle()",
        "        print('--- FINAL ARCHITECTURAL REPORT ---')",
        "        print(f'Memory State: {self.memory}')",
        "        print(f'Final Torque: {self.torque:.2f}%')",
        "        print(f'Exhaust Cycles Cleaned: {self.exhaust_vault}')",
        "if __name__ == '__main__':",
        "    app = SigmaFinalRuntime()",
        f"    app.run({rotations})"
    ])
    
    return "\n".join(python_code)

if __name__ == "__main__":
    with open("main.sigma", "r") as f:
        compiled = transpile(f.read())
        with open("compiled_sigma.py", "w") as out:
            out.write(compiled)
    print("Full Architecture Integrated: Sigma-C is now Omega-Class.")
