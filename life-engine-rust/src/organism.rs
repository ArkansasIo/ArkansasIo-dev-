use crate::brain::Brain;
use rand::Rng;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CellType { Mouth, Producer, Mover, Killer, Armor, Eye }

#[derive(Clone, Copy, Debug)]
pub struct Cell { pub x: i32, pub y: i32, pub kind: CellType }

#[derive(Clone, Debug)]
pub struct Organism {
    pub id: u64,
    pub x: i32,
    pub y: i32,
    pub cells: Vec<Cell>,
    pub energy: i32,
    pub age: u32,
    pub lifespan: u32,
    pub brain: Brain,
}

impl Organism {
    pub fn random(id: u64, x: i32, y: i32, rng: &mut impl Rng) -> Self {
        let kinds = [CellType::Mouth, CellType::Mover, CellType::Producer, CellType::Eye];
        let n = rng.random_range(2..=5);
        let mut cells = Vec::with_capacity(n);
        for i in 0..n {
            cells.push(Cell { x: (i as i32) % 2, y: (i as i32) / 2, kind: kinds[rng.random_range(0..kinds.len())] });
        }
        Self { id, x, y, cells, energy: 4, age: 0, lifespan: rng.random_range(50..=180), brain: Brain::default() }
    }

    pub fn has(&self, kind: CellType) -> bool { self.cells.iter().any(|c| c.kind == kind) }

    pub fn mutate(&mut self, rng: &mut impl Rng) {
        if self.cells.is_empty() { return; }
        let idx = rng.random_range(0..self.cells.len());
        let kinds = [CellType::Mouth, CellType::Producer, CellType::Mover, CellType::Killer, CellType::Armor, CellType::Eye];
        self.cells[idx].kind = kinds[rng.random_range(0..kinds.len())];
        if rng.random_bool(0.25) && self.cells.len() < 12 {
            let base = self.cells[rng.random_range(0..self.cells.len())];
            self.cells.push(Cell { x: base.x + rng.random_range(-1..=1), y: base.y + rng.random_range(-1..=1), kind: kinds[rng.random_range(0..kinds.len())] });
        } else if rng.random_bool(0.15) && self.cells.len() > 1 {
            self.cells.remove(idx);
        }
    }
}
