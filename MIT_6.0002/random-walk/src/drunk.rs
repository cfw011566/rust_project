use crate::prelude::*;
use rand::Rng;
use std::fmt;

#[derive(Debug, Clone)]
pub struct Drunk {
    name: String,
    step_choice: Vec<Location>,
}

impl fmt::Display for Drunk {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "name='{}', steps={:?}", self.name, self.step_choice)
    }
}

impl Drunk {
    pub fn new(name: String, steps: &[Location]) -> Self {
        Self {
            name,
            step_choice: steps.to_owned(),
        }
    }

    pub fn take_step(&self) -> (f64, f64) {
        let len = self.step_choice.len();
        let n = rand::thread_rng().gen_range(0..len);
        let step = &self.step_choice[n];
        (step.x(), step.y())
    }

    pub fn name(&self) -> &str {
        &self.name
    }
}
