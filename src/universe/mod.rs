mod star;
mod tree;

use std::f32::consts::PI;

use glam::{Vec3, Vec2};
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

    return (v1 * s * 200.).abs();
}


fn sgn(f:f32) -> f32 {
    if f > 0. { 1. }
    else if f < 0. { -1. }
    else { 0. }
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
        else {
            if v.x < 0. {
                if v.y >= 0. { (v.y/v.x).atan() + PI }
                else { (v.y/v.x).atan() - PI }
            }
            else {
                if v.y > 0. { PI/2. }
                else if v.y < 0. { -PI/2. }
                else { f32::NAN }
            }
        }
    }
}   

impl Universe {
    pub fn new() -> Self {
        Universe{nb_stars:0, stars:vec![], black_holes:vec![]}
    }

    fn star_line(&mut self, p1:Vec3, p2:Vec3, density:usize) {
        for i in 0..density {
            let mut dist = p1 - p2;
            dist /= density as f32 - 1.;
            self.stars.push(Star::new(p2 + dist * i as f32, Vec3::ZERO, 1.))
        }
    }

    pub fn init_cube(&mut self) {
        self.stars.clear();
        let a = Vec3 {x:-100., y:-100., z:100.};
        let b = Vec3 {x:100., y:-100., z:100.};
        let c = Vec3 {x:-100., y:100., z:100.};
        let d = Vec3 {x:100., y:100., z:100.};
        self.star_line(a, b, 100);
        self.star_line(a, c, 100);
        self.star_line(b, d, 100);
        self.star_line(c, d, 100);
        let a1 = Vec3 {x:-100., y:-100., z:200.};
        let b1 = Vec3 {x:100., y:-100., z:200.};
        let c1 = Vec3 {x:-100., y:100., z:200.};
        let d1 = Vec3 {x:100., y:100., z:200.};
        self.star_line(a1, b1, 10);
        self.star_line(a1, c1, 10);
        self.star_line(b1, d1, 10);
        self.star_line(c1, d1, 10);

        self.star_line(a, a1, 10);
        self.star_line(b, b1, 10);
        self.star_line(c, c1, 10);
        self.star_line(d, d1, 10);


    }

    pub fn add_galaxy(&mut self, position:Vec3, nb_stars: usize, bh_mass_ratio:f32) {
        let black_hole = Star::new(position, Vec3::ZERO, nb_stars as f32 * bh_mass_ratio);
        self.nb_stars += nb_stars;
        for _i in 0..self.nb_stars {
            let polar = Vec3::new(gen_gaussian(), random::<f32>() * 2.* PI, PI/2.);
            let mut carthesian = to_carthesian(&polar);
            carthesian.x += position.x;
            carthesian.y += position.y;
            let mut mov = polar;
            mov.y = mov.y + 90.;
            mov.x = ((black_hole.get_mass() + self.nb_stars as f32/2.) /mov.x).sqrt();
            self.stars.push(Star::new(carthesian, Vec3::ZERO, 1.))
        }
        self.black_holes.push(black_hole);
    }

    pub fn update_attraction_black_hole(&mut self, time_step:f32) {
        for star in &mut self.stars {
            for black_hole in &self.black_holes {
                star.update_attraction(black_hole, time_step);

            }
        }
    }

    // pub fn update_attractions_naive(&mut self, time_step:f32){
    //     for i in 0..self.stars.len() {
    //         for j in 0..self.stars.len() {
    //             if i != j {
    //                 let tmp = self.stars[j];
    //                 self.stars[i].update_attraction(tmp, time_step)
    //             }
    //         }
    //     }
    // }

    pub fn update_attractions_tree(&mut self, time_step:f32) {
        let mut t = Tree::new(1000.);
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