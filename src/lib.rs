#![allow(non_snake_case)]

use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use pixels::{Pixels, wgpu::Color};

// TODO Delete
use std::time::Duration;
use std::thread::sleep;

mod font;
use font::FONT;
mod display;

const MEM_SIZE: usize = 4096; // in bytes
const STACK_SIZE: usize = 16;
const N_REGISTERS: usize = 16;

const PROGRAM_START_ADDR: u16 = 0x200;
const FONT_ADDR: u16 = 0x050;

pub const DISPLAY_W: usize = 64;
pub const DISPLAY_H: usize = 32;
const ON_COLOR: [u8; 4] = [0xCD, 0xDA, 0xFF, 0xFF];
const OFF_COLOR: [u8; 4] = [0x00, 0x0C, 0x1C, 0xFF];


pub static mut DEBUG_ENABLED: bool = false;

pub struct Chip8 {
	memory: [u8; MEM_SIZE],
	display_buf: [[bool; DISPLAY_W]; DISPLAY_H],
	pixels: Option<Pixels>,
	pc: u16,    // program counter
	I: u16,	// index register
	stack: [u16; STACK_SIZE],
	delay_t: u8, // delay timer
	sound_t: u8, // sound timer
	V: [u8; N_REGISTERS], // registers
}

impl Chip8 {
	pub fn new() -> Self {
		println_debug!("Initializing emulator");
		let mut memory = [0; MEM_SIZE];

		// loading font to memory
		println_debug!("Loading font");
		for i in 0..FONT.len() {
			let addr = addr!(FONT_ADDR) + i;
			memory[addr] = FONT[i];
		}

		Self {
			memory,
			display_buf: [[false; DISPLAY_W]; DISPLAY_H],
			pixels: None,
			pc: PROGRAM_START_ADDR,
			I: 0x0,
			stack: [0x0; STACK_SIZE],
			delay_t: 0,
			sound_t: 0,
			V: [0; N_REGISTERS]
		}
	}

	pub fn load_rom(&mut self) -> () {
		println_debug!("Loading rom")
		// Do stuff
	}

	pub fn attach_pixel_buf(&mut self, pixels: Pixels) -> () {
		println_debug!("Attaching to display");
		self.pixels = Some(pixels);
		self.pixels.as_mut().unwrap().clear_color(Color {
			r: OFF_COLOR[0] as f64,
			g: OFF_COLOR[1] as f64,
			b: OFF_COLOR[2] as f64,
			a: OFF_COLOR[3] as f64,
		});
		self.display_buf[1][1] = true;
		self.render();
	}

	pub fn run(&mut self) -> () {
		println_debug!("Starting execution");
		let mut i = 0;
		loop {
			let instruction = self.fetch_instruction();
			let X = (instruction & 0x0F00).wrapping_shr(16);
			let Y = (instruction & 0x00F0).wrapping_shr(8);
			let N = instruction & 0x000F;
			let NN = instruction & 0x00FF;
			let NNN = instruction & 0x0FFF;
		}
	}

	fn fetch_instruction(&mut self) -> u16 {
		let instruction = (self.memory[addr!(self.pc)] as u16) << 8 | (self.memory[addr!(self.pc + 1)] as u16);
		self.pc = self.pc.wrapping_add(2);
		instruction
	}

	/// Applies the current display buffer (display_buf) to the screen
	fn render(&mut self) {
		let pixels = match &mut self.pixels {
			Some(pixels) => {
				pixels
			},
			None => {
				println!("Render before window initialization");
				return;
			}
		};
		for (i, pixel) in pixels.frame_mut().chunks_exact_mut(4).enumerate() {
            let x = (i % DISPLAY_W as usize) as usize;
            let y = (i / DISPLAY_W as usize) as usize;
            let rgba = if self.display_buf[y][x] {
                ON_COLOR
            } else {
                OFF_COLOR
            };

            pixel.copy_from_slice(&rgba);
        }
		pixels.render().unwrap();
	}
}

// converting u16 addresses to usize
#[macro_export]
macro_rules! addr {
	($num:expr) => {
		($num & 0x0FFF) as usize
	};
}

// debugging messages that can be enabled/disabled
#[macro_export]
macro_rules! println_debug {
	($msg:literal) => {
		unsafe {
			if DEBUG_ENABLED {
				println!($msg);
			}
		}
    };
    ($msg:literal, $($args:expr),*) => {
		unsafe {
			if DEBUG_ENABLED {
				println!($msg, $($args),*);
			}
		}
    };
}