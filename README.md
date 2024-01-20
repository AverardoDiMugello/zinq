# Zinq

Zinq is not QEMU; it is a generic emulator framework. Zinq aims to help software developers build software without hardware. It seeks to provide an easy-to-use API for building high-performance and high-fidelity transaction-level models of diverse computer systems. Zinq leverages several features of the Rust programming language to achieve this.

In Zinq, the system model and the emulation model are entirely separate, enabling a system model to be emulated in different ways according to the needs of the developer. For example, a developer may emulate some software under a high-speed, timing-inaccurate, non-deterministic emulation model until the system reaches a certain condition, and then switch the emulation to a lower-speed, higher-fidelity, deterministic emulation model for single-stepping through. Zinq emulators operate off of an abstract representation of instructions, memory, and events, allowing the emulators to be seamlessly retargeted to any system modeled with Zinq. This allows new, innovative emulation ideas to become immediately accessible to all Zinq-based models.

The Zinq framework lives in the zinq/ directory. It currently supports modeling instruction semantics and systems with one processor and a contiguous byte-addressabel address space. It will soon support modeling interrupts, MMIO, buses, peripherals, and multi-processor and heterogeneous systems.

Zinq seeks to provide standard model implementations of common architectures and devices, as well as standard emulator implementations. These standard implementations live in the znstd/ directory.

The Zinq std Arm architecture model lives in znstd/arch/arm. It currently supports ARMv8 immediate arithmetic and unconditional branching. It will soon support the entire ARM architecture.

The Zinq std emulators live in znstd/emulator. It currently contains a single-step emulator for systems that Zinq can model. It will soon include a QEMU-style basic-block-step JIT-compiling emulator, significantly more parameterization of the single-step emulator, a code-coverage emulator for fuzzing, and a symbolic execution emulator.

Much more to come.
