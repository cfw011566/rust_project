#[derive(Debug)]
pub struct Food {
    name: String,
    value: f64,
    calories: f64,
}

impl Food {
    pub fn new(name: &str, value: f64, calories: f64) -> Self {
        Self {
            name: name.to_string(),
            value,
            calories,
        }
    }

    pub fn build_menu(names: &[&str], values: &[f64], calories: &[f64]) -> Vec<Food> {
        let mut foods: Vec<Food> = Vec::new();
        for (i, &val) in values.iter().enumerate() {
            foods.push(Food::new(names[i], val, calories[i]));
        }
        foods
    }
}

use std::fmt;
impl fmt::Display for Food {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}: <{}, {}>", self.name, self.value, self.calories)
    }
}

impl Food {
    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn value(&self) -> f64 {
        self.value
    }

    pub fn calories(&self) -> f64 {
        self.calories
    }

    pub fn density(&self) -> f64 {
        self.value / self.calories
    }
}
