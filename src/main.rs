use std::thread;
use std::sync::{ Arc, Mutex };
use clap::Parser;

pub mod display;
use display::Display;
use chip8::*;

fn main() {
    // cli arg parsing
    let cli = Cli::parse();
    unsafe {
        DEBUG_ENABLED = cli.debug;
    }
    let rom_file = cli.rom;
    let ips = cli.ips;
    println_debug!("IPS:\t{}", ips);
    println_debug!("ROM:\t{}", rom_file);
    println_debug!("Debug:\tyes");

    let mut display = Display::create_window();
    display.set_window_title(format!("CHIP-8  -  {}", rom_file));

    let mut emulator = Chip8::new(ips, display.create_pixel_buf());
    match emulator.load_rom(&rom_file) {
        Ok(_) => {}
        Err(_) => {
            println!("Failed to load ROM");
            return;
        }
    }

    let keypad_state: Arc<Mutex<[bool; 16]>> = Arc::new(Mutex::new([false; 16]));
    let keypad_state2 = keypad_state.clone();
    //let (key_tx, key_rx) = mpsc::channel();

    // starting emulator thread
    match
        thread::Builder
            ::new()
            .name("emulator_thread".to_string())
            .spawn(move || {
                emulator.run(keypad_state2);
            })
    {
        Err(e) => {
            println!("Failed to spawn emulator thread: {e}");
            return;
        }
        Ok(_) => {}
    }
    display.run_event_loop(keypad_state);
}

// Argument parsing stuff
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Cli {
    /// ROM file to execute
    rom: String,

    /// Instructions per second
    #[arg(long, default_value_t = 700)]
    ips: usize,

    /// Print debug information
    #[arg(short, long)]
    debug: bool,
}
