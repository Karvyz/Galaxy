extern crate sdl2;
mod universe;

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
    universe.init_stars((WIDTH as f32 / 2., HEIGHT as f32 / 2.),100000);

    let mut event_pump = sdl_context.event_pump().unwrap();

    let mut start_time = std::time::Instant::now();
    let mut frame_counter = 0;

    'running: loop {
        let t = std::time::Instant::now();
        canvas.set_draw_color(Color::RGBA(0u8,0u8,0u8,1u8));
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
        
        canvas.set_draw_color(Color::RGBA(255, 255, 255, 17));
        universe.draw_stars(&mut canvas);

        canvas.present();
        universe.update_attractions_tree();
        // ::std::thread::sleep(Duration::new(1, 0));
        universe.update_positions(t.elapsed().as_secs_f32());
        // println!("fps : {}", 1./t.elapsed().as_secs_f32())
        // ::std::thread::sleep(Duration::new(3, 0));

        frame_counter += 1;
        if start_time.elapsed().as_secs() > 1 {
            println!("fps : {}",frame_counter - 1);
            universe.nb_stars();
            frame_counter = 0;
            start_time = std::time::Instant::now();
        }
        // return;
    }
}
