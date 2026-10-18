use rand::Rng;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CellKind { Mouth, Producer, Mover, Killer, Armor, Eye, Sensor, Muscle, Constructor, Storage, Receptor }

#[derive(Clone, Debug)]
pub struct Gene { pub kind: CellKind, pub x: i16, pub y: i16, pub orientation: u8 }

#[derive(Clone, Debug)]
pub struct Genome { pub genes: Vec<Gene>, pub mutation_rate: f32, pub move_range: i16, pub lifespan: u32 }

impl Genome {
    pub fn random(rng: &mut impl Rng) -> Self {
        let mut genes = Vec::new();
        for i in 0..rng.random_range(2..=8) {
            genes.push(Gene { kind: random_kind(rng), x: i % 3 - 1, y: i / 3, orientation: rng.random_range(0..4) });
        }
        Self { genes, mutation_rate: 0.05, move_range: 2, lifespan: rng.random_range(100..=400) }
    }
    pub fn mutate(&mut self, rng: &mut impl Rng) {
        if rng.random_bool(self.mutation_rate as f64) && !self.genes.is_empty() { self.genes[rng.random_range(0..self.genes.len())].kind = random_kind(rng); }
        if rng.random_bool(self.mutation_rate as f64) { self.genes.push(Gene { kind: random_kind(rng), x: rng.random_range(-3..=3), y: rng.random_range(-3..=3), orientation: rng.random_range(0..4) }); }
        if self.genes.len() > 1 && rng.random_bool(self.mutation_rate as f64) { self.genes.remove(rng.random_range(0..self.genes.len())); }
        if rng.random_bool(self.mutation_rate as f64) { self.move_range = (self.move_range + rng.random_range(-1..=1)).clamp(1, 12); }
    }
}
fn random_kind(rng: &mut impl Rng) -> CellKind { match rng.random_range(0..11) { 0=>CellKind::Mouth,1=>CellKind::Producer,2=>CellKind::Mover,3=>CellKind::Killer,4=>CellKind::Armor,5=>CellKind::Eye,6=>CellKind::Sensor,7=>CellKind::Muscle,8=>CellKind::Constructor,9=>CellKind::Storage,_=>CellKind::Receptor } }
