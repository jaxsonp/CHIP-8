#![allow(non_snake_case)]

use std::fs::File;
use std::io::prelude::*;
use std::ops::{BitAndAssign, BitOrAssign, BitXorAssign};
use std::path::Path;
use std::time::{Instant, Duration};
use std::thread::sleep;
use pixels::{Pixels, wgpu::Color};
use rand::Rng;

mod font;
use font::FONT;
mod display;

const MEM_SIZE: usize = 4096; // bytes
const N_REGISTERS: usize = 16;

const PROGRAM_START_ADDR: u16 = 0x200;
const FONT_ADDR: u16 = 0x050;

pub const DISPLAY_W: usize = 64;
pub const DISPLAY_H: usize = 32;
const REFRESH_RATE: usize = 60; // hz
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
	/// Function return stack
	stack: Vec<u16>,
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
			stack: Vec::new(),
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
		let time_per_instruction = Duration::from_secs_f64(1.0 / (self.ips as f64));
		let time_per_tick =  Duration::from_secs_f64(1.0 / (REFRESH_RATE as f64));
		let mut last_tick_time = Instant::now();
		loop {
			let start_time = Instant::now();

			let instruction = self.fetch_instruction();
			println_debug!("{:#05X} > {:#06X}", self.pc - 2, instruction);

			match self.decode_and_execute(instruction) {
				Ok(_) => {},
				Err(why) => {
					println!("Failed: {why}");
					break;
				}
			}

			if Instant::now() - last_tick_time > time_per_tick {
				// 60hz tick
				if self.pixel_buf_updated {
					self.render();
					self.pixel_buf_updated = false;
				}
				if self.delay_t > 0 {
					self.delay_t -= 1;
				}
				if self.sound_t > 0 {
					self.sound_t -= 1;
				}
				last_tick_time = Instant::now();
			}

			let elapsed = Instant::now() - start_time;
			if elapsed < time_per_instruction {
				sleep(time_per_instruction - elapsed);
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
							},
							(0xE, 0xE) => {
								// Return from subroutine
								self.pc = match self.stack.pop() {
									Some(pc) => pc,
									None => {
										return Err("Returned outside of subroutine");
									}
								}
							}
							_ => {
								return Err("Unknown instruction");
							},
						}
					},
					_ => {
						// Call machine code routine
						return Err("Unhandled instruction");
					},
				}
			},
			0x1 => {
				// Jump
				self.pc = NNN;
			},
			0x2 => {
				// Call subroutine at NNN
				self.stack.push(self.pc);
				self.pc = NNN;
			},
			0x3 => {
				// Skip if VX == NN
				if self.V[X] == NN {
					self.pc += 2;
				}
			},
			0x4 => {
				// Skip if VX != NN
				if self.V[X] != NN {
					self.pc += 2;
				}
			},
			0x5 => {
				// Skip if VX == VY
				if self.V[X] == self.V[Y] {
					self.pc += 2;
				}
			},
			0x6 => {
				// Set register to value
				self.V[X] = NN;
			},
			0x7 => {
				// Add value to register
				self.V[X] = self.V[X].wrapping_add(NN);
			},
			0x8 => {
				match nibbles[3] {
					0x0 => {
						// Assign VX = VY
						self.V[X] = self.V[Y];
					},
					0x1 => {
						// Assign VX |= VY (bitwise or)
						self.V[X].bitor_assign(self.V[Y]);
					},
					0x2 => {
						// Assign VX &= VY (bitwise and)
						self.V[X].bitand_assign(self.V[Y]);
					},
					0x3 => {
						// Assign VX ^= VY (bitwise xor)
						self.V[X].bitxor_assign(self.V[Y]);
					},
					0x4 => {
						// Assign VX += VY
						self.V[X] = self.V[X].wrapping_add(self.V[Y]);
					},
					0x5 => {
						// Assign VX -= VY
						self.V[X] = self.V[X].wrapping_sub(self.V[Y]);
					},
					0x6 => {
						// Bitshift right VX >>= 1
						// (store overflow in VF)
						self.V[0xF] = self.V[X] & 0x1;
						self.V[X] = self.V[X].wrapping_shr(1);
						self.V[X] &= 0x7F;

					},
					0x7 => {
						// Assign VX = VY - VX
						// (set VF to 0 if underflow, 1 otherwise)
						self.V[0xF] = if self.V[X] <= self.V[Y] { 1 } else { 0 };
						self.V[X] = self.V[Y] - self.V[X];

					},
					0xE => {
						// Bitshift left VX <<= 1
						// (store overflow in VF)
						self.V[X] = self.V[X].wrapping_shl(1);
						self.V[0xF] = self.V[X] & 0x1;
						self.V[X] &= 0xFE;
					},
					_ => {
						return Err("Unknown instruction");
					},
				}
			},
			0x9 => {
				// Skip if VX != VY
				if self.V[X] != self.V[Y] {
					self.pc += 2;
				}
			},
			0xA => {
				// Set index register
				self.I = NNN;
			},
			0xB => {
				// Jump to NNN + V0
				self.I = NNN + (self.V[0] as u16);
			},
			0xC => {
				// Rand gen
				// Sets VX to random u8 & NN
				self.V[X] = rand::thread_rng().gen::<u8>() & NN;
			},
			0xD => {
				// Draw
				// draws an 8 wide, N tall sprite at VX, VY from the memory location at I
				let sprite_x = self.V[X] as usize;
				let sprite_y = self.V[Y] as usize;
				let mut unset_pixel = false;
				for row in 0..(N as usize) {

					let mut pixel_values = self.memory[addr!(self.I) + row];
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
				self.pixel_buf_updated = true;
			},
			0xF => {
				match (nibbles[2], nibbles[3]) {
					(0x0, 0x7) => {
						// Set VS to delay timer value
						self.V[X] = self.delay_t;
					},
					(0x1, 0x5) => {
						// Set delay timer to VX
						self.delay_t = self.V[X];
					},
					(0x1, 0x8) => {
						// Set sound timer to VX
						self.sound_t = self.V[X];
					},
					(0x1, 0xE) => {
						// Increments I by VX
						self.I = self.I.wrapping_add(self.V[X] as u16);
					},
					(0x2, 0x9) => {
						// Set I to sprite location for char in VX
						self.I = FONT_ADDR + ((self.V[X] * 5) as u16)
					},
					(0x3, 0x3) => {
						// Binary coded decimal storage
						// Store VX's hundreds digit at I, tens at I+1, and ones at I+2
						self.memory[addr!(self.I)] = self.V[X].div_euclid(100);
						self.memory[addr!(self.I) + 1] = self.V[X].div_euclid(10) % 10;
						self.memory[addr!(self.I) + 2] = self.V[X] % 10;
					},
					(0x5, 0x5) => {
						// Register dump
						// Store V0, V1, ... VX at address I+0, I+1, ... I+X
						let addr = addr!(self.I);
						for i in 0..=X {
							self.memory[addr + i] = self.V[i];
						}
					},
					(0x6, 0x5) => {
						// Register load
						// Move values from I+0, I+1, ... I+X in V0, V1, ... VX
						let addr = addr!(self.I);
						for i in 0..=X {
							self.V[i] = self.memory[addr + i];
						}
					},
					_ => {
						return Err("Unknown instruction");
					},
				}
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

/// Converting u16 addresses to usize, masking the first 12 bits
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