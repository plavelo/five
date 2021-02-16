struct Decoded {
    opcode: u8,
    funct3: u8,
    funct7: u8,
    rs1: u8,
    rs2: u8,
    rd: u8,
    imm: u32,
}

const MASK_7BIT = 0b1111111u32;
const MASK_5BIT = 0b11111u32;

fn decode(&self, instruction: u32) -> Decoded {
    let opcode = instruction & MASK_7BIT;
    let rd = (instruction >> 7) & MASK_5BIT;
    let rs1 = (instruction >> 15) & MASK_5BIT;
    let rs2 = (instruction >> 20) & MASK_5BIT;
    
}
