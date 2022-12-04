use std::ops;


#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub struct Vec2 {
    x:f32,
    y:f32
}

impl Vec2 {
    pub fn new(x:f32, y:f32) -> Self{
        Vec2{x,y}
    }
	pub fn set_x(&mut self, x:f32) {self.x = x}
	pub fn set_y(&mut self, y:f32) {self.y = y}
	pub fn get_x(&self) -> f32 {self.x}
	pub fn get_y(&self) -> f32 {self.y}

	pub fn distance_2(&self, v:Vec2) -> f32 {
		f32::powf(v.x - self.x, 2.) + f32::powf(v.y - self.y, 2.)
	}

	pub fn distance(&self, v:Vec2) -> f32 {
        f32::sqrt(self.distance_2(v))
    }

	pub fn normalize(&self) -> Self{
		Vec2 { x: self.x / self.distance(Vec2::new(0., 0.)), y: self.y / self.distance(Vec2::new(0., 0.))}
	}

	pub fn add_x(&self, f:f32) -> Self {Vec2{x:self.x + f, y:self.y}}
	pub fn add_y(&self, f:f32) -> Self {Vec2{x:self.x, y:self.y + f}}

	pub fn sup_eq(&self, v:Vec2) -> bool {self.x >= v.x && self.y >= v.y}
	pub fn inf(&self, v:Vec2) -> bool {self.x < v.x && self.y < v.y}


	pub fn to_carthesian(&self) -> Self {
		Vec2 { x: self.x * self.y.to_radians().cos(), y: self.x * self.y.to_radians().sin() }
	}

	pub fn to_polar(&self) -> Self {
		Vec2 { x: (self.x * self.x + self.y * self.y).sqrt() , y: (self.y / self.x).tan() }
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

impl ops::Add<f32> for Vec2 {
	type Output = Vec2;

    fn add(self, other: f32) -> Vec2 {
		Vec2 {
			x : self.x + other,
			y : self.y + other,
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