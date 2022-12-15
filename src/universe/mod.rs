mod star;
mod tree;

use glam::Vec2;
use star::Star;
use rand::random;
use crate::universe::tree::Tree;


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

fn to_carthesian(v:Vec2) -> Vec2 {
    Vec2 { x: v.x * v.y.to_radians().cos(), y: v.x * v.y.to_radians().sin() }
}

impl Universe {
    pub fn init(nb_stars: usize, center:(f32,f32), bh_mass_ratio:f32) -> Self {
        let mut universe = Universe{nb_stars, stars:vec![], bh:Star::new(center.0, center.1, 0., 10., nb_stars as f32 * bh_mass_ratio)};
        universe.init_stars();
        universe
    }

    pub fn init_stars(&mut self) {
        self.stars.clear();
        self.stars.reserve_exact(self.nb_stars);
        for _i in 0..self.nb_stars {
            let polar = Vec2::new(gen_gaussian(), random::<f32>() * 360.);
            let mut carthesian = to_carthesian(polar);
            carthesian.x += self.bh.get_pos().x;
            carthesian.y += self.bh.get_pos().y;
            let mut mov = polar;
            mov.y = mov.y + 90.;
            // mov.set_y(if y > 2. * PI {y - 2. * PI} else {y});
            mov.x = ((self.bh.get_mass() + self.nb_stars as f32/2.) /mov.x).sqrt();
            self.stars.push(Star::newv(carthesian, to_carthesian(mov), 1.))
        }
    }

    pub fn update_attraction_black_hole(&mut self, time_step:f32) {
        for star in &mut self.stars {
            star.update_attraction(self.bh, time_step);
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
        }
    }

    pub fn draw_stars(&self, frame:&mut [u8], width:u32, height:u32) {
        // let color = [0xFF, 0xFF, 0xFF, 0xFF];
        let color = [0x60, 0x40, 0x80, 0xFF];

        for star in &self.stars {
            if star.get_pos().x >= 0. && star.get_pos().y >= 0. && star.get_pos().x < width as f32 && star.get_pos().y < height as f32 - 1.{
                let i = star.get_pos().y as usize * width as usize + star.get_pos().x as usize;
                for k in 0..3 {
                    if i as usize * 4 + k > (4 * width * height) as usize {println!("{:?} , {} , {}",star, i, i as usize)}

                    let mut nc = frame[i* 4 + k] as u16;
                    nc += color[k];
                    frame[i* 4 + k] = if nc > 255 {255} else {nc as u8};
                }
            }
        }
    }
}