mod star;

use glam::Vec3A;
use rand::Rng;
use star::Star;

pub struct Universe {
    pub stars: Vec<Star>,
}

impl Universe {
    pub fn new() -> Self {
        Universe { stars: Vec::new() }
    }

    // pub fn add_star(&mut self, star: Star) {
    //     self.stars.push(star);
    // }

    pub fn add_galaxy(&mut self, nb_stars: usize) {
        let mut rng = rand::thread_rng();
        for _ in 0..nb_stars {
            self.stars.push(
                Star::builder()
                    .coord(Vec3A::new(
                        rng.gen_range(-100.0..100.0),
                        rng.gen_range(-100.0..100.0),
                        rng.gen_range(-100.0..100.0),
                    ))
                    .build(),
            );
        }
    }

    pub fn update(&mut self, dt: f32) {
        for i in 0..self.stars.len() {
            for j in (i + 1)..self.stars.len() {
                let distance = self.stars[i].distance(&self.stars[j]);
                let acceleration = 1. / (distance * distance);
                let direction = (self.stars[j].coord - self.stars[i].coord).normalize();
                self.stars[i].speed += direction * acceleration * dt;
                self.stars[j].speed -= direction * acceleration * dt;
            }
        }
        for star in self.stars.iter_mut() {
            star.update();
        }
    }
}
