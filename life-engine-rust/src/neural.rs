use rand::Rng;

#[derive(Clone, Debug)]
pub struct NeuralNetwork {
    pub input: usize,
    pub hidden: usize,
    pub output: usize,
    pub weights: Vec<f32>,
}

impl NeuralNetwork {
    pub fn new(input: usize, hidden: usize, output: usize) -> Self {
        Self { input, hidden, output, weights: vec![0.0; (input * hidden) + hidden + (hidden * output) + output] }
    }

    pub fn random(input: usize, hidden: usize, output: usize, rng: &mut impl Rng) -> Self {
        let mut network = Self::new(input, hidden, output);
        for weight in &mut network.weights {
            *weight = rng.random_range(-1.0..=1.0);
        }
        network
    }

    pub fn parameter_count(&self) -> usize { self.weights.len() }

    pub fn evaluate(&self, inputs: &[f32]) -> Vec<f32> {
        let mut hidden_values = vec![0.0; self.hidden];
        let mut cursor = 0;
        for hidden in &mut hidden_values {
            for input in 0..self.input {
                *hidden += inputs.get(input).copied().unwrap_or(0.0) * self.weights[cursor];
                cursor += 1;
            }
            *hidden = (*hidden + self.weights[cursor]).tanh();
            cursor += 1;
        }

        let mut output_values = vec![0.0; self.output];
        for output in &mut output_values {
            for hidden in 0..self.hidden {
                *output += hidden_values[hidden] * self.weights[cursor];
                cursor += 1;
            }
            *output = (*output + self.weights[cursor]).tanh();
            cursor += 1;
        }
        output_values
    }

    pub fn mutate(&mut self, rate: f32, magnitude: f32, rng: &mut impl Rng) {
        for weight in &mut self.weights {
            if rng.random_bool(rate.clamp(0.0, 1.0) as f64) {
                *weight += rng.random_range(-magnitude..=magnitude);
                *weight = weight.clamp(-4.0, 4.0);
            }
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BrainAction { Ignore, SeekFood, FleePredator, SeekMate, Attack, Explore }

pub fn decide(vision: &[f32]) -> BrainAction {
    let food = vision.first().copied().unwrap_or(0.0);
    let threat = vision.get(1).copied().unwrap_or(0.0);
    let mate = vision.get(2).copied().unwrap_or(0.0);
    if threat > 0.7 { BrainAction::FleePredator }
    else if food > 0.5 { BrainAction::SeekFood }
    else if mate > 0.6 { BrainAction::SeekMate }
    else { BrainAction::Explore }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn network_has_expected_parameter_count() {
        let network = NeuralNetwork::new(3, 4, 2);
        assert_eq!(network.parameter_count(), 26);
    }

    #[test]
    fn network_evaluates_with_short_input() {
        let network = NeuralNetwork::new(3, 2, 2);
        let output = network.evaluate(&[1.0]);
        assert_eq!(output.len(), 2);
        assert!(output.iter().all(|value| value.is_finite()));
    }

    #[test]
    fn decision_layer_prioritizes_threat() {
        assert_eq!(decide(&[1.0, 0.9, 1.0]), BrainAction::FleePredator);
    }
}
