use rand::{Rng, SeedableRng};
use rand_chacha::ChaCha8Rng;

use crate::{
    ecology::Ecosystem,
    evolution::crossover,
    genome::Genome,
    neural::{decide, BrainAction, NeuralNetwork},
    physics::{integrate, Particle, Vec2},
};

#[derive(Clone, Debug)]
pub struct Agent {
    pub id: u64,
    pub parent: Option<u64>,
    pub generation: u32,
    pub genome: Genome,
    pub brain_inputs: Vec<f32>,
    pub brain: NeuralNetwork,
    pub energy: f32,
    pub age: u32,
    pub alive: bool,
    pub particle: Particle,
}

impl Agent {
    pub fn random(id: u64, rng: &mut impl Rng) -> Self {
        Self {
            id,
            parent: None,
            generation: 0,
            genome: Genome::random(rng),
            brain_inputs: vec![0.0; 4],
            brain: NeuralNetwork::random(4, 6, 6, rng),
            energy: 50.0,
            age: 0,
            alive: true,
            particle: Particle {
                position: Vec2 { x: rng.random_range(-50.0..50.0), y: rng.random_range(-50.0..50.0) },
                velocity: Vec2::default(),
                mass: 1.0,
                health: 10.0,
            },
        }
    }

    pub fn tick(&mut self, eco: &Ecosystem, rng: &mut impl Rng) {
        if !self.alive { return; }
        self.age += 1;
        self.energy -= 0.2;
        self.brain_inputs[0] = (eco.resources.energy / 1000.0).clamp(0.0, 1.0);
        self.brain_inputs[1] = eco.climate.toxicity.clamp(0.0, 1.0);
        self.brain_inputs[2] = (self.energy / 100.0).clamp(0.0, 1.0);
        self.brain_inputs[3] = (self.age as f32 / self.genome.lifespan.max(1) as f32).clamp(0.0, 1.0);

        let neural = self.brain.evaluate(&self.brain_inputs);
        let action = decide(&[self.brain_inputs[0], self.brain_inputs[1], neural.get(2).copied().unwrap_or(0.0)]);
        let direction = match action {
            BrainAction::SeekFood => 1.0,
            BrainAction::FleePredator => -1.0,
            BrainAction::Explore => 0.5,
            BrainAction::SeekMate | BrainAction::Attack => 0.75,
            BrainAction::Ignore => 0.0,
        };

        self.particle.velocity.x += rng.random_range(-1.0..1.0) * direction * self.genome.move_range as f32 * 0.1;
        self.particle.velocity.y += rng.random_range(-1.0..1.0) * direction * self.genome.move_range as f32 * 0.1;
        integrate(&mut self.particle, 1.0, Vec2 { x: 0.0, y: -0.01 });

        if self.age > self.genome.lifespan || self.energy <= 0.0 || self.particle.health <= 0.0 {
            self.alive = false;
        }
    }
}

pub struct LifeEngine {
    pub tick: u64,
    pub agents: Vec<Agent>,
    pub ecosystem: Ecosystem,
    pub max_population: usize,
    pub births: u64,
    pub deaths: u64,
    rng: ChaCha8Rng,
    next_id: u64,
}

impl LifeEngine {
    pub fn new(seed: u64, population: usize) -> Self {
        let mut rng = ChaCha8Rng::seed_from_u64(seed);
        let agents = (0..population).map(|i| Agent::random(i as u64 + 1, &mut rng)).collect();
        Self {
            tick: 0,
            agents,
            ecosystem: Ecosystem::default(),
            max_population: population.saturating_mul(4).max(population),
            births: 0,
            deaths: 0,
            rng,
            next_id: population as u64 + 1,
        }
    }

    pub fn step(&mut self) {
        self.tick += 1;
        self.ecosystem.tick();
        let before = self.agents.len();
        for agent in &mut self.agents {
            agent.tick(&self.ecosystem, &mut self.rng);
            if agent.alive {
                let uptake = agent.energy.min(0.5).max(0.0);
                self.ecosystem.resources.energy = (self.ecosystem.resources.energy - uptake).max(0.0);
            }
        }
        self.reproduce();
        self.agents.retain(|agent| agent.alive);
        self.deaths += before.saturating_sub(self.agents.len()) as u64;
    }

    fn reproduce(&mut self) {
        if self.agents.len() >= self.max_population || self.agents.len() < 2 { return; }
        let eligible: Vec<usize> = self.agents.iter().enumerate()
            .filter(|(_, agent)| agent.alive && agent.energy > 30.0)
            .map(|(index, _)| index)
            .collect();
        if eligible.len() < 2 { return; }

        for pair in eligible.chunks(2) {
            if pair.len() < 2 || self.agents.len() >= self.max_population { break; }
            if !self.rng.random_bool(0.08) { continue; }
            let first = self.agents[pair[0]].clone();
            let second = self.agents[pair[1]].clone();
            let mut child = first.clone();
            child.id = self.next_id;
            self.next_id += 1;
            child.parent = Some(first.id);
            child.generation = first.generation.max(second.generation) + 1;
            child.genome = crossover(&first.genome, &second.genome, &mut self.rng);
            child.genome.mutate(&mut self.rng);
            child.brain = NeuralNetwork::random(4, 6, 6, &mut self.rng);
            child.brain.mutate(child.genome.mutation_rate, 0.25, &mut self.rng);
            child.age = 0;
            child.energy = (first.energy + second.energy) * 0.25;
            child.particle.position.x += self.rng.random_range(-5.0..5.0);
            child.particle.position.y += self.rng.random_range(-5.0..5.0);
            self.agents[pair[0]].energy *= 0.5;
            self.agents[pair[1]].energy *= 0.5;
            self.agents.push(child);
            self.births += 1;
        }
    }

    pub fn population(&self) -> usize { self.agents.len() }

    pub fn reset(&mut self, seed: u64, population: usize) {
        *self = Self::new(seed, population);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn same_seed_produces_same_initial_state() {
        let a = LifeEngine::new(42, 10);
        let b = LifeEngine::new(42, 10);
        assert_eq!(a.agents[0].genome.genes.len(), b.agents[0].genome.genes.len());
        assert_eq!(a.agents[0].particle.position.x, b.agents[0].particle.position.x);
    }

    #[test]
    fn step_advances_clock() {
        let mut engine = LifeEngine::new(42, 4);
        engine.step();
        assert_eq!(engine.tick, 1);
    }
}
