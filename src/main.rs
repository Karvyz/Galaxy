#![deny(clippy::all)]
#![forbid(unsafe_code)]

use std::time::Instant;

use log::error;
use pixels::{Error, Pixels, SurfaceTexture};
use winit::dpi::LogicalSize;
use winit::event::{Event, VirtualKeyCode};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::WindowBuilder;
use winit_input_helper::WinitInputHelper;

mod universe;
use universe::Universe;

const WIDTH: u32 = 1000;
const HEIGHT: u32 = 1000;

// fn dim_frame(frame:&mut [u8]) {
//     for pixel in frame.chunks_exact_mut(4) {
//         for k in 0..3 {
//             let tmp = pixel[k] as i32 - 10 % 256;
//             pixel[k] = if tmp < 0 {0} else {tmp as u8};
//             // pixel[k] = (pixel[k] - 10) % 255;
//         }
//     }
// }

fn clear_frame(frame:&mut [u8]) {
    for pixel in frame.chunks_exact_mut(4) {
        pixel[0] = 0x00; // R
        pixel[1] = 0x00; // G
        pixel[2] = 0x00; // B
        pixel[3] = 0xff; // A
    } 
}

fn main() -> Result<(), Error> {
    println!("{}", 99/100);

    env_logger::init();
    let event_loop = EventLoop::new();
    let mut input = WinitInputHelper::new();
    let window = {
        let size = LogicalSize::new(WIDTH as f64, HEIGHT as f64);
        WindowBuilder::new()
            .with_title("Galaxy")
            .with_inner_size(size)
            .with_min_inner_size(size)
            .build(&event_loop)
            .unwrap()
    };

    let mut pixels = {
        let window_size = window.inner_size();
        let surface_texture = SurfaceTexture::new(window_size.width, window_size.height, &window);
        Pixels::new(WIDTH, HEIGHT, surface_texture)?
    };

    let mut universe = Universe::init(100000, ((WIDTH/2) as f32, (HEIGHT/2) as f32), 10.);

    let mut timer = std::time::Instant::now();
    let mut frame_counter = 0;

    event_loop.run(move |event, _, control_flow| {
        
        // Draw the current frame
        if let Event::RedrawRequested(_) = event {
            frame_counter += 1;
            let buffer = pixels.get_frame_mut();
            // dim_frame(buffer);
            clear_frame(buffer);
            universe.draw_stars(buffer,WIDTH,HEIGHT);
            let refresh_timing = 1./120.;
            universe.update_attraction_black_hole(refresh_timing);
            universe.update_attractions_tree(refresh_timing);
            universe.update_positions(refresh_timing);
            if pixels
                .render()
                .map_err(|e| error!("pixels.render() failed: {}", e))
                .is_err()
            {
                *control_flow = ControlFlow::Exit;
                return;
            }
        }

        // Handle input events
        if input.update(&event) {
            // Close events
            if input.key_pressed(VirtualKeyCode::Escape) || input.quit() {
                *control_flow = ControlFlow::Exit;
                return;
            }

            // Resize the window
            if let Some(size) = input.window_resized() {
                pixels.resize_surface(size.width, size.height);
            }

            // Update internal state and request a redraw
            window.request_redraw();
            if timer.elapsed().as_secs_f32() > 1. {
                println!("fps : {}", frame_counter);
                frame_counter = 0;
                timer = Instant::now();
            }

        }
    });
}