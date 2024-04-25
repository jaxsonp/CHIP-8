

mod font;

const MEM_SIZE: usize = 4096; // in bytes
const STACK_SIZE: usize = 16;
const N_REGISTERS: usize = 16;

const PROGRAM_START_ADDR: u16 = 0x200;

const DISPLAY_W: usize = 64;
const DISPLAY_H: usize = 32;

pub static mut debug_mode: bool = false;

#[allow(non_snake_case)]
pub struct Chip8 {
	memory: [u8; MEM_SIZE],
	display_buf: [[bool; DISPLAY_W]; DISPLAY_H],
	pc: u16,    // program counter
	I: u16,	// index register
	stack: [u16; STACK_SIZE],
	delay_t: u8, // delay timer
	sound_t: u8, // sound timer
	V: [u8; N_REGISTERS], // registers
}

impl Chip8 {
	pub fn new() -> Self {

		Self {
			memory: [0; MEM_SIZE],
			display_buf: [[false; DISPLAY_W]; DISPLAY_H],
			pc: PROGRAM_START_ADDR,
			I: 0x0,
			stack: [0x0; STACK_SIZE],
			delay_t: 0,
			sound_t: 0,
			V: [0; N_REGISTERS]
		}
	}

	pub fn load_rom(self) -> () {
		// Do stuff
	}
}

#[macro_export]
macro_rules! println_debug {
	($msg:literal) => {
		unsafe {
			if debug_mode {
				println!($msg);
			}
		}
    };
    ($msg:literal, $($args:tt),*) => {
		unsafe {
			if debug_mode {
				println!($msg, $($args),*);
			}
		}
    };
}