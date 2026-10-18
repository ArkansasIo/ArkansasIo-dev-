use crate::{config::Config, organism::{CellType, Organism}};
use rand::Rng;

#[derive(Clone, Copy, PartialEq, Eq)]
enum Tile { Empty, Food }

pub struct World {
    pub config: Config,
    tiles: Vec<Tile>,
    organisms: Vec<Organism>,
    next_id: u64,
}

impl World {
    pub fn new(config: Config, rng: &mut impl Rng) -> Self {
        let mut world = Self { tiles: vec![Tile::Empty; (config.width * config.height) as usize], organisms: Vec::new(), next_id: 1, config };
        for _ in 0..world.config.initial_population { world.spawn_random(rng); }
        world
    }
    fn index(&self, x: i32, y: i32) -> Option<usize> { Self::index_for(self.config.width, self.config.height, x, y) }
    fn index_for(width: i32, height: i32, x: i32, y: i32) -> Option<usize> { if x < 0 || y < 0 || x >= width || y >= height { None } else { Some((y * width + x) as usize) } }
    fn spawn_random(&mut self, rng: &mut impl Rng) { let x = rng.random_range(0..self.config.width); let y = rng.random_range(0..self.config.height); let id = self.next_id; self.next_id += 1; self.organisms.push(Organism::random(id, x, y, rng)); }
    pub fn population(&self) -> usize { self.organisms.len() }

    pub fn step(&mut self, rng: &mut impl Rng) {
        for i in 0..self.tiles.len() { if self.tiles[i] == Tile::Empty && rng.random_bool(self.config.food_spawn_chance) { self.tiles[i] = Tile::Food; } }
        let width = self.config.width; let height = self.config.height;
        for organism in &mut self.organisms {
            organism.age += 1; organism.energy -= 1;
            if organism.has(CellType::Producer) && rng.random_bool(0.20) { let x = organism.x + rng.random_range(-1..=1); let y = organism.y + rng.random_range(-1..=1); if let Some(idx) = Self::index_for(width, height, x, y) { self.tiles[idx] = Tile::Food; } }
            if organism.has(CellType::Mouth) { if let Some(idx) = Self::index_for(width, height, organism.x, organism.y) { if self.tiles[idx] == Tile::Food { self.tiles[idx] = Tile::Empty; organism.energy += 5; } } }
        }
        for organism in &mut self.organisms { if organism.has(CellType::Mover) { organism.x = (organism.x + rng.random_range(-1..=1)).clamp(0, width - 1); organism.y = (organism.y + rng.random_range(-1..=1)).clamp(0, height - 1); } }
        let mut newborns = Vec::new();
        if self.organisms.len() < self.config.max_population {
            for parent in &mut self.organisms {
                if parent.energy >= parent.cells.len() as i32 * 5 && rng.random_bool(0.04) {
                    parent.energy /= 2; let mut child = parent.clone(); child.id = self.next_id; self.next_id += 1;
                    child.x = (child.x + rng.random_range(-4..=4)).clamp(0, width - 1); child.y = (child.y + rng.random_range(-2..=2)).clamp(0, height - 1); child.age = 0; child.lifespan = rng.random_range(50..=180);
                    if rng.random_bool(self.config.mutation_chance) { child.mutate(rng); }
                    newborns.push(child); if self.organisms.len() + newborns.len() >= self.config.max_population { break; }
                }
            }
        }
        self.organisms.extend(newborns); self.organisms.retain(|o| o.age < o.lifespan && o.energy > -10);
    }

    pub fn render(&self) {
        let mut grid = vec![b' '; (self.config.width * self.config.height) as usize];
        for y in 0..self.config.height { for x in 0..self.config.width { if self.tiles[(y*self.config.width+x) as usize] == Tile::Food { grid[(y*self.config.width+x) as usize] = b'.'; } } }
        for o in &self.organisms { if let Some(i) = self.index(o.x, o.y) { grid[i] = if o.has(CellType::Killer) { b'K' } else if o.has(CellType::Eye) && o.has(CellType::Mover) { b'B' } else { b'o' }; } }
        for y in 0..self.config.height { println!("{}", String::from_utf8_lossy(&grid[(y*self.config.width) as usize..((y+1)*self.config.width) as usize])); }
        println!("o=organism B=brain K=predator .=food");
    }
}
