mod star;
mod vec;
mod tree;

use std::time::Instant;

use star::Star;
use rand::random;
use crate::universe::tree::Tree;

use self::vec::Vec2;

pub struct Universe {
    nb_stars: usize,
    stars:Vec<Star>,
    bh: Star
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

impl Universe {
    pub fn init(nb_stars: usize, center:(f32,f32), bh_mass_ratio:f32) -> Self {
        let mut universe = Universe{nb_stars, stars:vec![], bh:Star::new(center.0, center.1, 0., 10., nb_stars as f32 * bh_mass_ratio)};
        universe.init_stars();
        universe
    }

    pub fn nb_stars(&self) { println!("stars : {}", self.stars.len())}


    pub fn init_stars(&mut self) {
        self.stars.clear();
        self.stars.reserve_exact(self.nb_stars);
        for _i in 0..self.nb_stars {
            let polar = Vec2::new(gen_gaussian(), random::<f32>() * 360.);
            let mut carthesian = polar.to_carthesian();
            carthesian = carthesian.add_x(self.bh.get_pos().get_x());
            carthesian = carthesian.add_y(self.bh.get_pos().get_y());
            let mut mov = polar;
            mov.set_y(mov.get_y() + 90.);
            // mov.set_y(if y > 2. * PI {y - 2. * PI} else {y});
            mov.set_x(((self.bh.get_mass() + self.nb_stars as f32/2.) /mov.get_x()).sqrt());
            self.stars.push(Star::newv(carthesian, mov.to_carthesian(), 1.))
        }
    }

    pub fn update_attraction_black_hole(&mut self, time_step:f32) {
        for star in &mut self.stars {
            star.update_attraction(self.bh, time_step);
        }
    }

    pub fn update_attractions_naive(&mut self, time_step:f32){
        for i in 0..self.stars.len() {
            for j in 0..self.stars.len() {
                if i != j {
                    let tmp = self.stars[j];
                    self.stars[i].update_attraction(tmp, time_step)
                }
            }
        }
    }

    pub fn update_attractions_tree(&mut self, time_step:f32) {
        let mut t = Tree::new(1000.);
        for star in &self.stars {
            t.insert(star.clone());
        }
        t.update_tree();
        // println!("{:#?}", t);
        t.compute_interactions(time_step);
        self.stars.clear();
        self.stars.reserve_exact(self.nb_stars);
        self.stars = t.get_updated_stars();
    }

    pub fn update_positions(&mut self, time_step:f32) {
        for star in &mut self.stars {
            star.update_pos(time_step);
        }
    }

    pub fn draw_stars(&self, frame:&mut [u8], width:u32, height:u32) {
        // let color = [0xFF, 0xFF, 0xFF, 0xFF];
        let color = [0x60, 0x40, 0x80, 0xFF];

        for star in &self.stars {
            if star.get_pos().get_x() >= 0. && star.get_pos().get_y() >= 0. && star.get_pos().get_x() < width as f32 && star.get_pos().get_y() < height as f32 - 1.{
                let i = star.get_pos().get_y() as usize * width as usize + star.get_pos().get_x() as usize;
                for k in 0..3 {
                    if i as usize * 4 + k > (4 * width * height) as usize {println!("{:?} , {} , {}",star, i, i as usize)}

                    let mut nc = frame[i* 4 + k] as u16;
                    nc += color[k];
                    frame[i* 4 + k] = if nc > 255 {255} else {nc as u8};
                }
            }
        }
        // let i = self.bh.get_pos().get_y() * (width as f32) + self.bh.get_pos().get_x();
        // for k in 0..3 {
        //     let mut nc = frame[i as usize * 4 + k] as u16;
        //     nc += color[k];
        //     frame[i as usize * 4 + k] = if nc > 255 {255} else {nc as u8};
        // }
    }
}