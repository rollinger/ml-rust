use super::neuron::Neuron;

use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct Layer {
	pub neurons: Vec<Neuron>,
	last_output: Vec<f64>,
}

impl Layer {
	pub fn new(num_neurons: usize, input_size: usize, mut init_fn: impl FnMut() -> f64) -> Self {
		let neurons = init_dense_layer(input_size, num_neurons, &mut init_fn);
		return Self { neurons, last_output: vec![] };
	}

	pub fn forward(&mut self, inputs: &[f64]) -> Vec<f64> {
		self.last_output = self.neurons.iter_mut().map(|n| n.activate(inputs)).collect();
		return self.last_output.clone();
	}
}

fn init_dense_layer(input_size: usize, output_size: usize, mut init_fn: impl FnMut() -> f64) -> Vec<Neuron> {
	/* Initializes the layer's neurons from a init_function */
	(0..output_size).map(|_| {
		let weights = (0..input_size).map(|_| init_fn()).collect();
		let bias = init_fn();
		Neuron::from_parts(weights, bias)
	}).collect()
}