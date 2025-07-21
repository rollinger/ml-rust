use super::utils::{sigmoid, dot};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct Neuron {
	weights: Vec<f64>,
	bias: f64,
}

impl Neuron {
	pub fn from_parts(weights: Vec<f64>, bias: f64) -> Self {
		/* From layer initialization parts */
		return Self { weights, bias }
	}

	pub fn activate(&self, inputs: &[f64]) -> f64 {
		/* activation function */
		return sigmoid(dot(&self.weights, inputs) + self.bias);
	}
}