use crate::universe::vec::Vec2;


#[derive(Clone, Copy, Debug)]
pub struct Star {
    mass: f32,
    pos: Vec2,
    mov: Vec2
}

impl Star {
    pub fn new(x: f32, y: f32, ofx:f32, ofy:f32, mass:f32) -> Self {
        return Star { pos: Vec2::new(x, y), mov: Vec2::new(ofx, ofy), mass };
    }

    pub fn newv(pos:Vec2, mov:Vec2, mass:f32) -> Self{
        Star { pos, mov, mass }
    }

    pub fn get_mass(&self) -> f32 {self.mass}
    pub fn get_pos(&self) -> Vec2 {self.pos}
    pub fn get_mov(&self) -> Vec2 {self.mov}

    pub fn set_mass(&mut self, mass:f32) {self.mass = mass}

    pub fn update_pos(&mut self, time_step:f32) {
        self.pos += self.mov * time_step
    }

    pub fn update_attraction(&mut self, s:Star, time_step:f32) {
        self.mov += (s.pos - self.pos).normalize() / ((self.pos.distance_2(s.pos) + 1.)) * s.mass * time_step;
    }

    pub fn update_attraction_vec(&mut self, t:(Vec2, f32), time_step:f32) {
        // println!("self{:?}",self.mov);
        // println!("othe{:?}",t.0);

        // println!("norm{:?}",(t.0 - self.pos).normalize());
        // println!("dist{:?}",(self.pos.distance_2(t.0) + 1.));
        let tmp = ((t.0 - self.pos).normalize() / ((self.pos.distance_2(t.0) + 1.))) * t.1 * time_step;
        self.mov += tmp;

    }

    // pub fn distance(&self, p: Star) -> f32 {
    //     self.pos.distance(p.pos)
    // }
}