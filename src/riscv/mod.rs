pub mod cpu;
pub mod instruction;

pub fn emulate(encoded_instructions: Vec<u8>) {
    let mut cpu = cpu::Cpu::new(encoded_instructions);
    while cpu.pc < cpu.encoded_instructions.len() as u64 {
        let instruction = cpu.fetch();
        cpu.pc += 4;
        cpu.execute(instruction);
    }
}
