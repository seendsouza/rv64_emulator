#[derive(Debug, PartialEq)]
pub enum Register {
    X0,
    X1,
    X2,
    X3,
    X4,
    X5,
    X6,
    X7,
    X8,
    X9,
    X10,
    X11,
    X12,
    X13,
    X14,
    X15,
    X16,
    X17,
    X18,
    X19,
    X20,
    X21,
    X22,
    X23,
    X24,
    X25,
    X26,
    X27,
    X28,
    X29,
    X30,
    X31,
    PC,
}
impl From<Register> for usize {
    fn from(reg: Register) -> usize {
        reg as usize
    }
}
impl From<usize> for Register {
    fn from(val: usize) -> Register {
        match val {
            0 => Register::X0,
            1 => Register::X1,
            2 => Register::X2,
            3 => Register::X3,
            4 => Register::X4,
            5 => Register::X5,
            6 => Register::X6,
            7 => Register::X7,
            8 => Register::X8,
            9 => Register::X9,
            10 => Register::X10,
            11 => Register::X11,
            12 => Register::X12,
            13 => Register::X13,
            14 => Register::X14,
            15 => Register::X15,
            16 => Register::X16,
            17 => Register::X17,
            18 => Register::X18,
            19 => Register::X19,
            20 => Register::X20,
            21 => Register::X21,
            22 => Register::X22,
            23 => Register::X23,
            24 => Register::X24,
            25 => Register::X25,
            26 => Register::X26,
            27 => Register::X27,
            28 => Register::X28,
            29 => Register::X29,
            30 => Register::X30,
            31 => Register::X31,
            _ => unreachable!(),
        }
    }
}
// Decode a RISC-V 64 isntruction and return the instruction
pub fn decode(instruction: u32) -> Instruction {
    let opcode = instruction & 0b1111111;
    if let Some(instruction_format) = &ENCODING_TABLE[opcode as usize] {
        instruction_format.decode(instruction)
    } else {
        Instruction::Undefined
    }
}

#[derive(Debug)]
pub enum Instruction {
    Undefined,

    // B-Type
    Beq {
        rs1: Register,
        rs2: Register,
        imm: i32,
    },
    Bne {
        rs1: Register,
        rs2: Register,
        imm: i32,
    },
    Blt {
        rs1: Register,
        rs2: Register,
        imm: i32,
    },
    Bge {
        rs1: Register,
        rs2: Register,
        imm: i32,
    },
    Bltu {
        rs1: Register,
        rs2: Register,
        imm: i32,
    },
    Bgeu {
        rs1: Register,
        rs2: Register,
        imm: i32,
    },

    // I-Type
    Lb {
        rd: Register,
        rs1: Register,
        imm: i32,
    },
    Lh {
        rd: Register,
        rs1: Register,
        imm: i32,
    },
    Lw {
        rd: Register,
        rs1: Register,
        imm: i32,
    },
    Lbu {
        rd: Register,
        rs1: Register,
        imm: i32,
    },
    Lhu {
        rd: Register,
        rs1: Register,
        imm: i32,
    },
    Lwu {
        rd: Register,
        rs1: Register,
        imm: i32,
    },
    Ld {
        rd: Register,
        rs1: Register,
        imm: i32,
    },

    Fence {
        rd: Register,
        rs1: Register,
        succ: u32,
        pred: u32,
        fm: u32,
    },

    Addi {
        rd: Register,
        rs1: Register,
        imm: i32,
    },
    Slti {
        rd: Register,
        rs1: Register,
        imm: i32,
    },
    Sltiu {
        rd: Register,
        rs1: Register,
        imm: i32,
    },
    Xori {
        rd: Register,
        rs1: Register,
        imm: i32,
    },
    Ori {
        rd: Register,
        rs1: Register,
        imm: i32,
    },
    Andi {
        rd: Register,
        rs1: Register,
        imm: i32,
    },
    Slli {
        rd: Register,
        rs1: Register,
        shamt: u32,
    },
    Srli {
        rd: Register,
        rs1: Register,
        shamt: u32,
    },
    Srai {
        rd: Register,
        rs1: Register,
        shamt: u32,
    },
    Addiw {
        rd: Register,
        rs1: Register,
        imm: i32,
    },
    Slliw {
        rd: Register,
        rs1: Register,
        shamt: u32,
    },
    Srliw {
        rd: Register,
        rs1: Register,
        shamt: u32,
    },
    Sraiw {
        rd: Register,
        rs1: Register,
        shamt: u32,
    },

