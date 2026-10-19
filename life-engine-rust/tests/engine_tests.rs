use life_engine_rust::{
    evolution::crossover,
    genome::Genome,
    neural::{decide, BrainAction},
    replay::Replay,
    stats::{csv_document, json_document, SimulationStats},
    LifeEngine,
};
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
    for _ in 0..1_000 {
        genome.mutate(&mut rng);
    }
    assert!((1..=12).contains(&genome.move_range));
    assert!(genome.lifespan > 0);
}

#[test]
fn zero_population_is_supported() {
    let mut engine = LifeEngine::new(1234, 0);
    assert_eq!(engine.population(), 0);
    engine.step();
    assert_eq!(engine.tick, 1);
    assert_eq!(engine.population(), 0);
    let stats = SimulationStats::from_engine(&engine);
    assert_eq!(stats.mean_energy, 0.0);
    assert_eq!(stats.mean_age, 0.0);
}

#[test]
fn population_never_exceeds_configured_capacity() {
    let mut engine = LifeEngine::new(42, 40);
    for _ in 0..500 {
        engine.step();
        assert!(engine.population() <= engine.max_population);
    }
}

#[test]
fn reset_replays_the_same_initial_state() {
    let mut engine = LifeEngine::new(9876, 16);
    let expected_position = engine.agents[0].particle.position;
    let expected_lifespan = engine.agents[0].genome.lifespan;
    for _ in 0..10 {
        engine.step();
    }
    engine.reset(9876, 16);
    assert_eq!(engine.tick, 0);
    assert_eq!(engine.population(), 16);
    assert_eq!(engine.agents[0].particle.position.x, expected_position.x);
    assert_eq!(engine.agents[0].particle.position.y, expected_position.y);
    assert_eq!(engine.agents[0].genome.lifespan, expected_lifespan);
}

#[test]
fn exported_documents_include_header_and_valid_outer_json_shape() {
    let engine = LifeEngine::new(7, 3);
    let stat = SimulationStats::from_engine(&engine);
    let csv = csv_document(std::slice::from_ref(&stat));
    let json = json_document(std::slice::from_ref(&stat));
    assert!(csv.starts_with("tick,population,total_energy"));
    assert!(json.starts_with("[\n  {"));
    assert!(json.ends_with("\n]"));
}
