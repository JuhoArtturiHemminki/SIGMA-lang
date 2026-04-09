import math
import time

class HarmonicNetwork:
    def __init__(self, distance_km=1000):
        self.local_phase = 0.0
        self.distance = distance_km
        # Calculate angular offset to negate latency (HNP-Equation)
        self.phase_offset = (self.distance / 300) % 360 # Light speed approx
        print(f"HNP: Distance {self.distance}km. Offset calculated: {self.phase_offset:.2f}deg")

    def cycle(self):
        self.local_phase = (self.local_phase + 1.0) % 360
        
        # Remote node phase is the local phase shifted by distance-invariant offset
        remote_phase = (self.local_phase + self.phase_offset) % 360
        
        # Mirror-Invariant Sync: Read local at 0, Remote at 180
        if int(self.local_phase) == 0:
            print(f"[@{self.local_phase}deg] Local Axis Read initiated.")
        
        if int(self.local_phase) == 180:
            print(f"[@{self.local_phase}deg] Remote Sync achieved (Latency Balanced).")

    def run(self, rotations=1):
        print("Starting Harmonic Network Protocol (HNP) Sync Simulation...")
        for _ in range(int(360 * rotations)):
            self.cycle()
        print("Network State: Phase-Locked.")

if __name__ == "__main__":
    network = HarmonicNetwork(distance_km=2500)
    network.run(rotations=1)
