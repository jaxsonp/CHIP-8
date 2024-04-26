

use winit::{
    dpi::LogicalSize,
    event::{Event, WindowEvent},
    event_loop::EventLoop,
    window::{Window, WindowBuilder},
};
use winit_input_helper::WinitInputHelper;
use pixels::{Pixels, SurfaceTexture};

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
        let size = LogicalSize::new((DISPLAY_W * PIXEL_SIZE) as u32, (DISPLAY_H * PIXEL_SIZE) as u32);
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
        let mut pixels = Pixels::new(DISPLAY_W as u32, DISPLAY_H as u32, surface_texture).unwrap();
        //pixels.resize_buffer(DISPLAY_W as u32, DISPLAY_H as u32).unwrap();
        pixels
    }

    pub fn run_event_loop(self) -> () {
        println_debug!("Starting window event loop");
        self.event_loop.run(|event, window_target| {
            /*if let Event::RedrawRequested(_) = event {
                world.draw(pixels.frame_mut());
                if let Err(err) = pixels.render() {
                    log_error("pixels.render", err);
                    *control_flow = ControlFlow::Exit;
                    return;
                }
            }*/
            if let Event::WindowEvent { event, .. } = event {
                if event == WindowEvent::CloseRequested {
                    window_target.exit();
                }
            }
        }).unwrap();
        println_debug!("Window event loop exited");
    }

    pub fn set_window_title(&mut self, new_title: String) {
        self.window.set_title(new_title.as_str())
    }
}