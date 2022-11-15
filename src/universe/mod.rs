mod star;
mod vec;
mod tree;

use star::Star;
use rand::random;
use sdl2::{render::Canvas, video::Window};
use crate::universe::tree::Tree;

pub struct Universe {
    stars:Vec<Star>
}

impl Universe {
    pub fn new() -> Self {
        Universe{stars:vec![]}
    }

    pub fn init_stars(&mut self, center:(f32,f32), density:f32) {
        self.stars.clear();
        let c = vec::Vec2::new(center.0, center.1);
        for y in 0..1000{
            for x in 0..1000 {
                let t = vec::Vec2::new(x as f32, y as f32);
                if random::<f32>() < density/f32::powf(c.distance(t), 2.) {
                    self.stars.push(Star::new(x as f32, y as f32, random::<f32>() * 2. - 1.,random::<f32>() * 2. - 1.))
                }
            }
        }
        // println!("{} stars", self.stars.len())
    }

    pub fn update_attractions(&mut self){
        let mut t = Tree::new(1000.);
        for star in &self.stars {
            t.insert(star.clone());
        }
        t.update_tree();
        // t.compute_interactions()
        // println!("{:#?}", t);
        // println!("size : {}", t.get_nb_stars());
        for i in 0..self.stars.len() {
            for j in 0..self.stars.len() {
                if i != j {
                    let tmp = self.stars[j];
                    self.stars[i].update_attraction(tmp)
                }
            }
        }
    }
    pub fn update_positions(&mut self, time_step:f32) {
        for star in &mut self.stars {
            star.update_pos(time_step)
        }
    }

    pub fn draw_stars(&self, canvas:&mut Canvas<Window>) {
        for point in &self.stars{
            canvas.draw_point(point.to_sdl_point()).expect("ya un point qya buggé");
        }
    }
}