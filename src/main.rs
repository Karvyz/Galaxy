extern crate sdl2;
mod universe;

use std::time::Duration;

use universe::Universe;
use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;

static HEIGHT:u32 = 1000;
static WIDTH:u32 = 1000;
 
pub fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
 
    let window = video_subsystem.window("rust-sdl2 demo", WIDTH, HEIGHT)
        .position_centered()
        .build()
        .unwrap();
 
    let mut canvas = window.into_canvas().build().unwrap();
 
    let mut universe = Universe::new();
    universe.init_stars((WIDTH as f32 / 2., HEIGHT as f32 / 2.),20.);

    let mut event_pump = sdl_context.event_pump().unwrap();
    'running: loop {
        let t = std::time::Instant::now();
        canvas.set_draw_color(Color::BLACK);
        canvas.clear();
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running
                },
                _ => {}
            }
        }
        
        canvas.set_draw_color(Color::RGBA(255, 255, 255, 10));
        universe.draw_stars(&mut canvas);

        canvas.present();
        universe.update_attractions();
        universe.update_positions(t.elapsed().as_secs_f32());
        // println!("fps : {}", 1./t.elapsed().as_secs_f32())
        // ::std::thread::sleep(Duration::new(10, 0));
    }
}
