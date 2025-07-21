use super::neuron::Neuron;

pub struct Layer {
	neurons: Vec<Neuron>
}

impl Layer {
	pub fn new(num_neurons: usize, input_size: usize, mut init_fn: impl FnMut() -> f64) -> Self {
		let neurons = init_dense_layer(input_size, num_neurons, &mut init_fn);
		return Self { neurons };
	}

	pub fn forward(&self, inputs: &[f64]) -> Vec<f64> {
		return self.neurons.iter().map(|n| n.activate(inputs)).collect();
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