pub mod cpu;
pub mod execute;
pub mod instruction;

pub fn emulate(encoded_instructions: Vec<u8>) {
    let mut cpu = cpu::Cpu::new(encoded_instructions);
    while cpu.pc < cpu.encoded_instructions.len() as u64 {
        let encoded_instruction = cpu.fetch();
        let instruction = instruction::decode(encoded_instruction);
        cpu.pc += 4;
        cpu.execute(instruction);
    }
}
