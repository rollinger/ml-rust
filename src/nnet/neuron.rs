use super::utils::{sigmoid, dot};

use rand::Rng;

pub struct Neuron {
	weights: Vec<f64>,
	bias: f64,
}

impl Neuron {
	pub fn new(input_size: usize) -> Self {
		let mut rng = rand::thread_rng();
		let weights = (0..input_size).map(|_| rng.gen_range(-1.0..1.0)).collect();
		let bias = rng.gen_range(-1.0..1.0);
		return Self { weights, bias };
	}

	pub fn activate(&self, inputs: &[f64]) -> f64 {
		return sigmoid(dot(&self.weights, inputs) + self.bias);
	}
}