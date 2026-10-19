use std::fmt::Write;

use crate::unified::LifeEngine;

#[derive(Clone, Debug, Default, PartialEq)]
pub struct SimulationStats {
    pub tick: u64,
    pub population: usize,
    pub total_energy: f32,
    pub mean_energy: f32,
    pub mean_age: f32,
    pub births: u64,
    pub deaths: u64,
    pub generations: u64,
}

impl SimulationStats {
    pub fn from_engine(engine: &LifeEngine) -> Self {
        let population = engine.agents.len();
        let total_energy = engine.agents.iter().map(|agent| agent.energy).sum::<f32>();
        let total_age = engine.agents.iter().map(|agent| agent.age as f32).sum::<f32>();
        let generations = engine.agents.iter().map(|agent| agent.generation as u64).max().unwrap_or(0);
        Self {
            tick: engine.tick,
            population,
            total_energy,
            mean_energy: if population == 0 { 0.0 } else { total_energy / population as f32 },
            mean_age: if population == 0 { 0.0 } else { total_age / population as f32 },
            births: engine.births,
            deaths: engine.deaths,
            generations,
        }
    }

    pub fn csv_header() -> &'static str {
        "tick,population,total_energy,mean_energy,mean_age,births,deaths,generations"
    }

    pub fn csv_row(&self) -> String {
        format!("{},{},{:.4},{:.4},{:.4},{},{},{}", self.tick, self.population, self.total_energy, self.mean_energy, self.mean_age, self.births, self.deaths, self.generations)
    }

    pub fn json(&self) -> String {
        format!("{{\"tick\":{},\"population\":{},\"total_energy\":{:.4},\"mean_energy\":{:.4},\"mean_age\":{:.4},\"births\":{},\"deaths\":{},\"generations\":{}}}", self.tick, self.population, self.total_energy, self.mean_energy, self.mean_age, self.births, self.deaths, self.generations)
    }
}

pub fn csv_document(history: &[SimulationStats]) -> String {
    let mut output = String::from(SimulationStats::csv_header());
    output.push('\n');
    for stat in history {
        let _ = writeln!(output, "{}", stat.csv_row());
    }
    output
}

pub fn json_document(history: &[SimulationStats]) -> String {
    let mut output = String::from("[\n");
    for (index, stat) in history.iter().enumerate() {
        let _ = write!(output, "  {}{}\n", stat.json(), if index + 1 == history.len() { "" } else { "," });
    }
    output.push(']');
    output
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_engine_stats_are_safe() {
        let engine = LifeEngine::new(1, 0);
        let stats = SimulationStats::from_engine(&engine);
        assert_eq!(stats.population, 0);
        assert_eq!(stats.mean_energy, 0.0);
    }
}
