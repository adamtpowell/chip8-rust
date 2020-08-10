
use crate::screen::{Screen, RGBA};
use std::{fs};

pub const WIDTH: u32 = 640;
pub const HEIGHT: u32 = 480;

// TODO: Move controller to another module which handles all the controller layouts, so different emulator modules could
// use different controllers.
pub struct HexPad {
    pub zero: bool,
    pub one: bool,
    pub two: bool,
    pub three: bool,
    pub four: bool,
    pub five: bool,
    pub six: bool,
    pub seven: bool,
    pub eight: bool,
    pub nine: bool,
    pub a: bool,
    pub b: bool,
    pub c: bool,
    pub d: bool,
    pub e: bool,
    pub f: bool,
}

pub struct Emulator {
    time: f32,
    memory: [u8; 4096],
    V: [u8; 16],
    I: u16,
    PC: u16,
    SP: u8,
    stack: [u16; 16],
    delay: u8,
    sound: u8,
}

/*
from cowgod's guide:
nnn or addr - A 12-bit value, the lowest 12 bits of the instruction
n or nibble - A 4-bit value, the lowest 4 bits of the instruction
x - A 4-bit value, the lower 4 bits of the high byte of the instruction
y - A 4-bit value, the upper 4 bits of the low byte of the instruction
kk or byte - An 8-bit value, the lowest 8 bits of the instruction
*/

struct Chip8Instruction {
    instruction: u16,
}
impl Chip8Instruction {
    fn get_nnn(&self) -> u16 {
        self.instruction & 0x0FFF
    }
    fn get_n(&self) -> u8 {
        (self.instruction & 0x000F) as u8
    }
    fn get_x(&self) -> u8 {
        ((self.instruction & 0x0F00) >> 8) as u8 
    }
    fn get_y(&self) -> u8 {
        ((self.instruction & 0x00F0) >> 4) as u8
    }
    fn get_kk(&self) -> u8 {
        (self.instruction & 0x00FF) as u8
    }
    fn get_nibble1(&self) -> u8 {
        ((self.instruction & 0xF000) >> 12) as u8
    }
    fn get_nibble2(&self) -> u8 {
        ((self.instruction & 0x0F00) >> 8) as u8
    }
    fn get_nibble3(&self) -> u8 {
        ((self.instruction & 0x00F0) >> 4) as u8
    }
    fn get_nibble4(&self) -> u8 {
        (self.instruction & 0x000F) as u8
    }
}
impl From<(u8, u8)> for Chip8Instruction {
    fn from(byte: (u8, u8)) -> Self {
        return Chip8Instruction {
            instruction: ((byte.0 as u16) << 8 | (byte.1 as u16)),
        }
    }
}

// It looks like it is a standard the the interpreter starts at 200, so code should be inserted into memory starting there.
impl Emulator {
    pub fn new(filename: &str) -> Self {
        let mut memory: [u8; 4096] = [0; 4096];
        // todo: fill screen and bottom memory with garbage for the aesthetic
        for (i, byte) in fs::read(filename).unwrap().iter().enumerate() {
            if (memory.len() > 0x200 + i) {
                memory[0x200 + i] = *byte;
            }
        }
        Emulator {
            time: 0.0,
            memory,
            V: [0; 16],
            I: 0,
            PC: 0x200,
            SP: 0,
            stack: [0; 16],
            delay: 0,
            sound: 0,
        }
    }

    fn run_instruction<S: Screen>(&mut self, inputs:&HexPad, screen: &mut S) {
        
        let instruction: Chip8Instruction = (self.memory[self.PC as usize], self.memory[self.PC as usize + 1]).into();
        println!("{} {} {} {}", instruction.get_nibble1(), instruction.get_nibble2(), instruction.get_nibble3(), instruction.get_nibble4());
        println!("{} {} {} {}", instruction.get_nnn(), instruction.get_x(), instruction.get_y(), instruction.get_kk());
    }

    pub fn update<S: Screen>(&mut self, inputs: &HexPad, screen: &mut S) {
        self.PC += 1;
        if self.PC as usize + 1 >= self.memory.len() {
            self.PC = 0; // PC overflows
        }
        self.run_instruction(inputs, screen);
    }
}