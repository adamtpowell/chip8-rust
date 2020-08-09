
use crate::screen::{Screen, RGBA};
use std::{fs};
use byteorder::{LittleEndian, ReadBytesExt};

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
    code: Vec<u8>,
}

// TODO: Make emulator a trait and have a chip-8 that implements it.

impl Emulator {
    pub fn new(file: &str) -> Self {
        Emulator {
            time: 0.0,
            code: fs::read("address.txt").unwrap(),
        }
    }

    pub fn update<S: Screen>(&mut self, inputs: &HexPad, screen: &mut S) {
        
    }
}