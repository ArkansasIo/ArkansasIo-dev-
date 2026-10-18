use rand::Rng;
use crate::genome::Genome;

#[derive(Clone, Debug)]
pub struct Lineage { pub id: u64, pub parent: Option<u64>, pub generation: u64, pub births: u64, pub deaths: u64 }

pub fn crossover(a: &Genome, b: &Genome, rng: &mut impl Rng) -> Genome {
    let mut genes = Vec::new();
    let n = a.genes.len().max(b.genes.len());
    for i in 0..n { if rng.random_bool(0.5) { if let Some(g)=a.genes.get(i){genes.push(g.clone());} } else if let Some(g)=b.genes.get(i){genes.push(g.clone());} }
    Genome { genes, mutation_rate: (a.mutation_rate+b.mutation_rate)/2.0, move_range: ((a.move_range+b.move_range)/2).max(1), lifespan: ((a.lifespan+b.lifespan)/2).max(1) }
}

pub fn natural_selection(score: f32, population_pressure: f32) -> bool { score > population_pressure }
pub fn mutation_event(genome: &mut Genome, rng: &mut impl Rng) { genome.mutate(rng); }
