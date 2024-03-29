#![forbid(unsafe_code)]

use std::time::Instant;

use glam::Vec3;
use pixels::{Error, Pixels, SurfaceTexture};
use winit::event::{Event, VirtualKeyCode};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::{WindowBuilder, Fullscreen};
use winit_input_helper::WinitInputHelper;

mod camera;
use camera::Camera;
mod universe;
use universe::Universe;

fn main() -> Result<(), Error> {

    let event_loop = EventLoop::new();
    let mut input = WinitInputHelper::new();
    let window = {
        WindowBuilder::new()
            .with_title("Galaxy")
            .with_fullscreen(Some(Fullscreen::Borderless(None)))
            .build(&event_loop)
            .unwrap()
    };

    let window_size = window.current_monitor().unwrap().size();

    let mut pixels = {
        let surface_texture = SurfaceTexture::new(window_size.width, window_size.height, &window);
        Pixels::new(window_size.width, window_size.height, surface_texture)?
    };

    let universe = Universe::new();
    let mut camera = Camera::default(window_size.width, window_size.height, universe);

    let mut timer = std::time::Instant::now();
    let mut frame_counter = 0;

    event_loop.run(move |event, _, control_flow| {

        // Draw the current frame
        if let Event::RedrawRequested(_) = event {
            frame_counter += 1;
            let buffer = pixels.get_frame_mut();
            // dim_frame(buffer);
            camera.display(buffer);
            let refresh_timing = 1./120.;
            camera.update_game(refresh_timing);
            if pixels
                .render()
                .map_err(|e| eprintln!("pixels.render() failed: {e}"))
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

            game_key_pressed(&mut camera, &input);

            // Resize the window
            if let Some(size) = input.window_resized() {
                pixels.resize_surface(size.width, size.height);
            }

            // Update internal state and request a redraw
            window.request_redraw();
            if timer.elapsed().as_secs_f32() > 1. {
                println!("fps : {frame_counter}");
                frame_counter = 0;
                timer = Instant::now();
            }

        }
    });
}


fn game_key_pressed(camera:&mut Camera, input:&WinitInputHelper) {

    // Movement
    if input.key_held(VirtualKeyCode::Z) {
        camera.movement(Vec3::NEG_Z);
    }

    if input.key_held(VirtualKeyCode::S) {
        camera.movement(Vec3::Z);
    }

    if input.key_held(VirtualKeyCode::Q) {
        camera.movement(Vec3::X);
    }

    if input.key_held(VirtualKeyCode::D) {
        camera.movement(Vec3::NEG_X);
    }

    if input.held_shift() {
        camera.movement(Vec3::NEG_Y);
    }

    if input.key_held(VirtualKeyCode::Space) {
        camera.movement(Vec3::Y);
    }


    // Camera rotation
    if input.key_held(VirtualKeyCode::A) {
        camera.rotation(Vec3::Z);
    }

    if input.key_held(VirtualKeyCode::E) {
        camera.rotation(Vec3::NEG_Z);
    }


    // Fuckup everything
    if input.key_held(VirtualKeyCode::P) {
        camera.rotation(Vec3::Y);
    }

    if input.key_held(VirtualKeyCode::M) {
        camera.rotation(Vec3::NEG_Y);
    }

    // Camera direction
    if input.key_held(VirtualKeyCode::J) {
        camera.direction(Vec3::NEG_X);
    }

    if input.key_held(VirtualKeyCode::L) {
        camera.direction(Vec3::X);
    }

    if input.key_held(VirtualKeyCode::I) {
        camera.direction(Vec3::NEG_Y);
    }

    if input.key_held(VirtualKeyCode::K) {
        camera.direction(Vec3::Y);
    }

    if input.key_pressed(VirtualKeyCode::N) {
        camera.add_galaxy();
    }

}