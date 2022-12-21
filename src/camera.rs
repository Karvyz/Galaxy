use std::f32::consts::PI;

use glam::{Vec3, Vec2};

use crate::universe::{Universe, to_polar, to_carthesian};

pub struct Camera {
    fov: f32,

    height:usize,
    width:usize,
    aspect_ratio: f32,
    znear:f32,

    universe: Universe
}

impl Camera {
    pub fn default(height:u32, width:u32, universe:Universe) -> Self {
        Camera { fov: 120., aspect_ratio:1., znear:10., universe, height: height.try_into().unwrap(), width: width.try_into().unwrap() }
    }

    pub fn display(&self, frame:&mut [u8]) {
        self.clear_frame(frame);
        self.draw_stars(frame);
    }

    pub fn update_game(&mut self, refresh_timing:f32) {
        self.universe.update_attraction_black_hole(refresh_timing);
        self.universe.update_attractions_tree(refresh_timing);
        self.universe.update_positions(refresh_timing);
    }

    pub fn movement(&mut self, movment_vector:Vec3) {
        for star in &mut self.universe.stars {
            star.pos += movment_vector;
        }
        for black_hole in &mut self.universe.black_holes {
            black_hole.pos += movment_vector;
        }
    }

    pub fn rotation(&mut self, roation_vector:Vec3) {
        let r = roation_vector * PI/180.;
        for star in &mut self.universe.stars {
            let mut pos_spherical = to_polar(&star.pos);
            pos_spherical += r;
            star.pos = to_carthesian(&pos_spherical);
        }
        for black_hole in &mut self.universe.black_holes {
            let mut pos_spherical = to_polar(&black_hole.pos);
            pos_spherical += r;
            black_hole.pos = to_carthesian(&pos_spherical);
        }
    }

    pub fn direction(&mut self, mut direction_vector:Vec3) {
        direction_vector *= 0.01;
        for star in &mut self.universe.stars {
            if direction_vector.x != 0. {
                let x = direction_vector.x.cos() * star.pos.x + (-direction_vector.x.sin() * star.pos.z);
                let z = direction_vector.x.sin() * star.pos.x + (direction_vector.x.cos() * star.pos.z);
                star.pos.x = x;
                star.pos.z = z;
            }
            if direction_vector.y != 0. {
                let y = direction_vector.y.cos() * star.pos.y + (-direction_vector.y.sin() * star.pos.z);
                let z = direction_vector.y.sin() * star.pos.y + (direction_vector.y.cos() * star.pos.z);
                star.pos.y = y;
                star.pos.z = z;
            }
        }
        for star in &mut self.universe.stars {
            if direction_vector.x != 0. {
                let x = direction_vector.x.cos() * star.mov.x + (-direction_vector.x.sin() * star.mov.z);
                let z = direction_vector.x.sin() * star.mov.x + (direction_vector.x.cos() * star.mov.z);
                star.mov.x = x;
                star.mov.z = z;
            }
            if direction_vector.y != 0. {
                let y = direction_vector.y.cos() * star.mov.y + (-direction_vector.y.sin() * star.mov.z);
                let z = direction_vector.y.sin() * star.mov.y + (direction_vector.y.cos() * star.mov.z);
                star.mov.y = y;
                star.mov.z = z;
            }
        }
        for black_hole in &mut self.universe.black_holes {
            if direction_vector.x != 0. {
                let x = direction_vector.x.cos() * black_hole.pos.x + (-direction_vector.x.sin() * black_hole.pos.z);
                let z = direction_vector.x.sin() * black_hole.pos.x + (direction_vector.x.cos() * black_hole.pos.z);
                black_hole.pos.x = x;
                black_hole.pos.z = z;
            }
            if direction_vector.y != 0. {
                let y = direction_vector.y.cos() * black_hole.pos.y + (-direction_vector.y.sin() * black_hole.pos.z);
                let z = direction_vector.y.sin() * black_hole.pos.y + (direction_vector.y.cos() * black_hole.pos.z);
                black_hole.pos.y = y;
                black_hole.pos.z = z;
            }
        }
    }


    fn clear_frame(&self, frame:&mut [u8]) {
        for pixel in frame.chunks_exact_mut(4) {
            pixel[0] = 0x00; // R
            pixel[1] = 0x00; // G
            pixel[2] = 0x00; // B
            pixel[3] = 0xff; // A
        } 
    }

    fn to_screen(&self, pos:Vec2) -> Vec2 {
        pos * 500. + 500.
    }

    fn draw_stars(&self, frame:&mut [u8]) {

        // let color = [0xFF, 0xFF, 0xFF, 0xFF];
        let color = [0x60, 0x40, 0x80, 0xFF];

        let scaling_factor:f32 = 1./((self.fov/2.).to_radians().tan());
        for star in &self.universe.stars {

            if star.pos.z > self.znear {

                let mut projected_coord = Vec2 { x: self.aspect_ratio * scaling_factor * star.get_pos().x, y: scaling_factor * star.get_pos().y};
                projected_coord /= star.get_pos().z;

                if projected_coord.x < 1. && projected_coord.x > -1. && projected_coord.y < 1. && projected_coord.y > -1. {

                    projected_coord = self.to_screen(projected_coord);
                    let i = projected_coord.y as usize * self.width + projected_coord.x as usize;

                    if i < self.width * self.height {
                        for k in 0..3 {
                            let mut nc = frame[i* 4 + k] as u16;
                            nc += color[k];
                            frame[i* 4 + k] = if nc > 255 {255} else {nc as u8};
                        }
                    }
                }
            }
        }
    }
}