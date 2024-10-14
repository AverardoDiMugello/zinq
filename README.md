# Zinq

Zinq is a framework for building deterministic hardware emulators with APIs to instrument the running software.

The initial API concepts can be viewed in the examples directory. The sail_linux_boot.rs file contains a model of the simplest possible ARM machine that can boot a Linux kernel, borrowed from the [Sail ARMv9.4a emulator test case](https://github.com/rems-project/sail-arm/tree/master/arm-v9.4-a).

The Linux boot currently takes about 5-7 minutes in release mode, far too slow to be useful for most users, but still about 5x faster than Sail's C emulator. Nearly all of this time is spent in emulating address translation logic, because Zinq does not yet support TLB emulation. This will be added soon. The example can be run with the below command. The only dependency is cargo and rustc for the host computer.

```
cargo run --release --example sail_linux_boot
```
