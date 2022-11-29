pub mod b;
pub mod i;
pub mod j;
pub mod r;
pub mod s;
pub mod u;
use crate::riscv::cpu::Cpu;
use crate::riscv::instruction::Instruction;

pub fn execute_instruction(instruction: Instruction, cpu: &mut Cpu) {
    return match instruction {
        // B-Type
        Instruction::Beq { rs1, rs2, imm } => b::execute_beq(rs1, rs2, imm, cpu),
        Instruction::Bne { rs1, rs2, imm } => b::execute_bne(rs1, rs2, imm, cpu),
        Instruction::Blt { rs1, rs2, imm } => b::execute_blt(rs1, rs2, imm, cpu),
        Instruction::Bge { rs1, rs2, imm } => b::execute_bge(rs1, rs2, imm, cpu),
        Instruction::Bltu { rs1, rs2, imm } => b::execute_bltu(rs1, rs2, imm, cpu),
        Instruction::Bgeu { rs1, rs2, imm } => b::execute_bgeu(rs1, rs2, imm, cpu),
        // I-Type
        Instruction::Lb { rd, rs1, imm } => i::execute_lb(rd, rs1, imm, cpu),
        Instruction::Lh { rd, rs1, imm } => i::execute_lh(rd, rs1, imm, cpu),
        Instruction::Lw { rd, rs1, imm } => i::execute_lw(rd, rs1, imm, cpu),
        Instruction::Lbu { rd, rs1, imm } => i::execute_lbu(rd, rs1, imm, cpu),
        Instruction::Lhu { rd, rs1, imm } => i::execute_lhu(rd, rs1, imm, cpu),
        Instruction::Lwu { rd, rs1, imm } => i::execute_lwu(rd, rs1, imm, cpu),
        Instruction::Ld { rd, rs1, imm } => i::execute_ld(rd, rs1, imm, cpu),
        Instruction::Fence {
            rd,
            rs1,
            succ,
            pred,
            fm,
        } => i::execute_fence(rd, rs1, succ, pred, fm, cpu),
        Instruction::Addi { rd, rs1, imm } => i::execute_addi(rd, rs1, imm, cpu),
        Instruction::Slti { rd, rs1, imm } => i::execute_slti(rd, rs1, imm, cpu),
        Instruction::Sltiu { rd, rs1, imm } => i::execute_sltiu(rd, rs1, imm, cpu),
        Instruction::Xori { rd, rs1, imm } => i::execute_xori(rd, rs1, imm, cpu),
        Instruction::Ori { rd, rs1, imm } => i::execute_ori(rd, rs1, imm, cpu),
        Instruction::Andi { rd, rs1, imm } => i::execute_andi(rd, rs1, imm, cpu),
        Instruction::Slli { rd, rs1, shamt } => i::execute_slli(rd, rs1, shamt, cpu),
        Instruction::Srli { rd, rs1, shamt } => i::execute_srli(rd, rs1, shamt, cpu),
        Instruction::Srai { rd, rs1, shamt } => i::execute_srai(rd, rs1, shamt, cpu),
        Instruction::Addiw { rd, rs1, imm } => i::execute_addiw(rd, rs1, imm, cpu),
        Instruction::Slliw { rd, rs1, shamt } => i::execute_slliw(rd, rs1, shamt, cpu),
        Instruction::Srliw { rd, rs1, shamt } => i::execute_srliw(rd, rs1, shamt, cpu),
        Instruction::Sraiw { rd, rs1, shamt } => i::execute_sraiw(rd, rs1, shamt, cpu),
        Instruction::Jalr { rd, rs1, imm } => i::execute_jalr(rd, rs1, imm, cpu),
        Instruction::Ebreak => i::execute_ebreak(cpu),
        Instruction::Ecall => i::execute_ecall(cpu),
        // J-Type
        Instruction::Jal { rd, imm } => j::execute_jal(rd, imm, cpu),
        // R-Type
        Instruction::Add { rd, rs1, rs2 } => r::execute_add(rd, rs1, rs2, cpu),
        Instruction::Sub { rd, rs1, rs2 } => r::execute_sub(rd, rs1, rs2, cpu),
        Instruction::Sll { rd, rs1, rs2 } => r::execute_sll(rd, rs1, rs2, cpu),
        Instruction::Slt { rd, rs1, rs2 } => r::execute_slt(rd, rs1, rs2, cpu),
        Instruction::Sltu { rd, rs1, rs2 } => r::execute_sltu(rd, rs1, rs2, cpu),
        Instruction::Xor { rd, rs1, rs2 } => r::execute_xor(rd, rs1, rs2, cpu),
        Instruction::Srl { rd, rs1, rs2 } => r::execute_srl(rd, rs1, rs2, cpu),
        Instruction::Sra { rd, rs1, rs2 } => r::execute_sra(rd, rs1, rs2, cpu),
        Instruction::Or { rd, rs1, rs2 } => r::execute_or(rd, rs1, rs2, cpu),
        Instruction::And { rd, rs1, rs2 } => r::execute_and(rd, rs1, rs2, cpu),
        Instruction::Addw { rd, rs1, rs2 } => r::execute_addw(rd, rs1, rs2, cpu),
        Instruction::Subw { rd, rs1, rs2 } => r::execute_subw(rd, rs1, rs2, cpu),
        Instruction::Sllw { rd, rs1, rs2 } => r::execute_sllw(rd, rs1, rs2, cpu),
        Instruction::Srlw { rd, rs1, rs2 } => r::execute_srlw(rd, rs1, rs2, cpu),
        Instruction::Sraw { rd, rs1, rs2 } => r::execute_sraw(rd, rs1, rs2, cpu),
        // S-Type
        Instruction::Sb { rs2, rs1, imm } => s::execute_sb(rs2, rs1, imm, cpu),
        Instruction::Sh { rs2, rs1, imm } => s::execute_sh(rs2, rs1, imm, cpu),
        Instruction::Sw { rs2, rs1, imm } => s::execute_sw(rs2, rs1, imm, cpu),
        Instruction::Sd { rs2, rs1, imm } => s::execute_sd(rs2, rs1, imm, cpu),
        // U-Type
        Instruction::Auipc { rd, imm } => u::execute_auipc(rd, imm, cpu),
        Instruction::Lui { rd, imm } => u::execute_lui(rd, imm, cpu),
        Instruction::Undefined => (),
    };
}
