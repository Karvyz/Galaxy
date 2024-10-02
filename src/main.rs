use glam::Vec3A;
use std::num::NonZeroU32;
use std::rc::Rc;
use winit::event::{ElementState, Event, KeyEvent, WindowEvent};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::keyboard::{Key, NamedKey};
use winit::window::Window;

mod window;
use window::WinitAppBuilder;

mod camera;
use camera::Camera;

mod universe;

fn main() {
    let event_loop = EventLoop::new().unwrap();

    let universe = universe::Universe::new();
    let mut camera = Camera::default(universe);
    camera.add_galaxy();

    let mut timer = std::time::Instant::now();
    let mut fps = 0;

    let mut update_time = 0.;
    let mut display_time = 0.;

    let mut app = WinitAppBuilder::with_init(|elwt| {
        let window = {
            let window = elwt.create_window(Window::default_attributes());
            Rc::new(window.unwrap())
        };
        let context = softbuffer::Context::new(window.clone()).unwrap();
        let surface = softbuffer::Surface::new(&context, window.clone()).unwrap();

        (window, surface)
    })
    .with_event_handler(|state, event, elwt| {
        let (window, surface) = state;
        elwt.set_control_flow(ControlFlow::Poll);

        match event {
            Event::WindowEvent {
                window_id,
                event: WindowEvent::RedrawRequested,
            } if window_id == window.id() => {
                let (width, height) = {
                    let size = window.inner_size();
                    (size.width, size.height)
                };
                surface
                    .resize(
                        NonZeroU32::new(width).unwrap(),
                        NonZeroU32::new(height).unwrap(),
                    )
                    .unwrap();
                let start = std::time::Instant::now();
                camera.update_game(1. / 200.);
                update_time += start.elapsed().as_secs_f32();
                let start = std::time::Instant::now();
                let mut buffer = surface.buffer_mut().unwrap();
                camera.clear_frame(buffer.as_mut());
                camera.draw_stars(buffer.as_mut(), width, height);

                buffer.present().unwrap();
                window.request_redraw();
                display_time += start.elapsed().as_secs_f32();

                fps += 1;
                if timer.elapsed().as_secs() >= 1 {
                    println!("FPS: {}", fps);
                    println!("Update time: {}", update_time * 1000. / fps as f32);
                    println!("Display time: {}", display_time * 1000. / fps as f32);
                    fps = 0;
                    update_time = 0.;
                    display_time = 0.;
                    timer = std::time::Instant::now();
                }
            }
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                window_id,
            } if window_id == window.id() => {
                elwt.exit();
            }
            Event::WindowEvent {
                event: WindowEvent::KeyboardInput { event, .. },
                window_id,
            } if window_id == window.id() => {
                handle_key_event(event, window, &mut camera);
            }
            _ => {}
        }
    });

    event_loop.run_app(&mut app).unwrap();
}

fn handle_key_event(event: KeyEvent, _window: &Window, camera: &mut Camera) {
    match event {
        KeyEvent {
            logical_key: key,
            state: ElementState::Pressed,
            ..
        } => match key.as_ref() {
            Key::Character("w") => {
                camera.movement(Vec3A::new(0., 0., -1.));
            }
            Key::Character("s") => {
                camera.movement(Vec3A::new(0., 0., 1.));
            }
            Key::Character("a") => {
                camera.movement(Vec3A::new(1., 0., 0.));
            }
            Key::Character("d") => {
                camera.movement(Vec3A::new(-1., 0., 0.));
            }
            Key::Named(NamedKey::Space) => {
                camera.movement(Vec3A::new(0., 1., 0.));
            }
            Key::Named(NamedKey::Shift) => {
                camera.movement(Vec3A::new(0., -1., 0.));
            }
            _ => {}
        },
        KeyEvent {
            logical_key: key,
            state: ElementState::Released,
            ..
        } => match key.as_ref() {
            Key::Character("w") => {
                println!("released w")
            }
            Key::Character("s") => {
                println!("released s")
            }
            _ => {}
        },
    }
}
