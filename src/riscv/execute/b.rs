use crate::riscv::cpu::Cpu;
use crate::riscv::cpu::Register;

pub fn execute_beq(rs1: Register, rs2: Register, imm: i32, cpu: &mut Cpu) {}
pub fn execute_bne(rs1: Register, rs2: Register, imm: i32, cpu: &mut Cpu) {}
pub fn execute_blt(rs1: Register, rs2: Register, imm: i32, cpu: &mut Cpu) {}
pub fn execute_bge(rs1: Register, rs2: Register, imm: i32, cpu: &mut Cpu) {}
pub fn execute_bltu(rs1: Register, rs2: Register, imm: i32, cpu: &mut Cpu) {}
pub fn execute_bgeu(rs1: Register, rs2: Register, imm: i32, cpu: &mut Cpu) {}
