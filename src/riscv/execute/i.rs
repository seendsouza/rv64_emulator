use crate::riscv::cpu::Cpu;
use crate::riscv::cpu::Register;

pub fn execute_lb(rd: Register, rs1: Register, imm: i32, cpu: &mut Cpu) {}
pub fn execute_lh(rd: Register, rs1: Register, imm: i32, cpu: &mut Cpu) {}
pub fn execute_lw(rd: Register, rs1: Register, imm: i32, cpu: &mut Cpu) {}
pub fn execute_lbu(rd: Register, rs1: Register, imm: i32, cpu: &mut Cpu) {}
pub fn execute_lhu(rd: Register, rs1: Register, imm: i32, cpu: &mut Cpu) {}
pub fn execute_lwu(rd: Register, rs1: Register, imm: i32, cpu: &mut Cpu) {}
pub fn execute_ld(rd: Register, rs1: Register, imm: i32, cpu: &mut Cpu) {}

pub fn execute_fence(rd: Register, rs1: Register, succ: u32, pred: u32, fm: u32, cpu: &mut Cpu) {}

pub fn execute_addi(rd: Register, rs1: Register, imm: i32, cpu: &mut Cpu) {}
pub fn execute_slti(rd: Register, rs1: Register, imm: i32, cpu: &mut Cpu) {}
pub fn execute_sltiu(rd: Register, rs1: Register, imm: i32, cpu: &mut Cpu) {}
pub fn execute_xori(rd: Register, rs1: Register, imm: i32, cpu: &mut Cpu) {}
pub fn execute_ori(rd: Register, rs1: Register, imm: i32, cpu: &mut Cpu) {}
pub fn execute_andi(rd: Register, rs1: Register, imm: i32, cpu: &mut Cpu) {}
pub fn execute_slli(rd: Register, rs1: Register, shamt: u32, cpu: &mut Cpu) {}
pub fn execute_srli(rd: Register, rs1: Register, shamt: u32, cpu: &mut Cpu) {}
pub fn execute_srai(rd: Register, rs1: Register, shamt: u32, cpu: &mut Cpu) {}
pub fn execute_addiw(rd: Register, rs1: Register, imm: i32, cpu: &mut Cpu) {}
pub fn execute_slliw(rd: Register, rs1: Register, shamt: u32, cpu: &mut Cpu) {}
pub fn execute_srliw(rd: Register, rs1: Register, shamt: u32, cpu: &mut Cpu) {}
pub fn execute_sraiw(rd: Register, rs1: Register, shamt: u32, cpu: &mut Cpu) {}

pub fn execute_jalr(rd: Register, rs1: Register, imm: i32, cpu: &mut Cpu) {}

pub fn execute_ebreak(cpu: &mut Cpu) {}
pub fn execute_ecall(cpu: &mut Cpu) {}
