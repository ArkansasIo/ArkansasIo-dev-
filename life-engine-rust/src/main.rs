mod brain;
mod config;
mod organism;
mod world;

use config::Config;
use rand::SeedableRng;
use rand_chacha::ChaCha8Rng;
use world::World;

fn main() {
    let config = Config::default();
    let mut rng = ChaCha8Rng::seed_from_u64(config.seed);
    let mut world = World::new(config, &mut rng);

    for tick in 0..world.config.max_ticks {
        world.step(&mut rng);
        if tick % world.config.render_every == 0 {
            print!("\x1B[2J\x1B[H");
            println!("LIFE ENGINE RUST | tick={} population={} seed={}", tick, world.population(), world.config.seed);
            world.render();
        }
        if world.population() == 0 { break; }
    }
}
