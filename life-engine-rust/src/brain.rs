#[derive(Clone, Copy, Debug)]
pub enum Action { Ignore, ChaseFood, FleePredator }

#[derive(Clone, Copy, Debug)]
pub struct Brain {
    pub food_response: Action,
    pub predator_response: Action,
}

impl Default for Brain {
    fn default() -> Self {
        Self { food_response: Action::ChaseFood, predator_response: Action::FleePredator }
    }
}
