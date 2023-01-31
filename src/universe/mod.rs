mod star;
mod tree;

use std::f32::consts::PI;

use glam::Vec3;
pub use star::Star;
use rand::random;
use crate::universe::tree::Tree;


pub struct Universe {
    nb_stars: usize,
    pub stars:Vec<Star>,
    pub black_holes: Vec<Star>,
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

    v1 * s
}

pub fn to_carthesian(v:&Vec3) -> Vec3 {
    Vec3 {  
        x: v.x * v.y.sin() * v.z.cos(),
        y: v.x * v.y.sin() * v.z.sin(),
        z: v.x * v.y.cos()
    }
}

pub fn to_polar(v:&Vec3) -> Vec3 {
    Vec3 {
        x: (v.x * v.x + v.y * v.y + v.z * v.z).sqrt(),

        y: if v.z != 0. {
            if v.z > 0. { ((v.x * v.x + v.y * v.y).sqrt() / v.z).atan()}
            else { ((v.x * v.x + v.y * v.y).sqrt() / v.z).atan() + PI}
        } 
        else if v.x * v.y != 0. {PI/2.}
        else {f32::NAN},

        z: if v.x > 0. { (v.y/v.x).atan()}
        else if v.x < 0. {
            if v.y >= 0. { (v.y/v.x).atan() + PI }
            else { (v.y/v.x).atan() - PI }
        }
        else if v.y > 0. { PI/2. }
        else if v.y < 0. { -PI/2. }
        else { f32::NAN }
    }
}   

impl Universe {
    pub fn new() -> Self {
        Universe{nb_stars:0, stars:vec![], black_holes:vec![]}
    }

    pub fn add_galaxy(&mut self, position_center:Vec3, nb_stars: usize, bh_mass_ratio:f32) {
        let black_hole = Star::new(position_center, Vec3::ZERO, nb_stars as f32 * bh_mass_ratio);
        self.nb_stars += nb_stars;
        for _i in 0..self.nb_stars {
            let pos_star = Vec3::new(gen_gaussian() * 100., gen_gaussian() * 100., gen_gaussian() * 20.);
            let mut spherical_pos = to_polar(&pos_star);
            spherical_pos.z += (90_f32).to_radians();
            spherical_pos.x = (black_hole.mass/spherical_pos.x).sqrt();
            let movement = to_carthesian(&spherical_pos);
            self.stars.push(Star::new(pos_star + position_center, movement, 1.))
        }
        self.black_holes.push(black_hole);
    }

    pub fn update_attractions_black_holes(&mut self, time_step:f32) {
        for star in &mut self.stars {
            for black_hole in &self.black_holes {
                star.update_attraction(black_hole, time_step);
            }
        }

        for i in 0..self.black_holes.len() {
            for j in 0..self.black_holes.len() {
                if i != j {
                    let b = self.black_holes[j];
                    self.black_holes[i].update_attraction(&b, time_step)
                }
            }
        }
    }

    pub fn update_attractions_tree(&mut self, time_step:f32) {
        let mut t = Tree::new(2000.);
        for i in 0..self.stars.len() {
            t.insert(&self.stars, i);
        }
        t.update_tree(&self.stars);
        t.compute_interactions(&mut self.stars, time_step);
    }

    pub fn update_positions(&mut self, time_step:f32) {
        for star in &mut self.stars {
            star.update_pos(time_step);
            // println!("{:?}", star)
        }
    }
}