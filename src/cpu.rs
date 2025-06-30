use crate::inst;

#[derive(Clone, Copy, Debug)]
pub struct Reg<'a> {
    pub name: &'a str,
    pub abi_name: &'a str,
    pub value: u32,
}
#[derive(Debug)]
pub struct Cpu<'a> {
    pub regs: [Reg<'a>; 32],
    pub program: Vec<u32>,
    pc: usize,
}

impl<'a> Cpu<'a> {
    pub fn new() -> Cpu<'a> {
        let regs: [Reg; 32] = [
            Reg {
                name: "x0",
                abi_name: "zero",
                value: 0,
            },
            Reg {
                name: "x1",
                abi_name: "ra",
                value: 0,
            },
            Reg {
                name: "x2",
                abi_name: "sp",
                value: 1024 * 128,
            },
            Reg {
                name: "x3",
                abi_name: "gp",
                value: 0,
            },
            Reg {
                name: "x4",
                abi_name: "tp",
                value: 0,
            },
            Reg {
                name: "x5",
                abi_name: "t0",
                value: 0,
            },
            Reg {
                name: "x6",
                abi_name: "t1",
                value: 0,
            },
            Reg {
                name: "x7",
                abi_name: "t2",
                value: 0,
            },
            Reg {
                name: "x8",
                abi_name: "s0",
                value: 0,
            },
            Reg {
                name: "x9",
                abi_name: "s1",
                value: 0,
            },
            Reg {
                name: "x10",
                abi_name: "a0",
                value: 0,
            },
            Reg {
                name: "x11",
                abi_name: "a1",
                value: 0,
            },
            Reg {
                name: "x12",
                abi_name: "a2",
                value: 0,
            },
            Reg {
                name: "x13",
                abi_name: "a3",
                value: 0,
            },
            Reg {
                name: "x14",
                abi_name: "a4",
                value: 0,
            },
            Reg {
                name: "x15",
                abi_name: "a5",
                value: 0,
            },
            Reg {
                name: "x16",
                abi_name: "a6",
                value: 0,
            },
            Reg {
                name: "x17",
                abi_name: "a7",
                value: 0,
            },
            Reg {
                name: "x18",
                abi_name: "s2",
                value: 0,
            },
            Reg {
                name: "x19",
                abi_name: "s3",
                value: 0,
            },
            Reg {
                name: "x20",
                abi_name: "s4",
                value: 0,
            },
            Reg {
                name: "x21",
                abi_name: "s5",
                value: 0,
            },
            Reg {
                name: "x22",
                abi_name: "s6",
                value: 0,
            },
            Reg {
                name: "x23",
                abi_name: "s7",
                value: 0,
            },
            Reg {
                name: "x24",
                abi_name: "s8",
                value: 0,
            },
            Reg {
                name: "x25",
                abi_name: "s9",
                value: 0,
            },
            Reg {
                name: "x26",
                abi_name: "s10",
                value: 0,
            },
            Reg {
                name: "x27",
                abi_name: "s11",
                value: 0,
            },
            Reg {
                name: "x28",
                abi_name: "t3",
                value: 0,
            },
            Reg {
                name: "x29",
                abi_name: "t4",
                value: 0,
            },
            Reg {
                name: "x30",
                abi_name: "t5",
                value: 0,
            },
            Reg {
                name: "x31",
                abi_name: "t6",
                value: 0,
            },
        ];
        return Cpu {
            regs,
            program: Vec::new(),
            pc: 0,
        };
    }
    pub fn load_program(&mut self, prog: Vec<u32>) {
        self.program = prog;
    }
    pub fn fetch(&mut self) -> u32 {
        let ins = self.program.get(self.pc).unwrap().clone();
        self.pc += 1;
        return ins;
    }

    pub fn decode(&self, ins: u32) -> inst::BinaryInstruction {
        inst::BinaryInstruction::parse(ins)
    }
    pub fn execute(&mut self, ins: inst::BinaryInstruction) {
        match ins {
            inst::BinaryInstruction::I {
                opcode,
                rd,
                funct3,
                rs,
                imm,
            } => {
                let rd = rd as usize;
                let rs = rs as usize;
                match opcode {
                    0b0010011 => {
                        match funct3 {
                            //addi
                            0x0 => {
                                self.regs[rd].value = self.regs[rs].value + imm as u32;
                            }
                            //xori
                            0x4 => {
                                self.regs[rd].value = self.regs[rs].value ^ imm as u32;
                            }
                            //ori
                            0x6 => {
                                self.regs[rd].value = self.regs[rs].value | imm as u32;
                            }
                            //andi
                            0x7 => {
                                self.regs[rd].value = self.regs[rs].value & imm as u32;
                            }
                            0x1 => {
                                let shift_bits_num = imm & 0b11111;
                                let funct7 = imm >> 5;
                                match funct7 {
                                    //slli
                                    0x00 => {
                                        self.regs[rd].value = self.regs[rs].value << shift_bits_num;
                                    }
                                    _ => {}
                                }
                            }
                            0x5 => {
                                let shift_bits_num = imm & 0b11111;
                                let funct7 = imm >> 5;
                                match funct7 {
                                    //srli
                                    0x00 => {
                                        self.regs[rd].value = self.regs[rs].value >> shift_bits_num;
                                    }
                                    //srai - msb-extension
                                    0x20 => {
                                        self.regs[rd].value =
                                            (self.regs[rs].value as i32 >> shift_bits_num) as u32;
                                    }
                                    _ => {}
                                }
                            }
                            //slti
                            0x2 => {
                                let rs1_value = self.regs[rs].value;
                                self.regs[rd].value = if rs1_value < imm as u32 { 1 } else { 0 }
                            }
                            //sltiu - zero-extension
                            0x3 => {
                                let rs1_value = self.regs[rs].value;
                                self.regs[rd].value = if rs1_value < imm as u32 { 1 } else { 0 }
                            }
                            _ => {}
                        }
                    }
                    _ => {}
                }
            }
            inst::BinaryInstruction::U { opcode, rd, imm } => todo!(),
            inst::BinaryInstruction::R {
                opcode,
                rd,
                funct3,
                rs1,
                rs2,
                funct7,
            } => todo!(),
            inst::BinaryInstruction::S {
                opcode,
                imm1,
                funct3,
                rs1,
                rs2,
                imm2,
            } => todo!(),
            inst::BinaryInstruction::B {
                opcode,
                imm1,
                funct3,
                rs1,
                rs2,
                imm2,
            } => todo!(),
            inst::BinaryInstruction::J { opcode, rd, imm } => todo!(),
            inst::BinaryInstruction::Unkown => todo!(),
        }
    }
}
