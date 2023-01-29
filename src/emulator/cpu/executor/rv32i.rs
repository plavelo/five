use crate::{
    emulator::{
        bus::SystemBus,
        cpu::{
            csr::ControlAndStatusRegister,
            executor::{extend_sign, Executor, MASK_5BIT, MASK_6BIT},
            f::FloatingPointRegister,
            pc::ProgramCounter,
            x::IntegerRegister,
        },
    },
    isa::{
        instruction::{
            rv32i::{
                Rv32iOpcodeB, Rv32iOpcodeI, Rv32iOpcodeJ, Rv32iOpcodeR, Rv32iOpcodeS, Rv32iOpcodeU,
            },
            Instruction,
        },
        privileged::{
            cause::{Cause, Exception},
            mode::PrivilegeMode,
        },
    },
};

pub struct Rv32iExecutor;

impl Executor for Rv32iExecutor {
    type OpcodeR = Rv32iOpcodeR;
    type OpcodeI = Rv32iOpcodeI;
    type OpcodeS = Rv32iOpcodeS;
    type OpcodeB = Rv32iOpcodeB;
    type OpcodeU = Rv32iOpcodeU;
    type OpcodeJ = Rv32iOpcodeJ;

    fn execute(
        instruction: Instruction<
            Rv32iOpcodeR,
            Rv32iOpcodeI,
            Rv32iOpcodeS,
            Rv32iOpcodeB,
            Rv32iOpcodeU,
            Rv32iOpcodeJ,
        >,
        prv: &PrivilegeMode,
        pc: &mut ProgramCounter,
        x: &mut IntegerRegister,
        _: &mut FloatingPointRegister,
        _: &mut ControlAndStatusRegister,
        bus: &mut SystemBus,
    ) -> Result<(), Cause> {
        match instruction {
            Instruction::TypeR {
                opcode,
                rd,
                funct3: _,
                rs1,
                rs2,
                funct7: _,
            } => match opcode {
                Rv32iOpcodeR::Sll => {
                    x.writeu(rd, x.readu(rs1) << (x.readu(rs2) & MASK_5BIT));
                    Ok(())
                }
                Rv32iOpcodeR::Srl => {
                    x.writeu(rd, x.readu(rs1) >> (x.readu(rs2) & MASK_5BIT));
                    Ok(())
                }
                Rv32iOpcodeR::Sra => {
                    x.writei(rd, x.readi(rs1) >> (x.readu(rs2) & MASK_5BIT));
                    Ok(())
                }
                Rv32iOpcodeR::Add => {
                    x.writeu(rd, x.readu(rs1).wrapping_add(x.readu(rs2)));
                    Ok(())
                }
                Rv32iOpcodeR::Sub => {
                    x.writeu(rd, x.readu(rs1).wrapping_sub(x.readu(rs2)));
                    Ok(())
                }
                Rv32iOpcodeR::Xor => {
                    x.writeu(rd, x.readu(rs1) ^ x.readu(rs2));
                    Ok(())
                }
                Rv32iOpcodeR::Or => {
                    x.writeu(rd, x.readu(rs1) | x.readu(rs2));
                    Ok(())
                }
                Rv32iOpcodeR::And => {
                    x.writeu(rd, x.readu(rs1) & x.readu(rs2));
                    Ok(())
                }
                Rv32iOpcodeR::Slt => {
                    x.writeu(rd, u64::from(x.readi(rs1) < x.readi(rs2)));
                    Ok(())
                }
                Rv32iOpcodeR::Sltu => {
                    x.writeu(rd, u64::from(x.readu(rs1) < x.readu(rs2)));
                    Ok(())
                }
            },
            Instruction::TypeI {
                opcode,
                rd,
                funct3: _,
                rs1,
                imm,
            } => match opcode {
                Rv32iOpcodeI::Slli => {
                    x.writeu(rd, x.readu(rs1) << (imm & MASK_6BIT));
                    Ok(())
                }
                Rv32iOpcodeI::Srli => {
                    x.writeu(rd, x.readu(rs1) >> (imm & MASK_6BIT));
                    Ok(())
                }
                Rv32iOpcodeI::Srai => {
                    x.writei(rd, x.readi(rs1) >> (imm & MASK_6BIT));
                    Ok(())
                }
                Rv32iOpcodeI::Addi => {
                    x.writeu(rd, x.readu(rs1).wrapping_add(imm));
                    Ok(())
                }
                Rv32iOpcodeI::Xori => {
                    x.writeu(rd, x.readu(rs1) ^ imm);
                    Ok(())
                }
                Rv32iOpcodeI::Ori => {
                    x.writeu(rd, x.readu(rs1) | imm);
                    Ok(())
                }
                Rv32iOpcodeI::Andi => {
                    x.writeu(rd, x.readu(rs1) & imm);
                    Ok(())
                }
                Rv32iOpcodeI::Slti => {
                    x.writeu(rd, u64::from(x.readi(rs1) < imm as i64));
                    Ok(())
                }
                Rv32iOpcodeI::Sltiu => {
                    x.writeu(rd, u64::from(x.readu(rs1) < imm));
                    Ok(())
                }
                Rv32iOpcodeI::Jalr => {
                    let last = pc.read();
                    pc.jump((x.readi(rs1).wrapping_add(imm as i64) & !1) as u64);
                    x.writeu(rd, last.wrapping_add(4));
                    Ok(())
                }
                Rv32iOpcodeI::Fence => Ok(()), // not yet supported
                Rv32iOpcodeI::Ecall => match prv {
                    PrivilegeMode::User => {
                        Err(Cause::Exception(Exception::EnvironmentCallFromUserMode))
                    }
                    PrivilegeMode::Supervisor => Err(Cause::Exception(
                        Exception::EnvironmentCallFromSupervisorMode,
                    )),
                    PrivilegeMode::Machine => {
                        Err(Cause::Exception(Exception::EnvironmentCallFromMachineMode))
                    }
                },
                Rv32iOpcodeI::Ebreak => Ok(()), // not yet supported
                Rv32iOpcodeI::Lb => {
                    x.writei(
                        rd,
                        bus.load8(x.readi(rs1).wrapping_add(imm as i64) as u64) as i64,
                    );
                    Ok(())
                }
                Rv32iOpcodeI::Lh => {
                    x.writei(
                        rd,
                        bus.load16(x.readi(rs1).wrapping_add(imm as i64) as u64) as i64,
                    );
                    Ok(())
                }
                Rv32iOpcodeI::Lbu => {
                    x.writeu(
                        rd,
                        bus.load8(x.readi(rs1).wrapping_add(imm as i64) as u64) as u64,
                    );
                    Ok(())
                }
                Rv32iOpcodeI::Lhu => {
                    x.writeu(
                        rd,
                        bus.load16(x.readi(rs1).wrapping_add(imm as i64) as u64) as u64,
                    );
                    Ok(())
                }
                Rv32iOpcodeI::Lw => {
                    x.writeu(
                        rd,
                        bus.load32(x.readi(rs1).wrapping_add(imm as i64) as u64) as u64,
                    );
                    Ok(())
                }
            },
            Instruction::TypeS {
                opcode,
                funct3: _,
                rs1,
                rs2,
                imm,
            } => match opcode {
                Rv32iOpcodeS::Sb => {
                    bus.store8(
                        x.readi(rs1).wrapping_add(imm as i64) as u64,
                        x.readu(rs2) as u8,
                    );
                    Ok(())
                }
                Rv32iOpcodeS::Sh => {
                    bus.store16(
                        x.readi(rs1).wrapping_add(imm as i64) as u64,
                        x.readu(rs2) as u16,
                    );
                    Ok(())
                }
                Rv32iOpcodeS::Sw => {
                    bus.store32(
                        x.readi(rs1).wrapping_add(imm as i64) as u64,
                        x.readu(rs2) as u32,
                    );
                    Ok(())
                }
            },
            Instruction::TypeB {
                opcode,
                funct3: _,
                rs1,
                rs2,
                imm,
            } => match opcode {
                Rv32iOpcodeB::Beq => {
                    if x.readu(rs1) == x.readu(rs2) {
                        pc.jumpr(imm as i64);
                    }
                    Ok(())
                }
                Rv32iOpcodeB::Bne => {
                    if x.readu(rs1) != x.readu(rs2) {
                        pc.jumpr(imm as i64);
                    }
                    Ok(())
                }
                Rv32iOpcodeB::Blt => {
                    if x.readi(rs1) < x.readi(rs2) {
                        pc.jumpr(imm as i64);
                    }
                    Ok(())
                }
                Rv32iOpcodeB::Bge => {
                    if x.readi(rs1) >= x.readi(rs2) {
                        pc.jumpr(imm as i64);
                    }
                    Ok(())
                }
                Rv32iOpcodeB::Bltu => {
                    if x.readu(rs1) < x.readu(rs2) {
                        pc.jumpr(imm as i64);
                    }
                    Ok(())
                }
                Rv32iOpcodeB::Bgeu => {
                    if x.readu(rs1) >= x.readu(rs2) {
                        pc.jumpr(imm as i64);
                    }
                    Ok(())
                }
            },
            Instruction::TypeU { opcode, rd, imm } => match opcode {
                Rv32iOpcodeU::Lui => {
                    x.writei(
                        rd,
                        ((extend_sign(imm, 20) << 12) as u64 & 0xfffffffffffff000) as i64,
                    );
                    Ok(())
                }
                Rv32iOpcodeU::Auipc => {
                    x.writei(
                        rd,
                        (pc.read() as i64).wrapping_add(extend_sign(imm, 20) << 12),
                    );
                    Ok(())
                }
            },
            Instruction::TypeJ { opcode, rd, imm } => match opcode {
                Rv32iOpcodeJ::Jal => {
                    x.writeu(rd, pc.read().wrapping_add(4));
                    pc.jumpr(imm as i64);
                    Ok(())
                }
            },
        }
    }
}
