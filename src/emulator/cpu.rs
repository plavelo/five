pub mod csr;
mod decoder;
mod executor;
mod f;
mod pc;
mod trap_handler;
mod x;

use std::time::{SystemTime, UNIX_EPOCH};

use crate::{
    emulator::{
        bus::SystemBus,
        cpu::{
            csr::{ControlAndStatusRegister, Csr},
            decoder::{
                privileged::PrivilegedDecoder, rv32f::Rv32fDecoder, rv32i::Rv32iDecoder,
                rv32m::Rv32mDecoder, rv64f::Rv64fDecoder, rv64i::Rv64iDecoder, rv64m::Rv64mDecoder,
                zicsr::ZicsrDecoder, zifencei::ZifenceiDecoder, Decoder,
            },
            executor::{
                privileged::PrivilegedExecutor, rv32f::Rv32fExecutor, rv32i::Rv32iExecutor,
                rv32m::Rv32mExecutor, rv64f::Rv64fExecutor, rv64i::Rv64iExecutor,
                rv64m::Rv64mExecutor, zicsr::ZicsrExecutor, zifencei::ZifenceiExecutor, Executor,
            },
            f::FloatingPointRegister,
            pc::ProgramCounter,
            trap_handler::*,
            x::IntegerRegister,
        },
    },
    isa::{
        csr::user_level::{CYCLE, INSTRET, TIME},
        description::Describer,
        privileged::{
            cause::{Cause, Exception},
            mode::PrivilegeMode,
        },
        register::{fname, xname},
    },
};

#[derive(Default)]
pub struct Cpu {
    x: IntegerRegister,
    f: FloatingPointRegister,
    pc: ProgramCounter,
    pub csr: ControlAndStatusRegister,
    prv: PrivilegeMode,
    pub bus: SystemBus,
}

impl Cpu {
    pub fn run(&mut self, debug: bool, terminator: Option<impl Fn(&Cpu) -> Option<u64>>) -> u64 {
        while self.pc.read() < self.bus.memory.size() {
            let xsnapshot = self.x.snapshot();
            let fsnapshot = self.f.snapshot();
            // read an address from the pc
            let address = self.pc.read();
            // fetch an instruction
            let instruction = self.bus.load32(address);
            // decode and execute the instruction
            let result = if let Some(decoded) = PrivilegedDecoder::decode(instruction) {
                if debug {
                    println!("{:x}: {}", address, decoded.describe());
                }
                PrivilegedExecutor::execute(
                    decoded,
                    &self.prv,
                    &mut self.pc,
                    &mut self.x,
                    &mut self.f,
                    &mut self.csr,
                    &mut self.bus,
                )
            } else if let Some(decoded) = ZifenceiDecoder::decode(instruction) {
                if debug {
                    println!("{:x}: {}", address, decoded.describe());
                }
                ZifenceiExecutor::execute(
                    decoded,
                    &self.prv,
                    &mut self.pc,
                    &mut self.x,
                    &mut self.f,
                    &mut self.csr,
                    &mut self.bus,
                )
            } else if let Some(decoded) = ZicsrDecoder::decode(instruction) {
                if debug {
                    println!("{:x}: {}", address, decoded.describe());
                }
                ZicsrExecutor::execute(
                    decoded,
                    &self.prv,
                    &mut self.pc,
                    &mut self.x,
                    &mut self.f,
                    &mut self.csr,
                    &mut self.bus,
                )
            } else if let Some(decoded) = Rv32iDecoder::decode(instruction) {
                if debug {
                    println!("{:x}: {}", address, decoded.describe());
                }
                Rv32iExecutor::execute(
                    decoded,
                    &self.prv,
                    &mut self.pc,
                    &mut self.x,
                    &mut self.f,
                    &mut self.csr,
                    &mut self.bus,
                )
            } else if let Some(decoded) = Rv64iDecoder::decode(instruction) {
                if debug {
                    println!("{:x}: {}", address, decoded.describe());
                }
                Rv64iExecutor::execute(
                    decoded,
                    &self.prv,
                    &mut self.pc,
                    &mut self.x,
                    &mut self.f,
                    &mut self.csr,
                    &mut self.bus,
                )
            } else if let Some(decoded) = Rv32mDecoder::decode(instruction) {
                if debug {
                    println!("{:x}: {}", address, decoded.describe());
                }
                Rv32mExecutor::execute(
                    decoded,
                    &self.prv,
                    &mut self.pc,
                    &mut self.x,
                    &mut self.f,
                    &mut self.csr,
                    &mut self.bus,
                )
            } else if let Some(decoded) = Rv64mDecoder::decode(instruction) {
                if debug {
                    println!("{:x}: {}", address, decoded.describe());
                }
                Rv64mExecutor::execute(
                    decoded,
                    &self.prv,
                    &mut self.pc,
                    &mut self.x,
                    &mut self.f,
                    &mut self.csr,
                    &mut self.bus,
                )
            } else if let Some(decoded) = Rv32fDecoder::decode(instruction) {
                if debug {
                    println!("{:x}: {}", address, decoded.describe());
                }
                Rv32fExecutor::execute(
                    decoded,
                    &self.prv,
                    &mut self.pc,
                    &mut self.x,
                    &mut self.f,
                    &mut self.csr,
                    &mut self.bus,
                )
            } else if let Some(decoded) = Rv64fDecoder::decode(instruction) {
                if debug {
                    println!("{:x}: {}", address, decoded.describe());
                }
                Rv64fExecutor::execute(
                    decoded,
                    &self.prv,
                    &mut self.pc,
                    &mut self.x,
                    &mut self.f,
                    &mut self.csr,
                    &mut self.bus,
                )
            } else {
                Err(Cause::Exception(Exception::IllegalInstruction))
            };

            if debug {
                self.dump(xsnapshot, fsnapshot);
            }

            if let Some(ref func) = terminator {
                if let Some(result) = func(self) {
                    return result;
                }
            }

            // handle the trap
            if let Err(cause) = result {
                let (prv, pc) =
                    handle_cause(&cause, self.pc.read(), instruction, self.prv, &mut self.csr);
                self.prv = prv;
                self.pc.jump(pc);
            }
            // increment the pc when the pc has not been updated
            else if self.pc.read() == address {
                self.pc.increment();
            }
            // update the cycle
            let cycle = self.csr.read(CYCLE) + 1;
            self.csr.write(CYCLE, cycle);
            // update the time
            let time = SystemTime::now().duration_since(UNIX_EPOCH);
            if let Ok(t) = time {
                self.csr.write(TIME, t.as_secs());
            }
            // update the instret
            let instret = self.csr.read(INSTRET) + 1;
            self.csr.write(INSTRET, instret);
        }
        0
    }

    fn dump(&self, xsnapshot: [u64; 32], fsnapshot: [u64; 32]) {
        println!("{}", "-".repeat(90));
        println!("{}", self.x);
        println!("{}", "-".repeat(90));
        println!("{}", self.f);
        println!("{}", "-".repeat(90));
        let diffx = self.x.diff(xsnapshot);
        if !diffx.is_empty() {
            for d in diffx {
                println!("{:8}: {:x} -> {:x}", xname(d.0), d.1, d.2);
            }
            println!("{}", "-".repeat(90));
        }
        let difff = self.f.diff(fsnapshot);
        if !difff.is_empty() {
            for d in difff {
                println!("{:8}: {:x} -> {:x}", fname(d.0), d.1, d.2);
            }
            println!("{}", "-".repeat(90));
        }
        println!();
    }
}
