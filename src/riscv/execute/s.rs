use crate::riscv::cpu::Cpu;
use crate::riscv::cpu::Register;

pub fn execute_sb(rs2: Register, rs1: Register, imm: i32, cpu: &mut Cpu) {}
pub fn execute_sh(rs2: Register, rs1: Register, imm: i32, cpu: &mut Cpu) {}
pub fn execute_sw(rs2: Register, rs1: Register, imm: i32, cpu: &mut Cpu) {}
pub fn execute_sd(rs2: Register, rs1: Register, imm: i32, cpu: &mut Cpu) {}
