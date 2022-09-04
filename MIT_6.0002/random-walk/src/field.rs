use crate::prelude::*;
use rand::Rng;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Field {
    drunks: HashMap<String, Location>,
    name: String,
    worm_holes: HashMap<String, Location>,
}

impl Field {
    pub fn new() -> Self {
        let drunks: HashMap<String, Location> = HashMap::new();
        let worm_holes: HashMap<String, Location> = HashMap::new();
        let name = "Normal".to_string();
        Self {
            drunks,
            name,
            worm_holes,
        }
    }

    pub fn add_drunk(&mut self, drunk: &Drunk, location: &Location) {
        let name = drunk.name().to_string();
        self.drunks.insert(name, location.clone());
    }

    pub fn get_location(&self, drunk: &Drunk) -> Location {
        if let Some(loc) = self.drunks.get(drunk.name()) {
            loc.clone()
        } else {
            Location::new(0.0, 0.0)
        }
    }

    pub fn move_drunk(&mut self, drunk: &Drunk) {
        let name = drunk.name().to_string();
        if let Some(loc) = self.drunks.get(&name) {
            let mut loc = loc.clone();
            let (x_dist, y_dist) = drunk.take_step();
            loc.move_by(x_dist, y_dist);
            let key = format!("{}", loc);
            if let Some(next_loc) = self.worm_holes.get(&key) {
                let loc = next_loc.clone();
                self.drunks.insert(name, loc);
            } else {
                self.drunks.insert(name, loc);
            }
        }
    }

    pub fn set_name(&mut self, name: String) {
        self.name = name;
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn set_worm_holes(&mut self, num_holes: u16, x_range: i16, y_range: i16) {
        for _ in 0..num_holes {
            let x = rand::thread_rng().gen_range(-x_range..x_range);
            let y = rand::thread_rng().gen_range(-y_range..y_range);
            let loc = Location::new(x as f64, y as f64);
            let key = format!("{}", loc);
            let new_x = rand::thread_rng().gen_range(-x_range..x_range);
            let new_y = rand::thread_rng().gen_range(-y_range..y_range);
            let new_loc = Location::new(new_x as f64, new_y as f64);
            self.worm_holes.insert(key, new_loc);
        }
    }
}
