use life_engine_rust::{evolution::crossover, genome::Genome, neural::{decide, BrainAction}, replay::Replay, stats::SimulationStats, LifeEngine};
use rand::SeedableRng;
use rand_chacha::ChaCha8Rng;

#[test]
fn seeded_runs_start_identically() {
    let a = LifeEngine::new(1234, 20);
    let b = LifeEngine::new(1234, 20);
    assert_eq!(a.population(), b.population());
    assert_eq!(a.agents[0].particle.position.x, b.agents[0].particle.position.x);
    assert_eq!(a.agents[0].genome.lifespan, b.agents[0].genome.lifespan);
}

#[test]
fn simulation_progresses_and_records_stats() {
    let mut engine = LifeEngine::new(1234, 20);
    engine.step();
    engine.step();
    let stats = SimulationStats::from_engine(&engine);
    assert_eq!(stats.tick, 2);
    assert!(stats.total_energy.is_finite());
}

#[test]
fn replay_round_trip_is_stable() {
    let replay = Replay::new(99, 250, 32);
    let encoded = replay.encode();
    assert_eq!(Replay::decode(&encoded).unwrap(), replay);
}

#[test]
fn crossover_preserves_valid_genome_shape() {
    let mut rng = ChaCha8Rng::seed_from_u64(5);
    let a = Genome::random(&mut rng);
    let b = Genome::random(&mut rng);
    let child = crossover(&a, &b, &mut rng);
    assert!(!child.genes.is_empty());
    assert!(child.lifespan > 0);
    assert!(child.move_range > 0);
}

#[test]
fn brain_priority_is_deterministic() {
    assert_eq!(decide(&[0.8, 0.1, 0.0]), BrainAction::SeekFood);
    assert_eq!(decide(&[0.8, 0.9, 0.0]), BrainAction::FleePredator);
}

#[test]
fn mutation_stays_within_safe_bounds() {
    let mut rng = ChaCha8Rng::seed_from_u64(77);
    let mut genome = Genome::random(&mut rng);
    for _ in 0..1_000 { genome.mutate(&mut rng); }
    assert!((1..=12).contains(&genome.move_range));
    assert!(genome.lifespan > 0);
}
