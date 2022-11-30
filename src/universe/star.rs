use crate::universe::vec::Vec2;


#[derive(Clone, Copy, Debug)]
pub struct Star {
    pos: Vec2,
    mov: Vec2
}

impl Star {
    pub fn new(x: f32, y: f32, ofx:f32, ofy:f32) -> Self {
        return Star { pos: Vec2::new(x, y), mov: Vec2::new(ofx, ofy) };
    }

    pub fn newv(pos:Vec2, mov:Vec2) -> Self{
        Star { pos, mov }
    }

    pub fn get_pos(&self) -> Vec2 {self.pos}
    pub fn get_mov(&self) -> Vec2 {self.mov}

    pub fn to_sdl_point(&self) -> sdl2::rect::Point {
        self.pos.to_sdl_point()
    }

    pub fn update_pos(&mut self, time_step:f32) {
        self.pos += self.mov * time_step
    }

    pub fn update_attraction(&mut self, s:Star) {
        self.mov += (s.pos - self.pos).normalize() / ((self.pos.distance_2(s.pos) + 10.));
    }

    pub fn update_attraction_vec(&mut self, t:(Vec2, f32)) {
        // println!("self{:?}",self.mov);
        // println!("othe{:?}",t.0);

        // println!("norm{:?}",(t.0 - self.pos).normalize());
        // println!("dist{:?}",(self.pos.distance_2(t.0) + 1.));
        let tmp = ((t.0 - self.pos).normalize() / ((self.pos.distance_2(t.0) + 10.))) * t.1;
        self.mov += tmp;

    }

    // pub fn distance(&self, p: Star) -> f32 {
    //     self.pos.distance(p.pos)
    // }
}