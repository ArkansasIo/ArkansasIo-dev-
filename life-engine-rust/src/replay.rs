use crate::config::Config;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Replay {
    pub seed: u64,
    pub ticks: u64,
    pub population: usize,
}

impl Replay {
    pub fn new(seed: u64, ticks: u64, population: usize) -> Self {
        Self { seed, ticks, population }
    }

    pub fn encode(&self) -> String {
        format!(
            "LIFE_REPLAY_V1\nseed={}\nticks={}\npopulation={}\n",
            self.seed, self.ticks, self.population
        )
    }

    pub fn decode(input: &str) -> Result<Self, String> {
        let mut header_seen = false;
        let mut seed = None;
        let mut ticks = None;
        let mut population = None;

        for line in input.lines() {
            let line = line.trim();
            if line.is_empty() {
                continue;
            }
            if line == "LIFE_REPLAY_V1" {
                if header_seen {
                    return Err("invalid replay: duplicate header".into());
                }
                header_seen = true;
                continue;
            }

            let Some((key, value)) = line.split_once('=') else {
                return Err(format!("invalid replay line: {line}"));
            };
            let value = value.trim();
            match key.trim() {
                "seed" => set_once(&mut seed, value, "seed")?,
                "ticks" => set_once(&mut ticks, value, "ticks")?,
                "population" => set_once(&mut population, value, "population")?,
                unknown => return Err(format!("invalid replay field: {unknown}")),
            }
        }

        if !header_seen {
            return Err("invalid replay: missing LIFE_REPLAY_V1 header".into());
        }

        match (seed, ticks, population) {
            (Some(seed), Some(ticks), Some(population)) => Ok(Self { seed, ticks, population }),
            _ => Err("invalid replay: expected seed, ticks, and population".into()),
        }
    }

    pub fn config(&self) -> Config {
        Config {
            seed: self.seed,
            max_ticks: self.ticks,
            initial_population: self.population,
            ..Config::default()
        }
    }
}

fn set_once<T: std::str::FromStr>(
    slot: &mut Option<T>,
    value: &str,
    key: &str,
) -> Result<(), String> {
    if slot.is_some() {
        return Err(format!("invalid replay: duplicate {key} field"));
    }
    *slot = Some(
        value
            .parse()
            .map_err(|_| format!("invalid replay value for {key}"))?,
    );
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn replay_round_trips() {
        let replay = Replay::new(7, 100, 12);
        assert_eq!(Replay::decode(&replay.encode()).unwrap(), replay);
    }

    #[test]
    fn replay_requires_version_header() {
        assert!(Replay::decode("seed=1\nticks=2\npopulation=3\n").is_err());
    }

    #[test]
    fn replay_rejects_duplicate_fields_and_unknown_keys() {
        assert!(Replay::decode(
            "LIFE_REPLAY_V1\nseed=1\nseed=2\nticks=3\npopulation=4\n"
        )
        .is_err());
        assert!(Replay::decode(
            "LIFE_REPLAY_V1\nseed=1\nticks=2\npopulation=3\nextra=4\n"
        )
        .is_err());
    }
}
