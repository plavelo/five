pub mod csr;
pub mod pc;
pub mod x;

use crate::{
    emulator::{
        bus::SystemBus,
        cpu::{
            csr::ControlAndStatusRegister,
            pc::ProgramCounter,
            x::{IntegerRegister, GP},
        },
    },
    isa::instruction::{Instruction, OpcodeB, OpcodeI, OpcodeJ, OpcodeR, OpcodeS, OpcodeU},
};

const MASK_5BIT: u32 = 0b11111;
const MASK_12BIT: u32 = 0b111111111111;

#[allow(dead_code)]
#[derive(Default)]
pub struct Cpu {
    x: IntegerRegister,
    pc: ProgramCounter,
    csr: ControlAndStatusRegister,
    pub bus: SystemBus,
}

impl Cpu {
    pub fn run(&mut self) -> u32 {
        let mut counter = 0;
        while self.pc.read() < self.bus.memory.size() && !self.bus.is_finished && counter < 65536 {
            let address = self.pc.read();
            let fetched = self.bus.load32(address);
            let decoded = Instruction::decode(fetched);
            if let Some(instruction) = decoded {
                execute(
                    instruction,
                    &mut self.pc,
                    &mut self.x,
                    &mut self.csr,
                    &mut self.bus,
                )
            }
            self.pc.increment();
            counter += 1;
        }
        self.x.readu(GP)
    }
}

