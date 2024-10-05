pub struct Memory {
    data: [u8; 0xFFFF], // 64KB of memory
}

impl Memory {
    pub fn new() -> Self {
        Memory {
            data: [0; 0xFFFF], // Initialize memory to zero
        }
    }

    pub fn read(&self, address: u16) -> u8 {
        self.data[address as usize]
    }

    pub fn write(&mut self, address: u16, value: u8) {
        self.data[address as usize] = value;
    }
}

pub struct Registers {
    pub a: u8,
    pub b: u8,
    pub c: u8,
    pub d: u8,
    pub e: u8,
    pub h: u8,
    pub l: u8,
    pub sp: u16,
    pub pc: u16,
}

pub struct ClockData {
    pub total_cycles: usize,
    pub cycles: u8,
}

#[derive(Clone, Copy)]
pub enum Register {
    A,
    B,
    C,
    D,
    E,
    H,
    L,
}

#[derive(Clone, Copy)]
pub enum RegisterPair {
    AF,
    BC,
    DE,
    HL,
    SP,
    PC,
}

pub enum Instruction {
    AdcAR8(Register),
    AdcAHL,
    AdcAImm(u8),
    AddAR8(Register),
    AddAHL,
    AddAImm(u8),
    AddHLR16(RegisterPair),
    AddHLSP,
    AddSPE8(i8),
}

pub struct CPU<'a> {
    pub registers: Registers,
    pub clock: ClockData,
    pub memory: &'a mut Memory,
}

impl<'a> CPU<'a> {
    
    /*CPU Instructions */
    
    
    pub fn get_register_value(&self, reg: Register) -> u8 {
        match reg {
            Register::A => self.registers.a,
            Register::B => self.registers.b,
            Register::C => self.registers.c,
            Register::D => self.registers.d,
            Register::E => self.registers.e,
            Register::H => self.registers.h,
            Register::L => self.registers.l,
        }
    }
    pub fn execute_instruction(&mut self, opcode: u8) {
        match opcode {
            0x88 => {
                let franny = Instruction::AdcAR8(Register::B);

            },
            None => panic!("natoehunt"),

        }
    }

    pub fn clock(&mut self) {
        if self.clock.cycles == 0 {
            // Fetch the opcode (for example from PC)
            let opcode = self.memory.read(self.registers.pc);
            self.registers.pc += 1;
            self.execute_instruction(opcode);
        }
    }
}
