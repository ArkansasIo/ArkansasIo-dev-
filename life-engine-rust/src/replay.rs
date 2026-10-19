use crate::config::Config;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Replay {
    pub seed: u64,
    pub ticks: u64,
    pub population: usize,
}

impl Replay {
    pub fn new(seed: u64, ticks: u64, population: usize) -> Self { Self { seed, ticks, population } }

    pub fn encode(&self) -> String {
        format!("LIFE_REPLAY_V1\nseed={}\nticks={}\npopulation={}\n", self.seed, self.ticks, self.population)
    }

    pub fn decode(input: &str) -> Result<Self, String> {
        let mut seed = None;
        let mut ticks = None;
        let mut population = None;
        for line in input.lines() {
            if line == "LIFE_REPLAY_V1" { continue; }
            let Some((key, value)) = line.split_once('=') else { continue; };
            match key {
                "seed" => seed = value.parse().ok(),
                "ticks" => ticks = value.parse().ok(),
                "population" => population = value.parse().ok(),
                _ => {}
            }
        }
        match (seed, ticks, population) {
            (Some(seed), Some(ticks), Some(population)) => Ok(Self { seed, ticks, population }),
            _ => Err("invalid replay: expected seed, ticks, and population".into()),
        }
    }

    pub fn config(&self) -> Config {
        Config { seed: self.seed, max_ticks: self.ticks, initial_population: self.population, ..Config::default() }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn replay_round_trips() {
        let replay = Replay::new(7, 100, 12);
        assert_eq!(Replay::decode(&replay.encode()).unwrap(), replay);
    }
}