fn execute(
    instruction: Instruction,
    pc: &mut ProgramCounter,
    x: &mut IntegerRegister,
    csr: &mut ControlAndStatusRegister,
    bus: &mut SystemBus,
) {
    match instruction {
        Instruction::TypeR {
            opcode,
            rs1,
            rs2,
            rd,
        } => match opcode {
            OpcodeR::Sll => x.writeu(rd, x.readu(rs1) << (x.readu(rs2) & MASK_5BIT)),
            OpcodeR::Srl => x.writeu(rd, x.readu(rs1) >> (x.readu(rs2) & MASK_5BIT)),
            OpcodeR::Sra => x.writei(rd, x.readi(rs1) >> (x.readu(rs2) & MASK_5BIT)),
            OpcodeR::Add => x.writeu(rd, x.readu(rs1).wrapping_add(x.readu(rs2))),
            OpcodeR::Sub => x.writeu(rd, x.readu(rs1).wrapping_sub(x.readu(rs2))),
            OpcodeR::Xor => x.writeu(rd, x.readu(rs1) ^ x.readu(rs2)),
            OpcodeR::Or => x.writeu(rd, x.readu(rs1) | x.readu(rs2)),
            OpcodeR::And => x.writeu(rd, x.readu(rs1) & x.readu(rs2)),
            OpcodeR::Slt => x.writeu(rd, if x.readi(rs1) < x.readi(rs2) { 1 } else { 0 }),
            OpcodeR::Sltu => x.writeu(rd, if x.readu(rs1) < x.readu(rs2) { 1 } else { 0 }),
        },
        Instruction::TypeI {
            opcode,
            rs1,
            rd,
            imm,
        } => match opcode {
            OpcodeI::Slli => x.writeu(rd, x.readu(rs1) << (imm & MASK_5BIT)),
            OpcodeI::Srli => x.writeu(rd, x.readu(rs1) >> (imm & MASK_5BIT)),
            OpcodeI::Srai => x.writei(rd, x.readi(rs1) >> (imm & MASK_5BIT)),
            OpcodeI::Addi => x.writeu(rd, x.readu(rs1).wrapping_add(imm)),
            OpcodeI::Xori => x.writeu(rd, x.readu(rs1) ^ imm),
            OpcodeI::Ori => x.writeu(rd, x.readu(rs1) | imm),
            OpcodeI::Andi => x.writeu(rd, x.readu(rs1) & imm),
            OpcodeI::Slti => x.writeu(rd, if x.readi(rs1) < imm as i32 { 1 } else { 0 }),
            OpcodeI::Sltiu => x.writeu(rd, if x.readu(rs1) < imm { 1 } else { 0 }),
            OpcodeI::Jalr => {
                let last = pc.read();
                pc.jump((x.readi(rs1).wrapping_add(imm as i32) & !1) as u32);
                x.writeu(rd, last + 4);
            }
            OpcodeI::Fence => {}  // not yet supported
            OpcodeI::FenceI => {} // not yet supported
            OpcodeI::Ecall => {}  // not yet supported
            OpcodeI::Ebreak => {} // not yet supported
            OpcodeI::Csrrw => x.writeu(rd, csr.csrrw(imm & MASK_12BIT, x.readu(rs1))),
            OpcodeI::Csrrs => x.writeu(rd, csr.csrrs(imm & MASK_12BIT, x.readu(rs1))),
            OpcodeI::Csrrc => x.writeu(rd, csr.csrrc(imm & MASK_12BIT, x.readu(rs1))),
            OpcodeI::Csrrwi => x.writeu(rd, csr.csrrw(imm & MASK_12BIT, rs1 as u32)),
            OpcodeI::Csrrsi => x.writeu(rd, csr.csrrs(imm & MASK_12BIT, rs1 as u32)),
            OpcodeI::Csrrci => x.writeu(rd, csr.csrrc(imm & MASK_12BIT, rs1 as u32)),
            OpcodeI::Lb => x.writei(
                rd,
                bus.load8(x.readi(rs1).wrapping_add(imm as i32) as u32) as i32,
            ),
            OpcodeI::Lh => x.writei(
                rd,
                bus.load16(x.readi(rs1).wrapping_add(imm as i32) as u32) as i32,
            ),
            OpcodeI::Lbu => x.writeu(
                rd,
                bus.load8(x.readi(rs1).wrapping_add(imm as i32) as u32) as u32,
            ),
            OpcodeI::Lhu => x.writeu(
                rd,
                bus.load16(x.readi(rs1).wrapping_add(imm as i32) as u32) as u32,
            ),
            OpcodeI::Lw => x.writeu(rd, bus.load32(x.readi(rs1).wrapping_add(imm as i32) as u32)),
        },
        Instruction::TypeS {
            opcode,
            rs1,
            rs2,
            imm,
        } => match opcode {
            OpcodeS::Sb => bus.store8((x.readi(rs1) + imm as i32) as u32, x.readu(rs2) as u8),
            OpcodeS::Sh => bus.store16((x.readi(rs1) + imm as i32) as u32, x.readu(rs2) as u16),
            OpcodeS::Sw => bus.store32((x.readi(rs1) + imm as i32) as u32, x.readu(rs2)),
        },
        Instruction::TypeB {
            opcode,
            rs1,
            rs2,
            imm,
        } => match opcode {
            OpcodeB::Beq => {
                if x.readu(rs1) == x.readu(rs2) {
                    pc.jumpr(imm as i32);
                }
            }
            OpcodeB::Bne => {
                if x.readu(rs1) != x.readu(rs2) {
                    pc.jumpr(imm as i32);
                }
            }
            OpcodeB::Blt => {
                if x.readi(rs1) < x.readi(rs2) {
                    pc.jumpr(imm as i32);
                }
            }
            OpcodeB::Bge => {
                if x.readi(rs1) >= x.readi(rs2) {
                    pc.jumpr(imm as i32);
                }
            }
            OpcodeB::Bltu => {
                if x.readu(rs1) < x.readu(rs2) {
                    pc.jumpr(imm as i32);
                }
            }
            OpcodeB::Bgeu => {
                if x.readu(rs1) >= x.readu(rs2) {
                    pc.jumpr(imm as i32);
                }
            }
        },
        Instruction::TypeU { opcode, rd, imm } => match opcode {
            OpcodeU::Lui => x.writeu(rd, imm),
            OpcodeU::Auipc => x.writeu(rd, pc.read().wrapping_add(imm)),
        },
        Instruction::TypeJ { opcode, rd, imm } => match opcode {
            OpcodeJ::Jal => {
                x.writeu(rd, pc.read() + 4);
                pc.jumpr(imm as i32);
            }
        },
    }
}
