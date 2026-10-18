mod brain;
mod config;
mod ecology;
mod evolution;
mod genome;
mod neural;
mod organism;
mod physics;
mod unified;
mod world;

use rand::SeedableRng;
use unified::LifeEngine;

fn main() {
    let seed = 0x4C494645u64;
    let mut engine = LifeEngine::new(seed, 100);
    for _ in 0..10_000 {
        engine.step(seed);
        if engine.tick % 10 == 0 {
            print!("\x1B[2J\x1B[H");
            println!("LIFE ENGINE UNIFIED | tick={} population={}", engine.tick, engine.population());
        }
        if engine.population() == 0 { break; }
    }
}
