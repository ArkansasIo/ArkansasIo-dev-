#[derive(Clone)]
pub struct Config {
    pub width: i32,
    pub height: i32,
    pub initial_population: usize,
    pub max_population: usize,
    pub max_ticks: u64,
    pub render_every: u64,
    pub food_spawn_chance: f64,
    pub mutation_chance: f64,
    pub seed: u64,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            width: 80,
            height: 30,
            initial_population: 24,
            max_population: 500,
            max_ticks: 10_000,
            render_every: 5,
            food_spawn_chance: 0.08,
            mutation_chance: 0.06,
            seed: 0x4C_49_46_45,
        }
    }
}
