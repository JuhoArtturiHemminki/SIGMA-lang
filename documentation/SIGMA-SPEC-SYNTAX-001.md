# TECHNICAL SPECIFICATION: THE SIGMA-LANG SYNTAX & OPERATIONAL SEMANTICS
## DOCUMENT ID: SIGMA-SPEC-SYNTAX-001
## SUBJECT: KINETIC PROGRAMMING INTERFACE (KPI)

### 1. THE DECLARATIVE AXIS
In Sigma, variables are not defined as static containers but as **Axis-Ankors** with kinetic properties. A linear assignment like `x = 10` is expressed in Sigma as an angular position.

#### 1.1. Atomic Definition
axis [Type] [Name] @ [PhaseAngle] : [Radius/Priority]
axis int32 Health @ 0deg : r0;      // Core position (L1/Registers)
axis float32 WindSpeed @ 90deg : r1; // Orbital position (RAM)

---

### 2. ROTATIONAL LOGIC: BRANCHING WITHOUT JUMPS
Sigma does not use `if` statements in the traditional sense. Instead, it utilizes the **Hemisphere Splitter** (`split`) operator to distribute torque.

#### 2.2. Physical Bifurcation
```
split (Health > 0) {
    sector TRUE [0deg..180deg] {
        // This code rotates on the upper hemisphere
        Player.Move();
    }
    sector FALSE [180deg..360deg] {
        // This code rotates on the lower hemisphere
        Player.Die();
    }
} commit @ 360deg; // State is solidified at the end of the rotation
```

**Mechanism:** The compiler generates code where both sectors are loaded into the pipeline. The torque of the invalid path is zeroed out (`tau = 0`) once the condition resolves, making it "transparent" to the processor.

---

### 3. ITERATION: THE SPIRAL OPERATOR
Loops are defined as **Torque Decay** (`decay`). A loop rotates as long as it possesses kinetic energy.

#### 3.1. Kinetic Loop
```
spiral (axis i = 0; i < 100; i++) friction(0.01) {
    // Each rotation consumes torque.
    // When i = 100, the Drain-Gate opens and the axis stalls @0deg.
    Matrix.Process(i);
} 
```

A traditional `while(true)` without decay would trigger a **Vacuum Leak** error in the compiler, as the axis could never reach a static state.

---

### 4. VECTOR DYNAMICS: MAVS-OPERATIONS
Vectors are handled using the **Angular Density** ($\rho$) parameter, enabling massive parallelism.

// Define a vector-axis with a density of 512
vector_axis float32 Particles density(512);

// Mirror-Invariant operation:
// Processes particles simultaneously from 0° and 180° directions.
sync Particles.Rotate() mirror_inversion(true);

---

### 5. ERROR HANDLING: CENTRIFUGAL SINKS
Errors do not interrupt the program; they are redirected based on their angle away from the active sector.

```
try @ ActiveSector {
    axis result = Data / Divider;
} trap (DivisionByZero) {
    // Erroneous momentum is automatically routed:
    route Error to DeadAngle(355deg);
}
```

---

### 6. OPERATIONAL SUMMARY: THE VORTEX CYCLE
The execution of a Sigma program is a continuous four-stage cycle:
1.  **Intake (@0°):** Instructions lock into their phase coordinates.
2.  **Compression (@90°):** Torque increases (Data-fetch).
3.  **Power (@180°):** ALU operations and symmetric computation.
4.  **Exhaust (@270°):** Result verification and removal of erroneous momentum.

---------------------------------------------------------------------------
"Software is no longer a sequence; it is a synchronized rotation."
===========================================================================
