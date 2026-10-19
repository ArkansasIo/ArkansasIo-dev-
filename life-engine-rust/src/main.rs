use std::{env, fs, io::{self, BufRead, Write}, path::Path};

use life_engine_rust::{stats::{csv_document, json_document, SimulationStats}, LifeEngine, Replay};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.iter().any(|arg| arg == "--help" || arg == "-h") {
        print_help();
        return;
    }

    let seed = value_after(&args, "--seed").and_then(|value| value.parse().ok()).unwrap_or(0x4C49_4645);
    let population = value_after(&args, "--population").and_then(|value| value.parse().ok()).unwrap_or(100);
    let ticks = value_after(&args, "--ticks").and_then(|value| value.parse().ok()).unwrap_or(1_000);

    let mut engine = LifeEngine::new(seed, population);
    let mut history = vec![SimulationStats::from_engine(&engine)];

    if args.iter().any(|arg| arg == "--interactive") {
        interactive(&mut engine, &mut history, seed, population);
    } else {
        run(&mut engine, &mut history, ticks);
        println!("LIFE ENGINE | tick={} population={} births={} deaths={}", engine.tick, engine.population(), engine.births, engine.deaths);
        println!("mean_energy={:.2} mean_age={:.2}", history.last().map(|s| s.mean_energy).unwrap_or(0.0), history.last().map(|s| s.mean_age).unwrap_or(0.0));
    }
}

fn value_after(args: &[String], flag: &str) -> Option<String> {
    args.windows(2).find(|window| window[0] == flag).map(|window| window[1].clone())
}

fn run(engine: &mut LifeEngine, history: &mut Vec<SimulationStats>, ticks: u64) {
    for _ in 0..ticks {
        if engine.population() == 0 { break; }
        engine.step();
        history.push(SimulationStats::from_engine(engine));
    }
}

fn interactive(engine: &mut LifeEngine, history: &mut Vec<SimulationStats>, seed: u64, population: usize) {
    println!("Life Engine interactive console. Type 'help' for commands.");
    let stdin = io::stdin();
    loop {
        print!("life> ");
        let _ = io::stdout().flush();
        let mut line = String::new();
        if stdin.lock().read_line(&mut line).is_err() { break; }
        let mut parts = line.split_whitespace();
        let command = parts.next().unwrap_or("");
        match command {
            "help" => println!("step [n] | run [n] | stats | save-csv <file> | save-json <file> | replay <file> | reset | quit"),
            "step" => {
                let count = parts.next().and_then(|value| value.parse().ok()).unwrap_or(1);
                run(engine, history, count);
                print_status(engine);
            }
            "run" => {
                let count = parts.next().and_then(|value| value.parse().ok()).unwrap_or(100);
                run(engine, history, count);
                print_status(engine);
            }
            "stats" => print_status(engine),
            "save-csv" => save(parts.next(), &csv_document(history), "CSV"),
            "save-json" => save(parts.next(), &json_document(history), "JSON"),
            "replay" => {
                let file = parts.next().unwrap_or("life.replay");
                let replay = Replay::new(seed, engine.tick, population);
                save(Some(file), &replay.encode(), "replay");
            }
            "reset" => {
                engine.reset(seed, population);
                history.clear();
                history.push(SimulationStats::from_engine(engine));
                print_status(engine);
            }
            "quit" | "exit" => break,
            "" => {}
            _ => println!("Unknown command. Type 'help'."),
        }
    }
}

fn print_status(engine: &LifeEngine) {
    let stats = SimulationStats::from_engine(engine);
    println!("tick={} population={} births={} deaths={} mean_energy={:.2} mean_age={:.2}", stats.tick, stats.population, stats.births, stats.deaths, stats.mean_energy, stats.mean_age);
}

fn save(path: Option<&str>, contents: &str, kind: &str) {
    let path = path.unwrap_or("life-engine-output.txt");
    if let Err(error) = fs::write(Path::new(path), contents) {
        eprintln!("failed to write {kind}: {error}");
    } else {
        println!("wrote {kind} to {path}");
    }
}

fn print_help() {
    println!("Life Engine Rust 0.2\n");
    println!("Usage: life-engine [--seed N] [--population N] [--ticks N] [--interactive]");
    println!("\nInteractive commands: step, run, stats, save-csv, save-json, replay, reset, quit");
}
