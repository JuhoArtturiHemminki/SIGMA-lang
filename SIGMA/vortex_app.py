import math

class SigmaApp:
    def __init__(self):
        self.phase = 0.0
        self.health = 100
        self.weight_phase = 90.0  # AI Weight
        self.data_stream = "INERT"
        self.torque = 0.0 # Initial momentum set to zero
        
    def run_cycle(self):
        self.phase = (self.phase + 1.0) % 360
        
        # 1. LOGIC: SHB Branching at the Singularity
        if int(self.phase) == 0:
            self.torque = 100.0 if self.health > 0 else 0.0
            
        # 2. AI: Neural Resonance (Only fires if Torque > 0)
        if abs(self.phase - self.weight_phase) < 1.0 and self.torque > 0:
            print(f"[@{self.phase}deg] AI Inference: SYSTEM_OPTIMAL")

        # 3. SECURITY: VOID Isolation Check
        if 270 <= self.phase <= 275:
            self.data_stream = "PROTECTED"

        # 4. EXHAUST: Dead-Angle Cleaning
        if 355 <= self.phase <= 359:
            self.data_stream = "INERT"

    def run(self, rotations=2): # Run 2 rotations to ensure @0deg trigger
        print("Executing Unified Sigma Application...")
        for _ in range(int(360 * rotations)):
            self.run_cycle()
        print("Application Cycle Complete.")

if __name__ == "__main__":
    app = SigmaApp()
    app.run(rotations=2)
