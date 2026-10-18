#[derive(Clone, Debug)]
pub struct NeuralNetwork { pub input: usize, pub hidden: usize, pub output: usize, pub weights: Vec<f32> }
impl NeuralNetwork {
    pub fn new(input: usize, hidden: usize, output: usize) -> Self { Self { input, hidden, output, weights: vec![0.0; (input*hidden)+(hidden*output)+hidden+output] } }
    pub fn evaluate(&self, inputs: &[f32]) -> Vec<f32> {
        let mut h = vec![0.0; self.hidden]; let mut p = 0;
        for j in 0..self.hidden { for i in 0..self.input { h[j] += inputs.get(i).copied().unwrap_or(0.0) * self.weights[p]; p += 1; } h[j] = h[j].tanh(); p += 1; }
        let mut out = vec![0.0; self.output]; for j in 0..self.output { for i in 0..self.hidden { out[j] += h[i] * self.weights[p]; p += 1; } out[j] = (out[j] + self.weights[p]).tanh(); p += 1; } out
    }
}

#[derive(Clone, Copy, Debug)]
pub enum BrainAction { Ignore, SeekFood, FleePredator, SeekMate, Attack, Explore }

pub fn decide(vision: &[f32]) -> BrainAction {
    let food = vision.get(0).copied().unwrap_or(0.0); let threat = vision.get(1).copied().unwrap_or(0.0); let mate = vision.get(2).copied().unwrap_or(0.0);
    if threat > 0.7 { BrainAction::FleePredator } else if food > 0.5 { BrainAction::SeekFood } else if mate > 0.6 { BrainAction::SeekMate } else { BrainAction::Explore }
}
