mod star;
mod vec;
mod tree;

use std::f32::consts::PI;

use star::Star;
use rand::random;
use sdl2::{render::Canvas, video::Window};
use crate::universe::tree::Tree;

use self::vec::Vec2;

pub struct Universe {
    stars:Vec<Star>
}

fn gen_gaussian() -> f32 {
    let mut s = 1.;
    let mut v1 = 0.;
    while s >= 1.0 || s == 0. {
        v1 = 2.0 * random::<f32>() - 1.0;
        let v2 = 2.0 * random::<f32>() - 1.0;
        s = v1 * v1 + v2 * v2;
    } ;
    s = f32::sqrt((-2.0 * s.log(10.)) / s);
 
    return (v1 * s) * 100.;
}

impl Universe {
    pub fn new() -> Self {
        
        Universe{stars:vec![]}
    }

    pub fn nb_stars(&self) { println!("stars : {}", self.stars.len())}


    pub fn init_stars(&mut self, center:(f32,f32), n_stars:i32) {
        self.stars.clear();
        for _i in 0..n_stars {
            let polar = Vec2::new(gen_gaussian(), random::<f32>() * PI);
            let mut carthesian = polar.to_carthesian();
            carthesian = carthesian.add_x(center.0);
            carthesian = carthesian.add_y(center.1);
            let mut mov = polar;
            let y = mov.get_y() + PI/2.;
            mov.set_y(if y > 2. * PI {y - 2. * PI} else {y});
            mov.set_x(0.);
            self.stars.push(Star::newv(carthesian, mov.to_carthesian()))
        }
    }

    // pub fn update_attractions_naive(&mut self){


    //     for i in 0..self.stars.len() {
    //         for j in 0..self.stars.len() {
    //             if i != j {
    //                 let tmp = self.stars[j];
    //                 self.stars[i].update_attraction(tmp)
    //             }
    //         }
    //     }
    // }

    pub fn update_attractions_tree(&mut self) {
        let mut t = Tree::new(1000.);
        for star in &self.stars {
            t.insert(star.clone());
        }
        t.update_tree();
        // println!("{:#?}", t);
        t.compute_interactions();
        self.stars.clear();
        self.stars = t.get_updated_stars();
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