pub mod cpu;
pub mod instruction;

pub fn emulate(encoded_instructions: Vec<u8>) {
    let mut cpu = cpu::Cpu::new(encoded_instructions);
    while cpu.pc < cpu.encoded_instructions.len() as u64 {
        let encoded_instruction = cpu.fetch();
        let instruction = instruction::decode(encoded_instruction);
        cpu.pc += 4;
        //if 87 * 4 <= cpu.pc && cpu.pc <= 92 * 4 {
        println!("{:#032b} | {:?}", encoded_instruction, instruction);
        cpu.execute(instruction);
        //}
    }
}
