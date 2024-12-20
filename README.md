**ATTENTION:** A very large re-architecting is currently underway locally and very nearly completed. If not for a brief detour to build and ship another project, it would be ready by now. Don't worry, this project is not dead.

# Zinq

Zinq is a framework for building deterministic hardware emulators with APIs to instrument and analyze the running software. Zinq is a recursive acronym for "Zinq is not QEMU" and seeks to address the short-comings of using QEMU for software analysis. Zinq's architecture is currently in an early research and prototyping phase.

Zinq's initial API concepts can be viewed in the examples directory:

- sail_linux_boot.rs contains a model of the simplest possible ARM machine that can boot a Linux kernel, borrowed from the [Sail ARMv9.4a emulator test case.](https://github.com/rems-project/sail-arm/tree/master/arm-v9.4-a)
- ctrl_flow_trace.rs contains a simple example of instrumenting the same ARMv9.4a machine for control-flow tracing.
- _Coming soon:_ visualize_ttables.rs shows a more complex instrumentation example in which system registers and memory ranges are hooked to display the address translation tables. The hooks are dynamically updated as the emulation progresses to re-display the tables as they are modified during the boot.

The example can be run with the below command. The only dependencies are [cargo and rust](https://www.rust-lang.org/tools/install) for your host machine.

```
cargo run --release --example sail_linux_boot # Or ctrl_flow_trace
```

Notes on the current architecure of Zinq can be found in the docs [here](/docs/Architecture.md).

### Performance

In release mode, the Linux boot in this example takes about 3-3.5 minutes with an emulated TLB and 6-7 minutes without one. With the emulated TLB, this is roughly 10x faster than Sail's C emulator (all perf-related qualifications aside) but still far slower than the performance goals of this project. Even with the emulated TLB, nearly all of the current runtime of this example is spent in ARM address translation code reading system register data for the TLB lookup. This can be easily optimized away in the near future. The control-flow trace only takes about 3.5 minutes, which means the instrumentation is pretty lightweight relative to the uninstrumented performance. However, this doesn't have much significance prior to the first round of optimizations of the base emulation.

### Correctness

The example is checked for correctness against a custom, heavily instrumented version of the Sail ARMv9.4a C emulator booting the same Linux kernel. This instrumented Sail emulator produces a 35 GB trace file of the boot, including the GPR state after every instruction, every address translation result, and every memory transaction. The Zinq model is instrumented to output equivalent information, asserting equivalence to the trace file. The trace took over 24 hours to produce, and the Zinq test case for checking against it takes about 75 minutes to run. Due to the size of the trace file, both it and the test case that checks against it are kept local for now. Sail is considered the "golden model" for ARM for the purposes of this project, as it is auto-generated from ARM's own machine-readable architecture and has the confidence of researchers for use in formal proofs. While the current approach to correctness-checking is fairly rigorous for the narrow purposes of this example and the initial Zinq architecture exploration, a less heavy-handed approach to validation is desired for the long-term and will soon be added.
