use glam::{self, Vec3A};

#[derive(Clone, Copy, Debug)]
pub struct Star {
    pub mass: f32,
    pub pos: Vec3A,
    pub mov: Vec3A,
}

impl Star {
    pub fn new(pos: Vec3A, mov: Vec3A, mass: f32) -> Self {
        Star { pos, mov, mass }
    }

    #[inline]
    pub fn get_mass(&self) -> f32 {
        self.mass
    }
    #[inline]
    pub fn get_pos(&self) -> Vec3A {
        self.pos
    }

    #[inline]
    pub fn update_pos(&mut self, time_step: f32) {
        self.pos += self.mov * time_step
    }

    #[inline]
    pub fn update_attraction(&mut self, s: &Star, time_step: f32) {
        self.mov += (s.pos - self.pos).normalize() / (self.pos.distance_squared(s.pos) + 1.)
            * s.mass
            * time_step;
    }

    #[inline]
    pub fn update_attraction_vec(&mut self, t: (Vec3A, f32), time_step: f32) {
        let tmp = ((t.0 - self.pos).normalize() / (self.pos.distance_squared(t.0) + 1.))
            * t.1
            * time_step;
        self.mov += tmp;
    }

    // pub fn distance(&self, p: Star) -> f32 {
    //     self.pos.distance(p.pos)
    // }
}
