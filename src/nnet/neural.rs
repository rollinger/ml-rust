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
	layers: Vec<Layer>,
}

impl NeuronalNetwork {
	fn new(sizes: &[usize]) -> Self {
		let mut layers = Vec::new();
		for w in sizes.windows(2) {
			layers.push(Layer::new(w[1],w[0]));
		}
		return Self { layers };
	}

	fn predict(&self, mut inputs: Vec<f64>) -> Vec<f64> {
		for layer in &self.layers {
			inputs = layer.forward(&inputs);
		}
		return inputs;
	}
}


#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_nnet() {
		// 2 input, 1 hidden layer of 4, 1 output layer of 3 neurons
		let nn = NeuronalNetwork::new(&[2, 4, 3]);
		let input = vec![0.5,-0.2];
		let output = nn.predict(input);
		println!("Prediction: {:?}", output);
	}
}