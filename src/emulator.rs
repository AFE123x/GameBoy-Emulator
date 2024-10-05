mod cpu;
use cpu::Memory;
use cpu::CPU;
use cpu::Registers;
use cpu::ClockData;
pub fn printaoeu(){
    let mut memory = Memory::new();
    let mut cpu = CPU {
        registers: Registers {
            a: 0,
            b: 0,
            c: 0,
            d: 0,
            e: 0,
            h: 0,
            l: 0,
            sp: 0,
            pc: 0,
        },
        clock: ClockData {
            total_cycles: 0,
            cycles: 0,
        },
        memory: &mut memory, // Pass the memory reference
    };
    cpu.clock();
}