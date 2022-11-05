use std::ops;


#[derive(Clone, Copy, Debug)]
pub struct Vec2 {
    x:f32,
    y:f32
}

impl Vec2 {
    pub fn new(x:f32, y:f32) -> Self{
        Vec2{x,y}
    }

	pub fn distance_2(&self, v:Vec2) -> f32 {
		f32::powf(v.x - self.x, 2.) + f32::powf(v.y - self.y, 2.)
	}

	pub fn distance(&self, v:Vec2) -> f32 {
        f32::sqrt(self.distance_2(v))
    }

	pub fn normalize(&mut self) -> Self{
		self.x /= self.distance(Vec2::new(0., 0.));
		self.y /= self.distance(Vec2::new(0., 0.));
		*self
	}

	pub fn to_sdl_point(&self) -> sdl2::rect::Point {
        sdl2::rect::Point::new(self.x as i32, self.y as i32)
    }
}
impl ops::Add for Vec2 {
	type Output = Vec2;

    fn add(self, other: Vec2) -> Vec2 {
		Vec2 {
			x : self.x + other.x,
			y : self.y + other.y,
		}
	}
}
impl ops::AddAssign for Vec2 {
    fn add_assign(&mut self, other: Vec2) {
		self.x += other.x;
		self.y += other.y;
	}
}
impl ops::Sub for Vec2 {
	type Output = Vec2;

    fn sub(self, other: Vec2) -> Vec2 {
		Vec2 {
			x : self.x - other.x,
			y : self.y - other.y,
		}
	}
}
impl ops::Mul<f32> for Vec2 {
	type Output = Vec2;

    fn mul(self, other: f32) -> Vec2 {
		Vec2 {
			x : self.x * other,
			y : self.y * other,
		}
	}
}
impl ops::Div<f32> for Vec2 {
	type Output = Vec2;

    fn div(self, other: f32) -> Vec2 {
		Vec2 {
			x : self.x / other,
			y : self.y / other,
		}
	}
}