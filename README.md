# five
![build](https://github.com/plavelo/five/workflows/test/badge.svg)
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](https://raw.githubusercontent.com/plavelo/five/main/LICENSE)

`five` is a RISC-V emulator in Rust.

This emulator is under development and currentry supports RV32I ISA.

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
* [ ] 32-bit ISA
  * [x] RV32I (except fence/fence_i/ecall/ebreak)
  * [ ] RV32M
  * [ ] RV32A
  * [ ] RV32F
  * [ ] RV32D
  * [ ] Zifencei
  * [ ] Zicsr
* [ ] 64-bit ISA
  * [ ] RV64I
  * [ ] RV64M
  * [ ] RV64A
  * [ ] RV64F
  * [ ] RV64D
  * [ ] Zifencei
  * [ ] Zicsr
* [ ] Privileged ISA
* [ ] Peripheral device
  * [ ] UART
  * [ ] Virtio
* [ ] Device tree

# Resources
* [RISC-V Specifications](https://riscv.org/technical/specifications/)