    Jalr {
        rd: Register,
        rs1: Register,
        imm: i32,
    },

    Ebreak,
    Ecall,

    // J-Type
    Jal {
        rd: Register,
        imm: i32,
    },

    // R-Type
    Add {
        rd: Register,
        rs1: Register,
        rs2: Register,
    },
    Sub {
        rd: Register,
        rs1: Register,
        rs2: Register,
    },
    Sll {
        rd: Register,
        rs1: Register,
        rs2: Register,
    },
    Slt {
        rd: Register,
        rs1: Register,
        rs2: Register,
    },
    Sltu {
        rd: Register,
        rs1: Register,
        rs2: Register,
    },
    Xor {
        rd: Register,
        rs1: Register,
        rs2: Register,
    },
    Srl {
        rd: Register,
        rs1: Register,
        rs2: Register,
    },
    Sra {
        rd: Register,
        rs1: Register,
        rs2: Register,
    },
    Or {
        rd: Register,
        rs1: Register,
        rs2: Register,
    },
    And {
        rd: Register,
        rs1: Register,
        rs2: Register,
    },
    Addw {
        rd: Register,
        rs1: Register,
        rs2: Register,
    },
    Subw {
        rd: Register,
        rs1: Register,
        rs2: Register,
    },
    Sllw {
        rd: Register,
        rs1: Register,
        rs2: Register,
    },
    Srlw {
        rd: Register,
        rs1: Register,
        rs2: Register,
    },
    Sraw {
        rd: Register,
        rs1: Register,
        rs2: Register,
    },

    // S-Type
    Sb {
        rs1: Register,
        rs2: Register,
        imm: i32,
    },
    Sh {
        rs1: Register,
        rs2: Register,
        imm: i32,
    },
    Sw {
        rs1: Register,
        rs2: Register,
        imm: i32,
    },
    Sd {
        rs1: Register,
        rs2: Register,
        imm: i32,
    },

    // U-Type
    Auipc {
        rd: Register,
        imm: i32,
    },
    Lui {
        rd: Register,
        imm: i32,
    },
}
enum InstructionFormat {
    R,
    I,
    S,
    B,
    U,
    J,
}

