use crate::riscv::instruction;

pub struct Cpu {
    // XLEN is 64 bits in rv64i
    pub registers: [u64; 32],
    pub pc: u64,
    pub encoded_instructions: Vec<u8>,
}
impl Cpu {
    pub fn new(encoded_instructions: Vec<u8>) -> Self {
        Self {
            registers: [0; 32],
            pc: 0,
            encoded_instructions,
        }
    }
    pub fn fetch(&self) -> instruction::Instruction {
        let index = self.pc as usize;
        let encoded_instruction = (self.encoded_instructions[index] as u32)
            | ((self.encoded_instructions[index + 1] as u32) << 8)
            | ((self.encoded_instructions[index + 2] as u32) << 16)
            | ((self.encoded_instructions[index + 3] as u32) << 24);
        instruction::decode(encoded_instruction)
    }
    pub fn execute(&self, instruction: instruction::Instruction) {
        println!("{:?}", instruction);
    }
}

#[derive(Debug, PartialEq)]
pub enum Register {
    PC,
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
