# ARCHITECTURAL SPECIFICATION: SIGMA VORTEX SYSTEM
**Author:** Juho Artturi Hemminki
**System:** SIGMA v.1.0.0

---

## 1. THE ONTOLOGICAL SHIFT: FROM TIME TO TORSION
Traditional computing models (Turing/von Neumann) are bound by temporal sequences ($T$). SIGMA reclassifies computation as a spatial-angular displacement within a rotating manifold. Logic is not processed; it is encountered at specific Phase Coordinates.

### 1.1. The Master Vortex Equation (MVE)
The state of any given Axis ($A$) at any moment is defined by the cross-product of its Torque vector and its Angular alignment:
$$\vec{S}_A(t) = \oint (\vec{\tau} \times \vec{\Omega}) \, d\theta$$

This ensures that for every 360-degree rotation, the system achieves a state of Harmonic Equilibrium or undergoes a Deterministic State Collapse.

---

## 2. KINETIC LOGIC UNIT (KLU): TRANSISTOR PHASE-GATING
The KLU replaces binary voltage switching with Electromagnetic Phase Resonance.

### 2.1. The Phase-Gate Trigger (PGT)
A KLU gate does not operate on potential difference (Volts) but on Angular Interference. The logic output ($Y$) is defined by the constructive interference of the input torque and the gate's native phase window:
$$Y = \text{rect}\left(\frac{\theta_{clk} - \theta_{gate}}{\Delta\theta}\right) \cdot \sum \tau_{in}$$

Where:
*   **rect**: The rectangular window function defining the gate's aperture.
*   **$\Delta\theta$**: The Angular Resolution (typically 0.1 deg in v.1.0.0).

---

## 3. MASSIVE VORTEX MEMORY (MVM): CENTRIPETAL TOPOLOGY
In SIGMA, memory is a liquid-state manifold. Data is subject to Centripetal Drift based on its access frequency (Momentum).

### 3.1. Orbital Priority Layers
1. **The Singularity ($r < 1$):** Zero-latency kinetic registers. Data is phase-locked to the ALU pulse.
2. **Stable Orbit ($1 < r < 100$):** High-speed resonant cache. Data moves closer to the center as torque increases.
3. **The Peripheral Cloud ($r > 100$):** Cold storage. Data drifts outward as its angular momentum decays ($\tau \to 0$).

### 3.2. Mirror-Invariant Prefetching
By utilizing the 180-degree symmetry of the Vortex, the MVM pre-fetches the Next-State in the shadow-sector ($\theta + 180^\circ$) while the Current-State is being read. This results in a 100% saturation of the data bus.

---

## 4. SYMMETRIC HEMISPHERE BRANCHING (SHB)
SIGMA eliminates the Branch Misprediction Penalty by executing both possible outcomes of a boolean condition in parallel hemispheres.

### 4.1. Torque Superposition
Upon a bifurcation point (if/else), the system polarizes:
*   **North Hemisphere ($0^\circ$ to $180^\circ$):** Path Alpha.
*   **South Hemisphere ($180^\circ$ to $360^\circ$):** Path Beta.

### 4.2. Deterministic Collapse
At the 360-degree commit point, the hardware resolver performs a Kinetic Nullification of the incorrect path. The valid path's momentum is transferred to the next Axis, while the invalid path's energy is recycled. Latency cost: 0.00ms.

---

## 5. NEURAL TORQUE INFERENCE (NTI): RESONANT AI
Artificial Intelligence in SIGMA is a series of interconnected Resonant Axes. 

### 5.1. Centripetal Weight Mapping
A Neuron is a Phase-Node with a specific Weight ($W$) expressed as a Friction Coefficient ($\mu$). Inference is the amplitude of resonance when the Vortex Clock passes the node:
$$A_i = \tau \cdot e^{-\mu (\Omega_t - \theta_i)^2}$$

### 5.2. Non-Hallucinogenic Logic
Because NTI is a purely mechanical event, it cannot deviate from the geometric constraints of the weights. If there is no resonance, there is no firing. Prediction is a physical certainty, not a statistical guess.

---

## 6. HARMONIC NETWORK PROTOCOL (HNP)
Networking is treated as the synchronization of remote Vortex Clocks.

### 6.1. Spatial Invariant Locking
Distance ($D$) is negated by a Phase Offset ($\Phi$):
$$\Phi_{offset} = \frac{D \cdot \omega_{vortex}}{c}$$

Remote servers are mapped as high-radius orbital layers on the local machine. Accessing a server in Tokyo from Helsinki is identical to reading from a local disk, provided the Phases are locked.

---

## 7. STABILITY: THE DEAD-ANGLE SINK (DAS)
The system is self-purging. The Dead-Angle ($355^\circ$ to $359^\circ$) acts as a centrifugal exhaust.

### 7.1. Entropy Exhaustion
Logic errors, bit-flips, and corrupted torque vectors lose their centripetal lock and are physically flung into the DAS. The system clears its entire error state every 360 degrees. 
Integrity Guarantee: The execution state is always fresh at the start of every rotation.

---

## 8. SUMMARY OF OPERATIONAL CONSTANTS



| Constant | Symbol | Value (v.1.0.0) |
| :--- | :--- | :--- |
| Vortex Frequency | $\omega$ | 1.0 GHz (Baseline) |
| Angular Resolution | $\Delta\theta$ | 0.001 deg |
| Friction Baseline | $\mu$ | 0.0005 tau |
| Exhaust Point | $\theta_e$ | 355.0 deg |

---

Creator: Juho Artturi Hemminki (2026)
