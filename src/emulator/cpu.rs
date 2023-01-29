mod csr;
mod decoder;
mod executor;
mod f;
mod pc;
mod trap_handler;
mod x;

use crate::{
    emulator::{
        bus::SystemBus,
        cpu::{
            csr::ControlAndStatusRegister,
            decoder::{
                privileged::PrivilegedDecoder, rv32f::Rv32fDecoder, rv32i::Rv32iDecoder,
                rv32m::Rv32mDecoder, rv64i::Rv64iDecoder, rv64m::Rv64mDecoder, zicsr::ZicsrDecoder,
                zifencei::ZifenceiDecoder, Decoder,
            },
            executor::{
                privileged::PrivilegedExecutor, rv32f::Rv32fExecutor, rv32i::Rv32iExecutor,
                rv32m::Rv32mExecutor, rv64i::Rv64iExecutor, rv64m::Rv64mExecutor,
                zicsr::ZicsrExecutor, zifencei::ZifenceiExecutor, Executor,
            },
            f::FloatingPointRegister,
            pc::ProgramCounter,
            trap_handler::*,
            x::IntegerRegister,
        },
    },
    isa::{
        description::Describer,
        privileged::mode::PrivilegeMode,
        register::{register_name, A0},
    },
};

#[derive(Default)]
pub struct Cpu {
    x: IntegerRegister,
    f: FloatingPointRegister,
    pc: ProgramCounter,
    csr: ControlAndStatusRegister,
    prv: PrivilegeMode,
    pub bus: SystemBus,
}

impl Cpu {
    pub fn run(&mut self) -> u64 {
        while self.pc.read() < self.bus.memory.size() {
            let snapshot = self.x.snapshot();
            // read an address from the pc
            let address = self.pc.read();
            // fetch an instruction
            let instruction = self.bus.load32(address);
            // decode and execute the instruction
            let result = if let Some(decoded) = PrivilegedDecoder::decode(instruction) {
                println!("{:x}: {}", address, decoded.describe());
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
                println!("{:x}: {}", address, decoded.describe());
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
                println!("{:x}: {}", address, decoded.describe());
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
                println!("{:x}: {}", address, decoded.describe());
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
                println!("{:x}: {}", address, decoded.describe());
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
                println!("{:x}: {}", address, decoded.describe());
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
                println!("{:x}: {}", address, decoded.describe());
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
                println!("{:x}: {}", address, decoded.describe());
                Rv32fExecutor::execute(
                    decoded,
                    &self.prv,
                    &mut self.pc,
                    &mut self.x,
                    &mut self.f,
                    &mut self.csr,
                    &mut self.bus,
                )
            } else {
                // end the loop when unable to decode the instruction
                break;
            };
            self.dump(snapshot);

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
        }
        self.x.readu(A0)
    }

    fn dump(&self, snapshot: [u64; 32]) {
        println!("\n{}", self.x);
        let diff = self.x.diff(snapshot);
        if !diff.is_empty() {
            println!("\ndiff:");

            for d in diff {
                println!("\t{:8}: {:x} -> {:x}", register_name(d.0), d.1, d.2);
            }
        }
        println!("\n---------------------------\n");
    }
}
