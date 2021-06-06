use crate::riscv::cpu;
// Decode a RISC-V 64 instruction and return the instruction
pub fn decode(instruction: u32) -> Instruction {
    let opcode = instruction & 0b1111111;
    if let Some(instruction_format) = &ENCODING_TABLE[opcode as usize] {
        instruction_format.decode(instruction)
    } else {
        Instruction::Undefined
    }
}

#[derive(Debug, PartialEq)]
pub enum Instruction {
    Undefined,

    // B-Type
    Beq {
        rs1: cpu::Register,
        rs2: cpu::Register,
        imm: i32,
    },
    Bne {
        rs1: cpu::Register,
        rs2: cpu::Register,
        imm: i32,
    },
    Blt {
        rs1: cpu::Register,
        rs2: cpu::Register,
        imm: i32,
    },
    Bge {
        rs1: cpu::Register,
        rs2: cpu::Register,
        imm: i32,
    },
    Bltu {
        rs1: cpu::Register,
        rs2: cpu::Register,
        imm: i32,
    },
    Bgeu {
        rs1: cpu::Register,
        rs2: cpu::Register,
        imm: i32,
    },

    // I-Type
    Lb {
        rd: cpu::Register,
        rs1: cpu::Register,
        imm: i32,
    },
    Lh {
        rd: cpu::Register,
        rs1: cpu::Register,
        imm: i32,
    },
    Lw {
        rd: cpu::Register,
        rs1: cpu::Register,
        imm: i32,
    },
    Lbu {
        rd: cpu::Register,
        rs1: cpu::Register,
        imm: i32,
    },
    Lhu {
        rd: cpu::Register,
        rs1: cpu::Register,
        imm: i32,
    },
    Lwu {
        rd: cpu::Register,
        rs1: cpu::Register,
        imm: i32,
    },
    Ld {
        rd: cpu::Register,
        rs1: cpu::Register,
        imm: i32,
    },

    Fence {
        rd: cpu::Register,
        rs1: cpu::Register,
        succ: u32,
        pred: u32,
        fm: u32,
    },

    Addi {
        rd: cpu::Register,
        rs1: cpu::Register,
        imm: i32,
    },
    Slti {
        rd: cpu::Register,
        rs1: cpu::Register,
        imm: i32,
    },
    Sltiu {
        rd: cpu::Register,
        rs1: cpu::Register,
        imm: i32,
    },
    Xori {
        rd: cpu::Register,
        rs1: cpu::Register,
        imm: i32,
    },
    Ori {
        rd: cpu::Register,
        rs1: cpu::Register,
        imm: i32,
    },
    Andi {
        rd: cpu::Register,
        rs1: cpu::Register,
        imm: i32,
    },
    Slli {
        rd: cpu::Register,
        rs1: cpu::Register,
        shamt: u32,
    },
    Srli {
        rd: cpu::Register,
        rs1: cpu::Register,
        shamt: u32,
    },
    Srai {
        rd: cpu::Register,
        rs1: cpu::Register,
        shamt: u32,
    },
    Addiw {
        rd: cpu::Register,
        rs1: cpu::Register,
        imm: i32,
    },
    Slliw {
        rd: cpu::Register,
        rs1: cpu::Register,
        shamt: u32,
    },
    Srliw {
        rd: cpu::Register,
        rs1: cpu::Register,
        shamt: u32,
    },
    Sraiw {
        rd: cpu::Register,
        rs1: cpu::Register,
        shamt: u32,
    },

    Jalr {
        rd: cpu::Register,
        rs1: cpu::Register,
        imm: i32,
    },

    Ebreak,
    Ecall,

    // J-Type
    Jal {
        rd: cpu::Register,
        imm: i32,
    },

