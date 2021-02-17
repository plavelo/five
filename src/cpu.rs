pub mod pc;
pub mod register;

use crate::{
    bus::SystemBus,
    cpu::{pc::ProgramCounter, register::Register},
    isa::instruction::{Instruction, OpcodeB, OpcodeI, OpcodeJ, OpcodeR, OpcodeS, OpcodeU},
    memory::Memory,
};

const MASK_5BIT: u32 = 0b11111;

#[allow(dead_code)]
pub struct Cpu {
    register: Register,
    pc: ProgramCounter,
    bus: SystemBus,
}

impl Cpu {
    pub fn new() -> Self {
        Self {
            register: Register::new(),
            pc: ProgramCounter::new(),
            bus: SystemBus::new(),
        }
    }

    pub fn cycle(&mut self) {
        while self.pc.read() < self.bus.memory.size() {
            let address = self.pc.read();
            let fetched = self.bus.memory.fetch(address);
            let decoded = Instruction::decode(fetched);
            if let Some(instruction) = decoded {
                execute(
                    instruction,
                    &mut self.pc,
                    &mut self.register,
                    &mut self.bus.memory,
                )
            }
        }
    }
}

fn execute(instruction: Instruction, pc: &mut ProgramCounter, x: &mut Register, mem: &mut Memory) {
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
            OpcodeI::Ecall => panic!("not implemented"),
            OpcodeI::Ebreak => panic!("not implemented"),
            OpcodeI::Csrrw => panic!("not implemented"),
            OpcodeI::Csrrs => panic!("not implemented"),
            OpcodeI::Csrrc => panic!("not implemented"),
            OpcodeI::Csrrwi => panic!("not implemented"),
            OpcodeI::Csrrsi => panic!("not implemented"),
            OpcodeI::Csrrci => panic!("not implemented"),
            OpcodeI::Lb => x.writei(
                rd,
                mem.load8(x.readi(rs1).wrapping_add(imm as i32) as u32) as i32,
            ),
            OpcodeI::Lh => x.writei(
                rd,
                mem.load16(x.readi(rs1).wrapping_add(imm as i32) as u32) as i32,
            ),
            OpcodeI::Lbu => x.writeu(
                rd,
                mem.load8(x.readi(rs1).wrapping_add(imm as i32) as u32) as u32,
            ),
            OpcodeI::Lhu => x.writeu(
                rd,
                mem.load16(x.readi(rs1).wrapping_add(imm as i32) as u32) as u32,
            ),
            OpcodeI::Lw => x.writeu(rd, mem.load32(x.readi(rs1).wrapping_add(imm as i32) as u32)),
        },
        Instruction::TypeS {
            opcode,
            rs1,
            rs2,
            imm,
        } => match opcode {
            OpcodeS::Sb => mem.store8((x.readi(rs1) + imm as i32) as u32, x.readu(rs2) as u8),
            OpcodeS::Sh => mem.store16((x.readi(rs1) + imm as i32) as u32, x.readu(rs2) as u16),
            OpcodeS::Sw => mem.store32((x.readi(rs1) + imm as i32) as u32, x.readu(rs2)),
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
