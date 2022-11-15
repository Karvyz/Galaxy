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

    pub fn get_pos(&self) -> Vec2 {self.pos}
    pub fn get_mov(&self) -> Vec2 {self.mov}

    pub fn to_sdl_point(&self) -> sdl2::rect::Point {
        self.pos.to_sdl_point()
    }

    pub fn update_pos(&mut self, time_step:f32) {
        self.pos += self.mov * time_step
    }

    pub fn update_attraction(&mut self, s:Star) {
        self.mov += (s.pos - self.pos).normalize() / ((self.pos.distance_2(s.pos) + 1.));
    }

    // pub fn distance(&self, p: Star) -> f32 {
    //     self.pos.distance(p.pos)
    // }
}