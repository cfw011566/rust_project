use std::fmt;

#[derive(Debug, Clone)]
pub struct Location {
    x: f64,
    y: f64,
}

impl fmt::Display for Location {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "<{},{}>", self.x, self.y)
    }
}

impl Location {
    pub fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }

    pub fn move_by(&mut self, delta_x: f64, delta_y: f64) {
        self.x += delta_x;
        self.y += delta_y;
    }

    pub fn distance_from(&self, other: &Location) -> f64 {
        let delta_x = self.x - other.x;
        let delta_y = self.y - other.y;
        delta_x.hypot(delta_y)
    }

    pub fn x(&self) -> f64 {
        self.x
    }

    pub fn y(&self) -> f64 {
        self.y
    }
}
