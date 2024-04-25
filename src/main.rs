
use clap::Parser;

use chip8::*;

fn main() {
	let cli = Cli::parse();
	unsafe {
		debug_mode = cli.debug;
	}
	let rom_file = cli.rom;


	println_debug!("Constructing interpreter");
	let interpreter = Chip8::new();

	println_debug!("Loading rom ({})", rom_file);
	interpreter.load_rom();

	println!("Starting");
}

// Argument parsing stuff
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Cli {

	/// ROM file to execute
	rom: String,

    /// Print debug information
    #[arg(short, long)]
    debug: bool,
}