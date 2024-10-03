use glam::Vec3A;

pub struct Star {
    pub coord: Vec3A,
    pub speed: Vec3A,
}

impl Star {
    pub fn builder() -> StarBuilder {
        StarBuilder::new()
    }

    pub fn update(&mut self) {
        self.coord += self.speed;
    }

    pub fn distance(&self, other: &Star) -> f32 {
        self.coord.distance(other.coord)
    }
}

pub struct StarBuilder {
    coord: Vec3A,
    speed: Vec3A,
}

#[allow(dead_code)]
impl StarBuilder {
    pub fn new() -> Self {
        StarBuilder {
            coord: Vec3A::ZERO,
            speed: Vec3A::ZERO,
        }
    }

    pub fn coord(mut self, coord: Vec3A) -> Self {
        self.coord = coord;
        self
    }

    pub fn x(mut self, x: f32) -> Self {
        self.coord.x = x;
        self
    }

    pub fn y(mut self, y: f32) -> Self {
        self.coord.y = y;
        self
    }

    pub fn z(mut self, z: f32) -> Self {
        self.coord.z = z;
        self
    }

    pub fn speed(mut self, speed: Vec3A) -> Self {
        self.speed = speed;
        self
    }

    pub fn dx(mut self, dx: f32) -> Self {
        self.speed.x = dx;
        self
    }

    pub fn dy(mut self, dy: f32) -> Self {
        self.speed.y = dy;
        self
    }

    pub fn dz(mut self, dz: f32) -> Self {
        self.speed.z = dz;
        self
    }

    pub fn build(self) -> Star {
        Star {
            coord: self.coord,
            speed: self.speed,
        }
    }
}
