use rand::Rng;
use serde::{Serialize, Deserialize};

use crate::persist::json::JsonPersist;
use super::layer::Layer;

#[derive(Serialize, Deserialize)]
pub struct NeuralNetwork {
	layers: Vec<Layer>,
}

impl JsonPersist for NeuralNetwork {}

impl NeuralNetwork {
	pub fn new(sizes: &[usize]) -> Self {
		let mut layers = Vec::new();
		let mut rng = rand::thread_rng();
		for w in sizes.windows(2) {
			layers.push(Layer::new(w[1], w[0], || rng.gen_range(-1.0..1.0)));
		}
		return Self { layers };
	}

	pub fn predict(&self, mut inputs: Vec<f64>) -> Vec<f64> {
		for layer in &self.layers {
			inputs = layer.forward(&inputs);
		}
		return inputs;
	}
}
