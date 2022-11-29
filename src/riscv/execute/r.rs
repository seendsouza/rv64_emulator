use crate::riscv::cpu::Cpu;
use crate::riscv::cpu::Register;

pub fn execute_add(rd: Register, rs1: Register, rs2: Register, cpu: &mut Cpu) {}
pub fn execute_sub(rd: Register, rs1: Register, rs2: Register, cpu: &mut Cpu) {}
pub fn execute_sll(rd: Register, rs1: Register, rs2: Register, cpu: &mut Cpu) {}
pub fn execute_slt(rd: Register, rs1: Register, rs2: Register, cpu: &mut Cpu) {}
pub fn execute_sltu(rd: Register, rs1: Register, rs2: Register, cpu: &mut Cpu) {}
pub fn execute_xor(rd: Register, rs1: Register, rs2: Register, cpu: &mut Cpu) {}
pub fn execute_srl(rd: Register, rs1: Register, rs2: Register, cpu: &mut Cpu) {}
pub fn execute_sra(rd: Register, rs1: Register, rs2: Register, cpu: &mut Cpu) {}
pub fn execute_or(rd: Register, rs1: Register, rs2: Register, cpu: &mut Cpu) {}
pub fn execute_and(rd: Register, rs1: Register, rs2: Register, cpu: &mut Cpu) {}
pub fn execute_addw(rd: Register, rs1: Register, rs2: Register, cpu: &mut Cpu) {}
pub fn execute_subw(rd: Register, rs1: Register, rs2: Register, cpu: &mut Cpu) {}
pub fn execute_sllw(rd: Register, rs1: Register, rs2: Register, cpu: &mut Cpu) {}
pub fn execute_srlw(rd: Register, rs1: Register, rs2: Register, cpu: &mut Cpu) {}
pub fn execute_sraw(rd: Register, rs1: Register, rs2: Register, cpu: &mut Cpu) {}
