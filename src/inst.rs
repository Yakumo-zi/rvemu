#[derive(Clone, Copy, Debug)]
pub enum BinaryInstruction {
    I {
        opcode: u8,
        rd: u8,
        funct3: u8,
        rs: u8,
        imm: u16,
    },
    U {
        opcode: u8,
        rd: u8,
        imm: u32,
    },
    R {
        opcode: u8,
        rd: u8,
        funct3: u8,
        rs1: u8,
        rs2: u8,
        funct7: u8,
    },
    S {
        opcode: u8,
        imm1: u8,
        funct3: u8,
        rs1: u8,
        rs2: u8,
        imm2: u8,
    },
    B {
        opcode: u8,
        imm1: u8,
        funct3: u8,
        rs1: u8,
        rs2: u8,
        imm2: u8,
    },
    J {
        opcode: u8,
        rd: u8,
        imm: u32,
    },
    Unkown,
}

impl BinaryInstruction {
    pub fn parse(ins: u32) -> BinaryInstruction {
        let opcode = (ins) & 0b1111111;
        match opcode {
            0b0010011 | 0b0000011 | 0b1100111 | 0b1110011 => {
                let rd = (ins >> 7) & 0b11111;
                let funct3 = (ins >> 12) & 0b111;
                let rs = (ins >> 15) & 0b11111;
                let imm = ins >> 20;
                BinaryInstruction::I {
                    opcode: opcode as u8,
                    rd: rd as u8,
                    funct3: funct3 as u8,
                    rs: rs as u8,
                    imm: imm as u16,
                }
            }
            0b0010111 | 0b110111 => {
                let rd = (ins >> 7) & 0b11111;
                let imm = ins >> 12;
                BinaryInstruction::U {
                    opcode: opcode as u8,
                    rd: rd as u8,
                    imm: imm,
                }
            }
            0b0100011 => {
                let imm1 = (ins >> 7) & 0b11111;
                let funct3 = (ins >> 12) & 0b111;
                let rs1 = (ins >> 15) & 0b11111;
                let rs2 = (ins >> 20) & 0b11111;
                let imm2 = ins >> 25;
                BinaryInstruction::S {
                    opcode: opcode as u8,
                    imm1: imm1 as u8,
                    funct3: funct3 as u8,
                    rs1: rs1 as u8,
                    rs2: rs2 as u8,
                    imm2: imm2 as u8,
                }
            }
            0b1100011=>{
                let imm1 = (ins >> 7) & 0b11111;
                let funct3 = (ins >> 12) & 0b111;
                let rs1 = (ins >> 15) & 0b11111;
                let rs2 = (ins >> 20) & 0b11111;
                let imm2 = ins >> 25;
                BinaryInstruction::B {
                    opcode: opcode as u8,
                    imm1: imm1 as u8,
                    funct3: funct3 as u8,
                    rs1: rs1 as u8,
                    rs2: rs2 as u8,
                    imm2: imm2 as u8,
                }

            }
            0b1101111 => {
                let rd = (ins >> 7) & 0b11111;
                let imm = ins >> 12;
                BinaryInstruction::J {
                    opcode: opcode as u8,
                    rd: rd as u8,
                    imm: imm,
                }
            }
            0b0110011 => {
                let rd = (ins >> 7) & 0b11111;
                let funct3 = (ins >> 12) & 0b111;
                let rs1 = (ins >> 15) & 0b11111;
                let rs2 = (ins >> 20) & 0b11111;
                let funct7 = ins >> 25;
                BinaryInstruction::R {
                    opcode: opcode as u8,
                    rd: rd as u8,
                    funct3: funct3 as u8,
                    rs1: rs1 as u8,
                    rs2: rs2 as u8,
                    funct7: funct7 as u8,
                }
            }

            _ => BinaryInstruction::Unkown,
        }
    }
}
