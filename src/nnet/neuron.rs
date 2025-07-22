use super::utils::{sigmoid, dot};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct Neuron {
	pub weights: Vec<f64>,
	pub bias: f64,
	pub last_input: Vec<f64>,
	pub last_z: f64,
}

impl Neuron {
	pub fn from_parts(weights: Vec<f64>, bias: f64) -> Self {
		/* From layer initialization parts */
		return Self { weights, bias, last_input: vec![], last_z: 0.0 }
	}

	pub fn activate(&mut self, inputs: &[f64]) -> f64 {
		/* activation function */
		self.last_input = inputs.to_vec();
		self.last_z = dot(&self.weights, inputs) + self.bias;
		return sigmoid(self.last_z);
	}

	pub fn update_weights(&mut self, delta: f64, learning_rate: f64) {
		for (w, x) in self.weights.iter_mut().zip(&self.last_input) {
			*w -= learning_rate * delta * x;
		}
		self.bias -= learning_rate * delta;
	}
}