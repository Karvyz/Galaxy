use glam::Vec3A;

use crate::universe;

pub struct Camera {
    coord: Vec3A,
    direction: Vec3A,
}

impl Camera {
    pub fn default() -> Self {
        Camera {
            coord: Vec3A::new(0., 0., 0.),
            direction: Vec3A::new(0., 0., 1.),
        }
    }

    pub fn clear_frame(&self, frame: &mut [u32]) {
        for pixel in frame.iter_mut() {
            *pixel = 0;
        }
    }

    pub fn render(&self, universe: &universe::Universe, frame: &mut [u32], frame_size: (u32, u32)) {
        for star in universe.stars.iter() {
            let relative_position = star.coord - self.coord;
            let x = relative_position.dot(self.direction) as i32;
            let y = relative_position.cross(self.direction).dot(Vec3A::Y) as i32;
            // let z = relative_position.cross(Vec3A::Y).dot(self.direction) as i32;

            let half_width = frame_size.0 as i32 / 2;
            let half_height = frame_size.1 as i32 / 2;

            if x < -half_width || x >= half_width || y < -half_height || y >= half_height {
                continue;
            }

            let index = ((x + half_width) + (y + half_height) * frame_size.0 as i32) as usize;
            frame[index] = 0x00FF_FFFF;
        }
    }
}
