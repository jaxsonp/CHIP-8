#![allow(dead_code)]
use std::sync::{ Arc, Mutex };
use rand::distributions::Open01;
use winit::{
    dpi::{ LogicalSize, PhysicalSize },
    event::{ ElementState, Event, KeyEvent, WindowEvent },
    event_loop::EventLoop,
    keyboard::{ Key, KeyCode, PhysicalKey },
    window::{ Window, WindowBuilder },
};
use pixels::{ Pixels, SurfaceTexture };

use crate::*;

pub const PIXEL_SIZE: usize = 15;

pub struct Display {
    size: LogicalSize<u32>,
    event_loop: EventLoop<()>,
    window: Window,
}

impl Display {
    pub fn create_window() -> Self {
        println_debug!("Configuring window");
        env_logger::init();
        let size = LogicalSize::new((SCREEN_W * PIXEL_SIZE) as u32, (SCREEN_H * PIXEL_SIZE) as u32);
        println_debug!(" - Size: {} x {}", size.width, size.height);
        let event_loop = EventLoop::new().unwrap();
        let window = WindowBuilder::new()
            .with_title("CHIP-8")
            .with_decorations(true)
            .with_inner_size(size)
            .with_resizable(false)
            .build(&event_loop)
            .unwrap();
        Self {
            size,
            event_loop,
            window,
        }
    }

    pub fn create_pixel_buf(&self) -> Pixels {
        let surface_texture = SurfaceTexture::new(self.size.width, self.size.height, &self.window);
        let pixels = Pixels::new(SCREEN_W as u32, SCREEN_H as u32, surface_texture).unwrap();
        pixels
    }

    pub fn run_event_loop(self, keypad_state: Arc<Mutex<[bool; 16]>>) -> () {
        let update_keypad = |key: usize, pressed: bool| {
            let mut keypad_state = keypad_state.lock().unwrap();
            (*keypad_state)[key] = pressed;
        };

        println_debug!("Starting window event loop");
        self.event_loop
            .run(|event, window_target| {
                match event {
                    Event::WindowEvent { window_id, event } => {
                        match event {
                            WindowEvent::CloseRequested => {
                                window_target.exit();
                            }
                            WindowEvent::KeyboardInput {
                                event: KeyEvent {
                                    physical_key,
                                    logical_key,
                                    text,
                                    location,
                                    state: ElementState::Pressed,
                                    repeat: false,
                                    ..
                                },
                                ..
                            } => {
                                match physical_key {
                                    PhysicalKey::Code(KeyCode::Digit1) => update_keypad(0x1, true),
                                    PhysicalKey::Code(KeyCode::Digit2) => update_keypad(0x2, true),
                                    PhysicalKey::Code(KeyCode::Digit3) => update_keypad(0x3, true),
                                    PhysicalKey::Code(KeyCode::Digit4) => update_keypad(0xc, true),
                                    PhysicalKey::Code(KeyCode::KeyQ) => update_keypad(0x4, true),
                                    PhysicalKey::Code(KeyCode::KeyW) => update_keypad(0x5, true),
                                    PhysicalKey::Code(KeyCode::KeyE) => update_keypad(0x6, true),
                                    PhysicalKey::Code(KeyCode::KeyR) => update_keypad(0xd, true),
                                    PhysicalKey::Code(KeyCode::KeyA) => update_keypad(0x7, true),
                                    PhysicalKey::Code(KeyCode::KeyS) => update_keypad(0x8, true),
                                    PhysicalKey::Code(KeyCode::KeyD) => update_keypad(0x9, true),
                                    PhysicalKey::Code(KeyCode::KeyF) => update_keypad(0xe, true),
                                    PhysicalKey::Code(KeyCode::KeyZ) => update_keypad(0xa, true),
                                    PhysicalKey::Code(KeyCode::KeyX) => update_keypad(0x0, true),
                                    PhysicalKey::Code(KeyCode::KeyC) => update_keypad(0xb, true),
                                    PhysicalKey::Code(KeyCode::KeyV) => update_keypad(0xf, true),
                                    _ => {}
                                }
                            }
                            WindowEvent::KeyboardInput {
                                event: KeyEvent {
                                    physical_key,
                                    logical_key,
                                    text,
                                    location,
                                    state: ElementState::Released,
                                    repeat: false,
                                    ..
                                },
                                ..
                            } => {
                                match physical_key {
                                    PhysicalKey::Code(KeyCode::Digit1) => update_keypad(0x1, false),
                                    PhysicalKey::Code(KeyCode::Digit2) => update_keypad(0x2, false),
                                    PhysicalKey::Code(KeyCode::Digit3) => update_keypad(0x3, false),
                                    PhysicalKey::Code(KeyCode::Digit4) => update_keypad(0xc, false),
                                    PhysicalKey::Code(KeyCode::KeyQ) => update_keypad(0x4, false),
                                    PhysicalKey::Code(KeyCode::KeyW) => update_keypad(0x5, false),
                                    PhysicalKey::Code(KeyCode::KeyE) => update_keypad(0x6, false),
                                    PhysicalKey::Code(KeyCode::KeyR) => update_keypad(0xd, false),
                                    PhysicalKey::Code(KeyCode::KeyA) => update_keypad(0x7, false),
                                    PhysicalKey::Code(KeyCode::KeyS) => update_keypad(0x8, false),
                                    PhysicalKey::Code(KeyCode::KeyD) => update_keypad(0x9, false),
                                    PhysicalKey::Code(KeyCode::KeyF) => update_keypad(0xe, false),
                                    PhysicalKey::Code(KeyCode::KeyZ) => update_keypad(0xa, false),
                                    PhysicalKey::Code(KeyCode::KeyX) => update_keypad(0x0, false),
                                    PhysicalKey::Code(KeyCode::KeyC) => update_keypad(0xb, false),
                                    PhysicalKey::Code(KeyCode::KeyV) => update_keypad(0xf, false),
                                    _ => {}
                                }
                            }
                            _ => {}
                        }
                    }
                    _ => {}
                }
            })
            .unwrap();
        println_debug!("Window event loop exited");
    }

    pub fn set_window_title(&mut self, new_title: String) {
        self.window.set_title(new_title.as_str())
    }
}
