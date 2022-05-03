# five
[![build](https://github.com/plavelo/five/workflows/test/badge.svg)](https://github.com/plavelo/five/actions)
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](https://raw.githubusercontent.com/plavelo/five/main/LICENSE)

`five` is a RISC-V emulator in Rust.

This emulator is under development and currently supports RV32I ISA.

# Usage
You'll need to install [cargo-make](https://github.com/sagiegurari/cargo-make) before running the emulator.
```
cargo install cargo-make
```
To run a binary, use the cargo make CLI command.
```
git clone git@github.com:plavelo/five.git
cd five
cargo make cli <path-to-binary>
```

# Testing
In order to run the tests, you'll need [RISC-V toolchain](https://static.dev.sifive.com/dev-tools/riscv64-unknown-elf-gcc-8.1.0-2019.01.0-x86_64-linux-ubuntu14.tar.gz).
```
curl -o toolchain.tar.gz https://static.dev.sifive.com/dev-tools/riscv64-unknown-elf-gcc-8.1.0-2019.01.0-x86_64-linux-ubuntu14.tar.gz
sudo mkdir /opt/riscv
sudo tar xfz toolchain.tar.gz --strip-components 1 -C /opt/riscv
export PATH=/opt/riscv/bin:$PATH
```
Build RISC-V executable binaries for testing.
```
cargo make buildtest
```
Then, run the tests with the following command.
```
cargo test
```
You can also specify a binary to run the test as follows.
```
cargo make cli ./riscv-tests/isa/rv32ui-p-add.bin
```

# Features
* [ ] 32-bit/64-bit ISA
  * [x] RV32I/RV64I (except fence/fence_i/ecall/ebreak)
  * [x] RV32M/RV64M
  * [ ] RV32F/RV64F
  * [ ] RV32D/RV64D
  * [ ] RV32A/RV64A
  * [ ] RV32C/RV64C
  * [ ] Zifencei
  * [ ] Zicsr
* [ ] Privileged ISA
* [ ] Peripheral device
  * [ ] UART
  * [ ] Virtio
* [ ] Device tree

# Resources
* [RISC-V Specifications](https://riscv.org/technical/specifications/)
