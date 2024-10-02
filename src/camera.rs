use std::f32::consts::PI;

use glam::{Vec2, Vec3A};

use crate::universe::{to_carthesian, to_polar, Universe};

const LUMINOSITY: f32 = 0.3;
const COLOR: [f32; 3] = [96. * LUMINOSITY, 64. * LUMINOSITY, 128. * LUMINOSITY];

pub struct Camera {
    fov: f32,
    znear: f32,
    universe: Universe,
}

impl Camera {
    pub fn default(universe: Universe) -> Self {
        Camera {
            fov: 120.,
            znear: 1.,
            universe,
        }
    }

    pub fn update_game(&mut self, refresh_timing: f32) {
        self.universe.update_attractions_black_holes(refresh_timing);
        self.universe.update_attractions_tree(refresh_timing);
        self.universe.update_positions(refresh_timing);
    }

    pub fn add_galaxy(&mut self) {
        self.universe.add_galaxy(Vec3A::Z * 100., 1000000, 0.1);
    }

    pub fn movement(&mut self, movment_vector: Vec3A) {
        for star in &mut self.universe.stars {
            star.pos += movment_vector;
        }
        for black_hole in &mut self.universe.black_holes {
            black_hole.pos += movment_vector;
        }
    }

    pub fn rotation(&mut self, roation_vector: Vec3A) {
        let r = roation_vector * PI / 180.;
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

    pub fn direction(&mut self, mut direction_vector: Vec3A) {
        direction_vector *= 0.01;
        for star in &mut self.universe.stars {
            if direction_vector.x != 0. {
                let x = direction_vector.x.cos() * star.pos.x
                    + (-direction_vector.x.sin() * star.pos.z);
                let z =
                    direction_vector.x.sin() * star.pos.x + (direction_vector.x.cos() * star.pos.z);
                star.pos.x = x;
                star.pos.z = z;
            }
            if direction_vector.y != 0. {
                let y = direction_vector.y.cos() * star.pos.y
                    + (-direction_vector.y.sin() * star.pos.z);
                let z =
                    direction_vector.y.sin() * star.pos.y + (direction_vector.y.cos() * star.pos.z);
                star.pos.y = y;
                star.pos.z = z;
            }
        }
        for star in &mut self.universe.stars {
            if direction_vector.x != 0. {
                let x = direction_vector.x.cos() * star.mov.x
                    + (-direction_vector.x.sin() * star.mov.z);
                let z =
                    direction_vector.x.sin() * star.mov.x + (direction_vector.x.cos() * star.mov.z);
                star.mov.x = x;
                star.mov.z = z;
            }
            if direction_vector.y != 0. {
                let y = direction_vector.y.cos() * star.mov.y
                    + (-direction_vector.y.sin() * star.mov.z);
                let z =
                    direction_vector.y.sin() * star.mov.y + (direction_vector.y.cos() * star.mov.z);
                star.mov.y = y;
                star.mov.z = z;
            }
        }
        for black_hole in &mut self.universe.black_holes {
            if direction_vector.x != 0. {
                let x = direction_vector.x.cos() * black_hole.pos.x
                    + (-direction_vector.x.sin() * black_hole.pos.z);
                let z = direction_vector.x.sin() * black_hole.pos.x
                    + (direction_vector.x.cos() * black_hole.pos.z);
                black_hole.pos.x = x;
                black_hole.pos.z = z;
            }
            if direction_vector.y != 0. {
                let y = direction_vector.y.cos() * black_hole.pos.y
                    + (-direction_vector.y.sin() * black_hole.pos.z);
                let z = direction_vector.y.sin() * black_hole.pos.y
                    + (direction_vector.y.cos() * black_hole.pos.z);
                black_hole.pos.y = y;
                black_hole.pos.z = z;
            }
        }
    }

    pub fn clear_frame(&self, frame: &mut [u32]) {
        for pixel in frame {
            *pixel = 0;
        }
    }

    #[inline]
    fn to_screen(&self, pos: Vec2, height: u32, width: u32) -> Vec2 {
        Vec2 {
            x: pos.x * width as f32 / 2. + width as f32 / 2.,
            y: pos.y * height as f32 / 2. + height as f32 / 2.,
        }
    }

    pub fn draw_stars(&self, frame: &mut [u32], width: u32, height: u32) {
        let scaling_factor: f32 = 1. / ((self.fov / 2.).to_radians().tan());
        let aspect_ratio = height as f32 / width as f32;
        for star in &self.universe.stars {
            if star.pos.z > self.znear {
                let mut projected_coord = Vec2 {
                    x: aspect_ratio * scaling_factor * star.get_pos().x,
                    y: scaling_factor * star.get_pos().y,
                };
                projected_coord /= star.get_pos().z;

                if projected_coord.x < 1.
                    && projected_coord.x > -1.
                    && projected_coord.y < 1.
                    && projected_coord.y > -1.
                {
                    projected_coord = self.to_screen(projected_coord, height, width);
                    let i = projected_coord.y as u32 * width + projected_coord.x as u32;

                    if i < width * height {
                        let index = i as usize;
                        let base_color = frame[index];
                        let mut red = (base_color >> 16) & 0xFF;
                        let mut green = (base_color >> 8) & 0xFF;
                        let mut blue = base_color & 0xFF;

                        if green < 255 {
                            green += ((COLOR[1] / star.pos.z) * 100.) as u32;
                            if green > 255 {
                                green = 255;
                            }
                            if red < 255 {
                                red += ((COLOR[0] / star.pos.z) * 100.) as u32;
                                if red > 255 {
                                    red = 255;
                                }
                                if blue < 255 {
                                    blue += ((COLOR[2] / star.pos.z) * 100.) as u32;
                                    if blue > 255 {
                                        blue = 255;
                                    }
                                }
                            }
                        }

                        frame[index] = ((red & 0xFF) << 16) | ((green & 0xFF) << 8) | (blue & 0xFF);
                    }
                }
            }
        }
    }
}