impl InstructionFormat {
    fn decode(&self, instruction: u32) -> Instruction {
        let opcode = instruction & 0b11111111;
        match self {
            InstructionFormat::B => {
                // Decode fields
                let imm12105 = (instruction >> 25) & 0b1111111;
                let rs2 = (((instruction >> 20) & 0b11111) as usize).into();
                let rs1 = (((instruction >> 15) & 0b11111) as usize).into();
                let funct3 = (instruction >> 12) & 0b11;
                let imm4111 = (instruction >> 7) & 0b11111;

                // Split the immediate
                let imm12 = (imm12105 & 0b1000000) >> 6;
                let imm105 = imm12105 & 0b0111111;
                let imm41 = (imm4111 & 0b11110) >> 1;
                let imm11 = imm4111 & 0b00001;

                // Merge and sign extend the immediate
                let imm = (imm12 << 12) | (imm11 << 11) | (imm105 << 5) | (imm41 << 1);
                let imm = ((imm as i32) << 19) >> 19;

                match funct3 {
                    0b000 => Instruction::Beq { rs1, rs2, imm },
                    0b001 => Instruction::Bne { rs1, rs2, imm },
                    0b100 => Instruction::Blt { rs1, rs2, imm },
                    0b101 => Instruction::Bge { rs1, rs2, imm },
                    0b110 => Instruction::Bltu { rs1, rs2, imm },
                    0b111 => Instruction::Bgeu { rs1, rs2, imm },
                    _ => Instruction::Undefined,
                }
            }

            InstructionFormat::I => {
                // Decode fields
                let imm = (instruction >> 20) & 0b1111_1111_1111;
                let rs1 = (((instruction >> 15) & 0b11111) as usize).into();
                let funct3 = (instruction >> 12) & 0b11;
                let rd = (((instruction >> 7) & 0b11111) as usize).into();

                // Shifts are encoded as a specialization of the I-type format
                // Shift amount field for Slli, Srli and Srai
                let shamt = (imm & 0b111111) as u32;
                // Shift amount field for Slliw, Srliw and Sraiw
                let shamtw = (imm & 0b11111) as u32;
                // 30th bit contains right shift type
                let rshift_type = ((imm & (1 << 30)) >> 30) as u32;

                // Sign extend the immediate
                let imm = ((imm as i32) << 20) >> 20;

                match opcode {
                    0b0000011 => match funct3 {
                        0b000 => Instruction::Lb { rd, rs1, imm },
                        0b001 => Instruction::Lh { rd, rs1, imm },
                        0b010 => Instruction::Lw { rd, rs1, imm },
                        0b100 => Instruction::Lbu { rd, rs1, imm },
                        0b101 => Instruction::Lhu { rd, rs1, imm },
                        0b110 => Instruction::Lwu { rd, rs1, imm },
                        0b011 => Instruction::Ld { rd, rs1, imm },
                        _ => Instruction::Undefined,
                    },
                    0b0001111 => match funct3 {
                        0b000 => Instruction::Fence {
                            // TODO: fix this
                            rd,
                            rs1,
                            succ: (0),
                            pred: (0),
                            fm: (0),
                        },
                        _ => Instruction::Undefined,
                    },
                    0b0010011 => match funct3 {
                        0b000 => Instruction::Addi { rd, rs1, imm },
                        0b010 => Instruction::Slti { rd, rs1, imm },
                        0b011 => Instruction::Sltiu { rd, rs1, imm },
                        0b100 => Instruction::Xori { rd, rs1, imm },
                        0b110 => Instruction::Ori { rd, rs1, imm },
                        0b111 => Instruction::Andi { rd, rs1, imm },
                        0b001 => Instruction::Slli { rd, rs1, shamt },
                        0b101 if rshift_type == 0 => Instruction::Srli { rd, rs1, shamt },
                        0b101 if rshift_type == 1 => Instruction::Srai { rd, rs1, shamt },
                        _ => Instruction::Undefined,
                    },
                    0b0011011 => match funct3 {
                        0b000 => Instruction::Addiw { rd, rs1, imm },
                        0b001 => Instruction::Slliw {
                            rd,
                            rs1,
                            shamt: shamtw,
                        },
                        0b101 if rshift_type == 0 => Instruction::Srliw {
                            rd,
                            rs1,
                            shamt: shamtw,
                        },
                        0b101 if rshift_type == 1 => Instruction::Sraiw {
                            rd,
                            rs1,
                            shamt: shamtw,
                        },
                        _ => Instruction::Undefined,
                    },
                    0b100111 => match funct3 {
                        0b000 => Instruction::Jalr { rd, rs1, imm },
                        _ => Instruction::Undefined,
                    },
                    0b1110011 => match funct3 {
                        0b000 if imm == 0 && rs1 == Register::X0 && rd == Register::X0 => {
                            Instruction::Ecall
                        }
                        0b000 if imm == 1 && rs1 == Register::X0 && rd == Register::X0 => {
                            Instruction::Ebreak
                        }
                        _ => Instruction::Undefined,
                    },
                    _ => Instruction::Undefined,
                }
            }

            InstructionFormat::J => {
                // Decode fields
                let imm = ((instruction & 0xfffff000) as i32) >> 12;
                let rd = (((instruction >> 7) & 0b11111) as usize).into();

                // Split the immediate
                let imm20 = (imm >> 19) & 1;
                let imm101 = (imm >> 9) & 0b11111111111;
                let imm11 = (imm >> 8) & 1;
                let imm1912 = (imm >> 0) & 0b00000000;

                // Merge and sign extend the immediate
                let imm = (imm20 << 20) | (imm1912 << 12) | (imm11 << 11) | (imm101 << 1);
                let imm = ((imm as i32) << 11) >> 11;

                match opcode {
                    0b1101111 => Instruction::Jal { rd, imm },
                    _ => Instruction::Undefined,
                }
            }

            InstructionFormat::R => {
                // Decode fields
                let funct7 = (instruction >> 25) & 0b1111111;
                let rs2 = (((instruction >> 20) & 0b11111) as usize).into();
                let rs1 = (((instruction >> 15) & 0b11111) as usize).into();
                let funct3 = (instruction >> 12) & 0b11;
                let rd = (((instruction >> 7) & 0b11111) as usize).into();

                match opcode {
                    0b0110011 => match (funct7, funct3) {
                        (0b0000000, 0b000) => Instruction::Add { rd, rs1, rs2 },
                        (0b0100000, 0b000) => Instruction::Sub { rd, rs1, rs2 },
                        (0b0000000, 0b001) => Instruction::Sll { rd, rs1, rs2 },
                        (0b0000000, 0b010) => Instruction::Slt { rd, rs1, rs2 },
                        (0b0000000, 0b011) => Instruction::Sltu { rd, rs1, rs2 },
                        (0b0000000, 0b100) => Instruction::Xor { rd, rs1, rs2 },
                        (0b0000000, 0b101) => Instruction::Srl { rd, rs1, rs2 },
                        (0b0100000, 0b101) => Instruction::Sra { rd, rs1, rs2 },
                        (0b0000000, 0b110) => Instruction::Or { rd, rs1, rs2 },
                        (0b0000000, 0b111) => Instruction::And { rd, rs1, rs2 },
                        _ => Instruction::Undefined,
                    },
                    0b0111011 => match (funct7, funct3) {
                        (0b0000000, 0b000) => Instruction::Addw { rd, rs1, rs2 },
                        (0b0100000, 0b000) => Instruction::Subw { rd, rs1, rs2 },
                        (0b0000000, 0b001) => Instruction::Sllw { rd, rs1, rs2 },
                        (0b0000000, 0b101) => Instruction::Srlw { rd, rs1, rs2 },
                        (0b0100000, 0b101) => Instruction::Sraw { rd, rs1, rs2 },
                        _ => Instruction::Undefined,
                    },
                    _ => Instruction::Undefined,
                }
            }

            InstructionFormat::S => {
                // Decode fields
                let imm115 = (instruction >> 25) & 0b1111111;
                let rs2 = (((instruction >> 20) & 0b11111) as usize).into();
                let rs1 = (((instruction >> 15) & 0b11111) as usize).into();
                let funct3 = (instruction >> 12) & 0b11;
                let imm40 = (instruction >> 7) & 0b11111;

                // Merge and sign extend the immediate
                let imm = (imm115 << 5) | imm40;
                let imm = ((imm as i32) << 20) >> 20;

                match opcode {
                    0b0100011 => match funct3 {
                        0b000 => Instruction::Sb { rs1, rs2, imm },
                        0b001 => Instruction::Sh { rs1, rs2, imm },
                        0b010 => Instruction::Sw { rs1, rs2, imm },
                        0b011 => Instruction::Sd { rs1, rs2, imm },
                        _ => Instruction::Undefined,
                    },
                    _ => Instruction::Undefined,
                }
            }

            InstructionFormat::U => {
                // Decode fields
                let imm = (instruction & 0xfffff000) as i32;
                let rd = (((instruction >> 7) & 0b11111) as usize).into();

                match opcode {
                    0b0010111 => Instruction::Auipc { rd, imm },
                    0b0110111 => Instruction::Lui { rd, imm },
                    _ => Instruction::Undefined,
                }
            }
        }
    }
}
const ENCODING_TABLE: [Option<InstructionFormat>; 128] = [
    /* 0b0000000 */ None,
    /* 0b0000001 */ None,
    /* 0b0000010 */ None,
    /* 0b0000011 */ Some(InstructionFormat::I),
    /* 0b0000100 */ None,
    /* 0b0000101 */ None,
    /* 0b0000110 */ None,
    /* 0b0000111 */ None,
    /* 0b0001000 */ None,
    /* 0b0001001 */ None,
    /* 0b0001010 */ None,
    /* 0b0001011 */ None,
    /* 0b0001100 */ None,
    /* 0b0001101 */ None,
    /* 0b0001110 */ None,
    /* 0b0001111 */ Some(InstructionFormat::I),
    /* 0b0010000 */ None,
    /* 0b0010001 */ None,
    /* 0b0010010 */ None,
    /* 0b0010011 */ Some(InstructionFormat::I),
    /* 0b0010100 */ None,
    /* 0b0010101 */ None,
    /* 0b0010110 */ None,
    /* 0b0010111 */ Some(InstructionFormat::U),
    /* 0b0011000 */ None,
    /* 0b0011001 */ None,
    /* 0b0011010 */ None,
    /* 0b0011011 */ Some(InstructionFormat::I),
    /* 0b0011100 */ None,
    /* 0b0011101 */ None,
    /* 0b0011110 */ None,
    /* 0b0011111 */ None,
    /* 0b0100000 */ None,
    /* 0b0100001 */ None,
    /* 0b0100010 */ None,
    /* 0b0100011 */ Some(InstructionFormat::S),
    /* 0b0100100 */ None,
    /* 0b0100101 */ None,
    /* 0b0100110 */ None,
    /* 0b0100111 */ None,
    /* 0b0101000 */ None,
    /* 0b0101001 */ None,
    /* 0b0101010 */ None,
    /* 0b0101011 */ None,
    /* 0b0101100 */ None,
    /* 0b0101101 */ None,
    /* 0b0101110 */ None,
    /* 0b0101111 */ None,
    /* 0b0110000 */ None,
    /* 0b0110001 */ None,
    /* 0b0110010 */ None,
    /* 0b0110011 */ Some(InstructionFormat::R),
    /* 0b0110100 */ None,
    /* 0b0110101 */ None,
    /* 0b0110110 */ None,
    /* 0b0110111 */ Some(InstructionFormat::U),
    /* 0b0111000 */ None,
    /* 0b0111001 */ None,
    /* 0b0111010 */ None,
    /* 0b0111011 */ Some(InstructionFormat::R),
    /* 0b0111100 */ None,
    /* 0b0111101 */ None,
    /* 0b0111110 */ None,
    /* 0b0111111 */ None,
    /* 0b1000000 */ None,
    /* 0b1000001 */ None,
    /* 0b1000010 */ None,
    /* 0b1000011 */ None,
    /* 0b1000100 */ None,
    /* 0b1000101 */ None,
    /* 0b1000110 */ None,
    /* 0b1000111 */ None,
    /* 0b1001000 */ None,
    /* 0b1001001 */ None,
    /* 0b1001010 */ None,
    /* 0b1001011 */ None,
    /* 0b1001100 */ None,
    /* 0b1001101 */ None,
    /* 0b1001110 */ None,
    /* 0b1001111 */ None,
    /* 0b1010000 */ None,
    /* 0b1010001 */ None,
    /* 0b1010010 */ None,
    /* 0b1010011 */ None,
    /* 0b1010100 */ None,
    /* 0b1010101 */ None,
    /* 0b1010110 */ None,
    /* 0b1010111 */ None,
    /* 0b1011000 */ None,
    /* 0b1011001 */ None,
    /* 0b1011010 */ None,
    /* 0b1011011 */ None,
    /* 0b1011100 */ None,
    /* 0b1011101 */ None,
    /* 0b1011110 */ None,
    /* 0b1011111 */ Some(InstructionFormat::J),
    /* 0b1100000 */ None,
    /* 0b1100001 */ None,
    /* 0b1100010 */ None,
    /* 0b1100011 */ Some(InstructionFormat::B),
    /* 0b1100100 */ None,
    /* 0b1100101 */ None,
    /* 0b1100110 */ None,
    /* 0b1100111 */ Some(InstructionFormat::I),
    /* 0b1101000 */ None,
    /* 0b1101001 */ None,
    /* 0b1101010 */ None,
    /* 0b1101011 */ None,
    /* 0b1101100 */ None,
    /* 0b1101101 */ None,
    /* 0b1101110 */ None,
    /* 0b1101111 */ None,
    /* 0b1110000 */ None,
    /* 0b1110001 */ None,
    /* 0b1110010 */ None,
    /* 0b1110011 */ Some(InstructionFormat::I),
    /* 0b1110100 */ None,
    /* 0b1110101 */ None,
    /* 0b1110110 */ None,
    /* 0b1110111 */ None,
    /* 0b1111000 */ None,
    /* 0b1111001 */ None,
    /* 0b1111010 */ None,
    /* 0b1111011 */ None,
    /* 0b1111100 */ None,
    /* 0b1111101 */ None,
    /* 0b1111110 */ None,
    /* 0b1111111 */ None,
];
