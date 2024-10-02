mod camera;
mod universe;

use std::time::Instant;

use camera::Camera;
use glam::Vec3;
use minifb::{Key, Window, WindowOptions};
use universe::Universe;

const WIDTH: usize = 960;
const HEIGHT: usize = 600;
const TARGET_FPS: usize = 1200;

fn main() {
    let mut buffer: Vec<u32> = vec![0; WIDTH * HEIGHT];

    let mut window = Window::new(
        "Test - ESC to exit",
        WIDTH,
        HEIGHT,
        WindowOptions::default(),
    )
    .unwrap();
    window.set_target_fps(TARGET_FPS);

    let universe = Universe::new();
    let mut camera = Camera::default(WIDTH as u32, HEIGHT as u32, universe);
    camera.add_galaxy();

    let mut total_time = 0.0;
    let mut nb_frames = 0;

    while window.is_open() && !window.is_key_down(Key::Escape) {
        let start_time = Instant::now();
        input_handler(&mut camera, &window);
        let update_start = Instant::now();
        camera.update_game(1. / (TARGET_FPS as f32));
        println!("Update time: {:?}", update_start.elapsed());
        let print_start = Instant::now();
        Camera::clear_buffer(&mut buffer);
        camera.update_buffer(&mut buffer);
        let window_start = Instant::now();
        window.update_with_buffer(&buffer, WIDTH, HEIGHT).unwrap();
        println!("Window time: {:?}", window_start.elapsed());
        println!("Print time: {:?}", print_start.elapsed());
        let elapsed_time = start_time.elapsed().as_secs_f32();
        total_time += elapsed_time;
        println!("FPS: {}", 1. / elapsed_time);
        nb_frames += 1;
    }

    println!("Average FPS: {}", nb_frames as f32 / total_time);
}

fn input_handler(camera: &mut Camera, window: &Window) {
    if window.is_key_down(Key::W) {
        camera.movement(Vec3::NEG_Z);
    }
    if window.is_key_down(Key::S) {
        camera.movement(Vec3::Z);
    }
    if window.is_key_down(Key::A) {
        camera.movement(Vec3::X);
    }
    if window.is_key_down(Key::D) {
        camera.movement(Vec3::NEG_X);
    }
    if window.is_key_down(Key::Space) {
        camera.movement(Vec3::Y);
    }
    if window.is_key_down(Key::LeftShift) {
        camera.movement(Vec3::NEG_Y);
    }
    if window.is_key_down(Key::Apostrophe) {
        camera.rotation(Vec3::Z);
    }
    if window.is_key_down(Key::L) {
        camera.rotation(Vec3::NEG_Z);
    }
    if window.is_key_down(Key::P) {
        camera.rotation(Vec3::Y);
    }
    if window.is_key_down(Key::M) {
        camera.rotation(Vec3::NEG_Y);
    }
    if window.is_key_down(Key::N) {
        camera.add_galaxy();
    }
}