    // R-Type
    Add {
        rd: cpu::Register,
        rs1: cpu::Register,
        rs2: cpu::Register,
    },
    Sub {
        rd: cpu::Register,
        rs1: cpu::Register,
        rs2: cpu::Register,
    },
    Sll {
        rd: cpu::Register,
        rs1: cpu::Register,
        rs2: cpu::Register,
    },
    Slt {
        rd: cpu::Register,
        rs1: cpu::Register,
        rs2: cpu::Register,
    },
    Sltu {
        rd: cpu::Register,
        rs1: cpu::Register,
        rs2: cpu::Register,
    },
    Xor {
        rd: cpu::Register,
        rs1: cpu::Register,
        rs2: cpu::Register,
    },
    Srl {
        rd: cpu::Register,
        rs1: cpu::Register,
        rs2: cpu::Register,
    },
    Sra {
        rd: cpu::Register,
        rs1: cpu::Register,
        rs2: cpu::Register,
    },
    Or {
        rd: cpu::Register,
        rs1: cpu::Register,
        rs2: cpu::Register,
    },
    And {
        rd: cpu::Register,
        rs1: cpu::Register,
        rs2: cpu::Register,
    },
    Addw {
        rd: cpu::Register,
        rs1: cpu::Register,
        rs2: cpu::Register,
    },
    Subw {
        rd: cpu::Register,
        rs1: cpu::Register,
        rs2: cpu::Register,
    },
    Sllw {
        rd: cpu::Register,
        rs1: cpu::Register,
        rs2: cpu::Register,
    },
    Srlw {
        rd: cpu::Register,
        rs1: cpu::Register,
        rs2: cpu::Register,
    },
    Sraw {
        rd: cpu::Register,
        rs1: cpu::Register,
        rs2: cpu::Register,
    },

    // S-Type
    Sb {
        rs1: cpu::Register,
        rs2: cpu::Register,
        imm: i32,
    },
    Sh {
        rs1: cpu::Register,
        rs2: cpu::Register,
        imm: i32,
    },
    Sw {
        rs1: cpu::Register,
        rs2: cpu::Register,
        imm: i32,
    },
    Sd {
        rs1: cpu::Register,
        rs2: cpu::Register,
        imm: i32,
    },

