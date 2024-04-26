#![allow(non_snake_case)]

use std::fs::File;
use std::io::prelude::*;
use std::ops::Shr;
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
	/// Instructions per second
	ips: usize,
	/// Memory (4KB)
	memory: [u8; MEM_SIZE],
	/// Pixel buffer
	pixel_buf: [[bool; DISPLAY_W]; DISPLAY_H],
	/// Pixel buffer updated flag (used to optimize rendering)
	pixel_buf_updated: bool,
	/// Pixels object (used for rendering)
	pixels: Pixels,
	/// Program counter
	pc: u16,
	/// Index register
	I: u16,
	/// Stack
	stack: [u16; STACK_SIZE],
	/// Stack index
	stack_i: usize,
	/// Delay timer
	delay_t: u8,
	/// Sound timer
	sound_t: u8,
	/// Registers V0-VF
	V: [u8; N_REGISTERS],
}

impl Chip8 {
	pub fn new(ips: usize, pixels: Pixels) -> Self {
		println_debug!("Initializing emulator");
		let mut memory = [0; MEM_SIZE];

		// loading font to memory
		println_debug!("Loading font");
		for i in 0..FONT.len() {
			memory[addr!(FONT_ADDR) + i] = FONT[i];
		}

		Self {
			ips,
			memory,
			pixel_buf: [[false; DISPLAY_W]; DISPLAY_H],
			pixel_buf_updated: false,
			pixels,
			pc: PROGRAM_START_ADDR,
			I: 0x0,
			stack: [0x0; STACK_SIZE],
			stack_i: 0,
			delay_t: 0,
			sound_t: 0,
			V: [0; N_REGISTERS]
		}
	}

	pub fn load_rom(&mut self, path_str: &String) -> Result<(), ()> {
		println_debug!("Loading ROM");

		let rom_path = Path::new(path_str.as_str());
    	println_debug!(" - Path: {}", rom_path.display());

		let mut file: File = match File::open(rom_path) {
			Err(why) => {
				println_debug!(" - Failed to open file {}: {}", rom_path.display(), why);
				return Err(());
			},
			Ok(file) => file,
		};

		let mut addr = addr!(PROGRAM_START_ADDR);
		let mut total_bytes = 0;
		loop {
			let bytes_read = file.read(&mut self.memory[addr..(addr + 1)]).expect(" - Failed to read ");
			if bytes_read == 0 { break };
			total_bytes += bytes_read;
			addr += 1;
		}
		println_debug!(" - Read {} bytes", total_bytes);
		Ok(())
	}

	/// Atarts execution cycle
	pub fn run(&mut self) -> () {
		// configuring pixel buffer
		self.pixels.clear_color(Color {
			r: OFF_COLOR[0] as f64,
			g: OFF_COLOR[1] as f64,
			b: OFF_COLOR[2] as f64,
			a: OFF_COLOR[3] as f64,
		});
		self.render();

		println_debug!("Starting execution");
		loop {

			let instruction = self.fetch_instruction();
			println_debug!("{:#05X} > {:#06X}", self.pc, instruction);

			match self.decode_and_execute(instruction) {
				Ok(_) => {},
				Err(why) => {
					println!("{why}");
					break;
				}
			}

			if instruction == 0x1228 {
				break;
			}
		}
		println_debug!("Completed execution");
	}

	fn fetch_instruction(&mut self) -> u16 {
		let instruction = (self.memory[addr!(self.pc)] as u16) << 8 | (self.memory[addr!(self.pc + 1)] as u16);
		self.pc = self.pc.wrapping_add(2);
		instruction
	}

	fn decode_and_execute(&mut self, instruction: u16) -> Result<(), &str> {

		let nibbles: [u16; 4] = [
			(instruction & 0xF000).checked_shr(12).unwrap(),
			(instruction & 0x0F00).checked_shr(8).unwrap(),
			(instruction & 0x00F0).checked_shr(4).unwrap(),
			instruction & 0x000F,
		];

		// Register identifiers
		let X = nibbles[1] as usize;
		let Y = nibbles[2] as usize;

		// Values/constants
		let N = (instruction & 0x000F) as u8;
		let NN = (instruction & 0x00FF) as u8;
		let NNN = instruction & 0x0FFF;

		// Instruction handling
		match nibbles[0] {
			0x0 => {
				match nibbles[1] {
					0x0 => {
						match (nibbles[2], nibbles[3]) {
							(0xE, 0x0) => {
								// Clear screen
								self.pixel_buf = [[false; DISPLAY_W]; DISPLAY_H];
								self.pixel_buf_updated = true;
								self.render(); // TODO delete
							},
							(0xE, 0xE) => {
								// Return
								// TODO
							}
							_ => {
								return Err("Unknown instruction");
							},
						}
					},
					_ => {
						return Err("Unknown instruction");
					},
				}
			},
			0x1 => {
				// Jump
				self.pc = NNN;
			},
			0x6 => {
				// Set register to value
				self.V[X] = NN;
			},
			0x7 => {
				// Add value to register
				self.V[X] += NN;
			},
			0xA => {
				// Set index register
				self.I = NNN;
			},
			0xD => {
				// Draw
				let sprite_x = self.V[X] as usize;
				let sprite_y = self.V[Y] as usize;
				let mut unset_pixel = false;
				println_debug!("Drawing sprite at {}, {}", sprite_x, sprite_y);
				for row in 0..(N as usize) {
					let mut pixel_values = self.memory[addr!(self.I) + row];
					println_debug!(" - {}: {:#04X}", row, pixel_values);
					for col in (0..8).rev() {
						if pixel_values & 0x1 == 1 {
							let old_value = self.pixel_buf[sprite_y + row][sprite_x + col];
							self.pixel_buf[sprite_y + row][sprite_x + col] = !old_value;
							if old_value == true { unset_pixel = true; }
						}
						pixel_values >>= 1;
					}
				}
				self.V[0xF] = if unset_pixel {1} else {0};
				self.render(); // TODO delte
			},
			_ => {
				return Err("Unknown instruction");
			},
		}

		Ok(())
	}

	/// Applies the current display buffer (pixel_buf) to the screen
	fn render(&mut self) {
		for (i, pixel) in self.pixels.frame_mut().chunks_exact_mut(4).enumerate() {
            let x = (i % DISPLAY_W as usize) as usize;
            let y = (i / DISPLAY_W as usize) as usize;
            let rgba = if self.pixel_buf[y][x] {
                ON_COLOR
            } else {
                OFF_COLOR
            };

            pixel.copy_from_slice(&rgba);
        }
		self.pixels.render().unwrap();
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