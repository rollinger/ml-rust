#![allow(dead_code)]

use rand::Rng;

fn sigmoid(x: f64) -> f64 {
	/* Returns the sigmoid of x */
	return 1.0/(1.0 + (-x).exp());
}

fn dot(v1: &[f64], v2: &[f64]) -> f64 {
	/* Dot Product of two vectors fo f64 */
	return v1.iter().zip(v2.iter()).map(|(a,b)| a * b).sum();
}

struct Neuron {
	weights: Vec<f64>,
	bias: f64,
}

impl Neuron {
	fn new(input_size: usize) -> Self {
		let mut rng = rand::thread_rng();
		let weights = (0..input_size).map(|_| rng.gen_range(-1.0..1.0)).collect();
		let bias = rng.gen_range(-1.0..1.0);
		return Self { weights, bias };
	}

	fn activate(&self, inputs: &[f64]) -> f64 {
		return sigmoid(dot(&self.weights, inputs) + self.bias);
	}
}

struct Layer {
	neurons: Vec<Neuron>
}

impl Layer {
	fn new(num_neurons: usize, input_size: usize) -> Self {
		let neurons = (0..num_neurons).map(|_| Neuron::new(input_size)).collect();
		return Self { neurons };
	}

	fn forward(&self, inputs: &[f64]) -> Vec<f64> {
		return self.neurons.iter().map(|n| n.activate(inputs)).collect();
	}
}

struct NeuronalNetwork {
	hidden: Layer,
	output: Neuron,
}

impl NeuronalNetwork {
	fn new(input_size: usize, hidden_size: usize) -> Self {
		return Self {
			hidden: Layer::new(hidden_size, input_size),
			output: Neuron::new(hidden_size),
		}
	}

	fn predict(&self, inputs: &[f64]) -> f64 {
		let hidden_out = self.hidden.forward(inputs);
		return self.output.activate(&hidden_out);
	}
}


#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_nnet() {
		let nn = NeuronalNetwork::new(2,4);
		let input = vec![0.5,-0.2];
		let output = nn.predict(&input);
		println!("Prediction: {:.4}", output);
	}
}