    // U-Type
    Auipc {
        rd: cpu::Register,
        imm: i32,
    },
    Lui {
        rd: cpu::Register,
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
        let opcode = instruction & 0b1111111;
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
                        0b000
                            if imm == 0 && rs1 == cpu::Register::X0 && rd == cpu::Register::X0 =>
                        {
                            Instruction::Ecall
                        }
                        0b000
                            if imm == 1 && rs1 == cpu::Register::X0 && rd == cpu::Register::X0 =>
                        {
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
    /* 0b1011111 */ None,
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
    /* 0b1101111 */ Some(InstructionFormat::J),
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

#[cfg(test)]
mod tests {
    #[test]
    fn decode_lui() {
        assert_eq!(
            super::decode(0x00010537),
            super::Instruction::Lui {
                rd: (crate::riscv::cpu::AbiRegister::A0).into(),
                imm: 0x10
            }
        );
    }
    #[test]
    fn decode_auipc() {
        assert_eq!(
            super::decode(0x00000297),
            super::Instruction::Auipc {
                rd: (crate::riscv::cpu::AbiRegister::T0).into(),
                imm: 0x0
            }
        );
    }
    #[test]
    fn decode_jal() {}
    #[test]
    fn decode_jalr() {
        assert_eq!(
            super::decode(0xf98680e7),
            super::Instruction::Jalr {
                rd: (crate::riscv::cpu::AbiRegister::Zero).into(),
                rs1: (crate::riscv::cpu::AbiRegister::A3).into(),
                imm: -104
            }
        );
    }
    #[test]
    fn decode_beq() {
        assert_eq!(
            super::decode(0x05778063),
            super::Instruction::Beq {
                rs1: (crate::riscv::cpu::AbiRegister::A5).into(),
                rs2: (crate::riscv::cpu::AbiRegister::S7).into(),
                imm: 10424
            }
        );
    }
    #[test]
    fn decode_bne() {
        assert_eq!(
            super::decode(0xff3416e3),
            super::Instruction::Beq {
                rs1: (crate::riscv::cpu::AbiRegister::S0Fp).into(),
                rs2: (crate::riscv::cpu::AbiRegister::S3).into(),
                imm: 0x103dc
            }
        );
    }
    #[test]
    fn decode_blt() {
        assert_eq!(
            super::decode(0x08e84663),
            super::Instruction::Blt {
                rs1: (crate::riscv::cpu::AbiRegister::A6).into(),
                rs2: (crate::riscv::cpu::AbiRegister::A4).into(),
                imm: 0x105b8
            }
        );
    }
    #[test]
    fn decode_bge() {
        assert_eq!(
            super::decode(0xfce7dce3),
            super::Instruction::Bge {
                rs1: (crate::riscv::cpu::AbiRegister::A5).into(),
                rs2: (crate::riscv::cpu::AbiRegister::A4).into(),
                imm: 10198
            }
        );
    }
    #[test]
    fn decode_bltu() {
        assert_eq!(
            super::decode(0xfed76ae3),
            super::Instruction::Bge {
                rs1: (crate::riscv::cpu::AbiRegister::A4).into(),
                rs2: (crate::riscv::cpu::AbiRegister::A3).into(),
                imm: 0x102c8
            }
        );
    }
    #[test]
    fn decode_bgeu() {
        assert_eq!(
            super::decode(0xf6c374e3),
            super::Instruction::Bgeu {
                rs1: (crate::riscv::cpu::AbiRegister::T1).into(),
                rs2: (crate::riscv::cpu::AbiRegister::A2).into(),
                imm: 0x102e0
            }
        );
    }
    #[test]
    fn decode_lb() {
        assert_eq!(
            super::decode(0x00078b03),
            super::Instruction::Lb {
                rd: (crate::riscv::cpu::AbiRegister::S6).into(),
                rs1: (crate::riscv::cpu::AbiRegister::A5).into(),
                imm: 0
            }
        );
    }
    #[test]
    fn decode_lh() {
        assert_eq!(
            super::decode(0x01099703),
            super::Instruction::Lh {
                rd: (crate::riscv::cpu::AbiRegister::A4).into(),
                rs1: (crate::riscv::cpu::AbiRegister::S3).into(),
                imm: 16
            }
        );
    }
    #[test]
    fn decode_lw() {
        assert_eq!(
            super::decode(0xfec42703),
            super::Instruction::Lw {
                rd: (crate::riscv::cpu::AbiRegister::A4).into(),
                rs1: (crate::riscv::cpu::AbiRegister::S0Fp).into(),
                imm: -20
            }
        );
    }
    #[test]
    fn decode_lbu() {
        assert_eq!(
            super::decode(0xf601c703),
            super::Instruction::Lbu {
                rd: (crate::riscv::cpu::AbiRegister::A4).into(),
                rs1: (crate::riscv::cpu::AbiRegister::Gp).into(),
                imm: -160
            }
        );
    }
    #[test]
    fn decode_lhu() {
        assert_eq!(
            super::decode(0x010c5783),
            super::Instruction::Lhu {
                rd: (crate::riscv::cpu::AbiRegister::A5).into(),
                rs1: (crate::riscv::cpu::AbiRegister::S8).into(),
                imm: 16
            }
        );
    }
    #[test]
    fn decode_sb() {
        assert_eq!(
            super::decode(0xf6f18023),
            super::Instruction::Sb {
                rs2: (crate::riscv::cpu::AbiRegister::A5).into(),
                rs1: (crate::riscv::cpu::AbiRegister::Gp).into(),
                imm: -160
            }
        );
    }
    #[test]
    fn decode_sh() {
        assert_eq!(
            super::decode(0x0ef11023),
            super::Instruction::Sh {
                rs2: (crate::riscv::cpu::AbiRegister::A5).into(),
                rs1: (crate::riscv::cpu::AbiRegister::Sp).into(),
                imm: 224
            }
        );
    }
    #[test]
    fn decode_sw() {
        assert_eq!(
            super::decode(0xfe042623),
            super::Instruction::Sw {
                rs2: (crate::riscv::cpu::AbiRegister::Zero).into(),
                rs1: (crate::riscv::cpu::AbiRegister::S0Fp).into(),
                imm: -20
            }
        );
    }
    #[test]
    fn decode_addi() {
        assert_eq!(
            super::decode(0x4a850513),
            super::Instruction::Addi {
                rd: (crate::riscv::cpu::AbiRegister::A0).into(),
                rs1: (crate::riscv::cpu::AbiRegister::A0).into(),
                imm: 1192
            }
        );
    }
    #[test]
    fn decode_slti() {
        assert_eq!(
            super::decode(0x000b2f13),
            super::Instruction::Slti {
                rd: (crate::riscv::cpu::AbiRegister::T5).into(),
                rs1: (crate::riscv::cpu::AbiRegister::S6).into(),
                imm: 0
            }
        );
    }
    #[test]
    fn decode_sltiu() {
        assert_eq!(
            super::decode(0x3b0b3a13),
            super::Instruction::Sltiu {
                rd: (crate::riscv::cpu::AbiRegister::S4).into(),
                rs1: (crate::riscv::cpu::AbiRegister::S6).into(),
                imm: 944
            }
        );
    }
    #[test]
    fn decode_xori() {
        assert_eq!(
            super::decode(0x8307c793),
            super::Instruction::Xori {
                rd: (crate::riscv::cpu::AbiRegister::A5).into(),
                rs1: (crate::riscv::cpu::AbiRegister::A5).into(),
                imm: -2000
            }
        );
    }
    #[test]
    fn decode_ori() {
        assert_eq!(
            super::decode(0x01096913),
            super::Instruction::Ori {
                rd: (crate::riscv::cpu::AbiRegister::S2).into(),
                rs1: (crate::riscv::cpu::AbiRegister::S2).into(),
                imm: 16
            }
        );
    }
    #[test]
    fn decode_andi() {
        assert_eq!(
            super::decode(0x00f77793),
            super::Instruction::Andi {
                rd: (crate::riscv::cpu::AbiRegister::A5).into(),
                rs1: (crate::riscv::cpu::AbiRegister::A4).into(),
                imm: 14
            }
        );
    }
    #[test]
    fn decode_add() {
        assert_eq!(
            super::decode(0x00878433),
            super::Instruction::Add {
                rd: (crate::riscv::cpu::AbiRegister::S0Fp).into(),
                rs1: (crate::riscv::cpu::AbiRegister::A5).into(),
                rs2: (crate::riscv::cpu::AbiRegister::S0Fp).into(),
            }
        );
    }
    #[test]
    fn decode_sub() {
        assert_eq!(
            super::decode(0x40a60633),
            super::Instruction::Sub {
                rd: (crate::riscv::cpu::AbiRegister::A2).into(),
                rs1: (crate::riscv::cpu::AbiRegister::A2).into(),
                rs2: (crate::riscv::cpu::AbiRegister::A0).into(),
            }
        );
    }
    #[test]
    fn decode_sll() {
        assert_eq!(
            super::decode(0x00c797b3),
            super::Instruction::Sll {
                rd: (crate::riscv::cpu::AbiRegister::A5).into(),
                rs1: (crate::riscv::cpu::AbiRegister::A5).into(),
                rs2: (crate::riscv::cpu::AbiRegister::A2).into(),
            }
        );
    }
    #[test]
    fn decode_slt() {
        assert_eq!(
            super::decode(0x00632e33),
            super::Instruction::Slt {
                rd: (crate::riscv::cpu::AbiRegister::T3).into(),
                rs1: (crate::riscv::cpu::AbiRegister::T1).into(),
                rs2: (crate::riscv::cpu::AbiRegister::T1).into(),
            }
        );
    }
    #[test]
    fn decode_sltu() {
        assert_eq!(
            super::decode(0x0123bfb3),
            super::Instruction::Sltu {
                rd: (crate::riscv::cpu::AbiRegister::T6).into(),
                rs1: (crate::riscv::cpu::AbiRegister::T2).into(),
                rs2: (crate::riscv::cpu::AbiRegister::S2).into(),
            }
        );
    }
    #[test]
    fn decode_xor() {
        assert_eq!(
            super::decode(0x00facab3),
            super::Instruction::Xor {
                rd: (crate::riscv::cpu::AbiRegister::S5).into(),
                rs1: (crate::riscv::cpu::AbiRegister::S5).into(),
                rs2: (crate::riscv::cpu::AbiRegister::A5).into(),
            }
        );
    }
    #[test]
    fn decode_srl() {
        assert_eq!(
            super::decode(0x00a457b3),
            super::Instruction::Srl {
                rd: (crate::riscv::cpu::AbiRegister::A5).into(),
                rs1: (crate::riscv::cpu::AbiRegister::S0Fp).into(),
                rs2: (crate::riscv::cpu::AbiRegister::A0).into(),
            }
        );
    }
    #[test]
    fn decode_sra() {
        assert_eq!(
            super::decode(0b01000000101001000101011110110011),
            super::Instruction::Sra {
                rd: (crate::riscv::cpu::AbiRegister::A5).into(),
                rs1: (crate::riscv::cpu::AbiRegister::S0Fp).into(),
                rs2: (crate::riscv::cpu::AbiRegister::A0).into(),
            }
        );
    }
    #[test]
    fn decode_or() {
        assert_eq!(
            super::decode(0x00c8e8b3),
            super::Instruction::Or {
                rd: (crate::riscv::cpu::AbiRegister::A7).into(),
                rs1: (crate::riscv::cpu::AbiRegister::A7).into(),
                rs2: (crate::riscv::cpu::AbiRegister::A2).into(),
            }
        );
    }
    #[test]
    fn decode_and() {
        assert_eq!(
            super::decode(0x00d7f7b3),
            super::Instruction::And {
                rd: (crate::riscv::cpu::AbiRegister::A5).into(),
                rs1: (crate::riscv::cpu::AbiRegister::A5).into(),
                rs2: (crate::riscv::cpu::AbiRegister::A3).into(),
            }
        );
    }
    #[test]
    fn decode_fence() {
        panic!("// TODO: implement this");
    }
    #[test]
    fn decode_ecall() {
        assert_eq!(super::decode(0x00000073), super::Instruction::Ecall);
    }
    #[test]
    fn decode_ebreak() {
        assert_eq!(super::decode(0x00100073), super::Instruction::Ebreak);
    }
    #[test]
    fn decode_lwu() {
        assert_eq!(
            super::decode(0x0000e903),
            super::Instruction::Lwu {
                rd: (crate::riscv::cpu::AbiRegister::S2).into(),
                rs1: (crate::riscv::cpu::AbiRegister::Ra).into(),
                imm: 0
            }
        );
    }
    #[test]
    fn decode_ld() {
        assert_eq!(
            super::decode(0xf581b503),
            super::Instruction::Ld {
                rd: (crate::riscv::cpu::AbiRegister::A0).into(),
                rs1: (crate::riscv::cpu::AbiRegister::Gp).into(),
                imm: -168,
            }
        );
    }
    #[test]
    fn decode_sd() {
        assert_eq!(
            super::decode(0x00813023),
            super::Instruction::Sd {
                rs2: (crate::riscv::cpu::AbiRegister::S0Fp).into(),
                rs1: (crate::riscv::cpu::AbiRegister::Sp).into(),
                imm: 0,
            }
        );
    }
    #[test]
    fn decode_slli() {
        assert_eq!(
            super::decode(0x00269693),
            super::Instruction::Slli {
                rd: (crate::riscv::cpu::AbiRegister::A3).into(),
                rs1: (crate::riscv::cpu::AbiRegister::A3).into(),
                shamt: 0x2
            }
        );
    }
    #[test]
    fn decode_srli() {
        assert_eq!(
            super::decode(0x01e75793),
            super::Instruction::Srli {
                rd: (crate::riscv::cpu::AbiRegister::A5).into(),
                rs1: (crate::riscv::cpu::AbiRegister::A4).into(),
                shamt: 0x1e
            }
        );
    }
    #[test]
    fn decode_srai() {
        assert_eq!(
            super::decode(0x4037d493),
            super::Instruction::Srai {
                rd: (crate::riscv::cpu::AbiRegister::S1).into(),
                rs1: (crate::riscv::cpu::AbiRegister::A5).into(),
                shamt: 0x3
            }
        );
    }
    #[test]
    fn decode_addiw() {
        assert_eq!(
            super::decode(0xfff4841b),
            super::Instruction::Addiw {
                rd: (crate::riscv::cpu::AbiRegister::S0Fp).into(),
                rs1: (crate::riscv::cpu::AbiRegister::S1).into(),
                imm: -1
            }
        );
    }
    #[test]
    fn decode_slliw() {
        assert_eq!(
            super::decode(0x0054141b),
            super::Instruction::Slliw {
                rd: (crate::riscv::cpu::AbiRegister::S0Fp).into(),
                rs1: (crate::riscv::cpu::AbiRegister::S0Fp).into(),
                shamt: 0x5
            }
        );
    }
    #[test]
    fn decode_srliw() {
        assert_eq!(
            super::decode(0x01f6d49b),
            super::Instruction::Srliw {
                rd: (crate::riscv::cpu::AbiRegister::S1).into(),
                rs1: (crate::riscv::cpu::AbiRegister::A3).into(),
                shamt: 0x1f
            }
        );
    }
    #[test]
    fn decode_sraiw() {
        assert_eq!(
            super::decode(0x4014d49b),
            super::Instruction::Sraiw {
                rd: (crate::riscv::cpu::AbiRegister::S1).into(),
                rs1: (crate::riscv::cpu::AbiRegister::S1).into(),
                shamt: 0x1
            }
        );
    }
    #[test]
    fn decode_addw() {
        assert_eq!(
            super::decode(0x00f707bb),
            super::Instruction::Addw {
                rd: (crate::riscv::cpu::AbiRegister::A5).into(),
                rs2: (crate::riscv::cpu::AbiRegister::A4).into(),
                rs1: (crate::riscv::cpu::AbiRegister::A5).into(),
            }
        );
    }
    #[test]
    fn decode_subw() {
        assert_eq!(
            super::decode(0x41b404bb),
            super::Instruction::Subw {
                rd: (crate::riscv::cpu::AbiRegister::S1).into(),
                rs2: (crate::riscv::cpu::AbiRegister::S10).into(),
                rs1: (crate::riscv::cpu::AbiRegister::S11).into(),
            }
        );
    }
    #[test]
    fn decode_sllw() {
        assert_eq!(
            super::decode(0x008a96bb),
            super::Instruction::Sllw {
                rd: (crate::riscv::cpu::AbiRegister::A3).into(),
                rs2: (crate::riscv::cpu::AbiRegister::S5).into(),
                rs1: (crate::riscv::cpu::AbiRegister::S0Fp).into(),
            }
        );
    }
    #[test]
    fn decode_srlw() {
        assert_eq!(
            super::decode(0x00b7573b),
            super::Instruction::Srlw {
                rd: (crate::riscv::cpu::AbiRegister::A4).into(),
                rs2: (crate::riscv::cpu::AbiRegister::A4).into(),
                rs1: (crate::riscv::cpu::AbiRegister::A1).into(),
            }
        );
    }
    #[test]
    fn decode_sraw() {
        assert_eq!(
            super::decode(0x40f5553b),
            super::Instruction::Sraw {
                rd: (crate::riscv::cpu::AbiRegister::A0).into(),
                rs2: (crate::riscv::cpu::AbiRegister::A0).into(),
                rs1: (crate::riscv::cpu::AbiRegister::A5).into(),
            }
        );
    }
}
