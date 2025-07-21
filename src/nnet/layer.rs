use super::neuron::Neuron;

pub struct Layer {
	neurons: Vec<Neuron>
}

impl Layer {
	pub fn new(num_neurons: usize, input_size: usize) -> Self {
		let neurons = (0..num_neurons).map(|_| Neuron::new(input_size)).collect();
		return Self { neurons };
	}

	pub fn forward(&self, inputs: &[f64]) -> Vec<f64> {
		return self.neurons.iter().map(|n| n.activate(inputs)).collect();
	}
}