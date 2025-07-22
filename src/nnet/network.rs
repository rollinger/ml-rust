use rand::Rng;
use serde::{Serialize, Deserialize};

use crate::persist::json::JsonPersist;
use super::utils::sigmoid_derivative;
use super::layer::Layer;

#[derive(Serialize, Deserialize)]
pub struct NeuralNetwork {
	pub layers: Vec<Layer>,
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

	pub fn predict(&mut self, mut inputs: Vec<f64>) -> Vec<f64> {
		for layer in &mut self.layers {
			inputs = layer.forward(&inputs);
		}
		return inputs;
	}

	pub fn train(&mut self, input: Vec<f64>, target: Vec<f64>, learning_rate: f64) {
		let output = self.predict(input);

        let mut deltas: Vec<Vec<f64>> = Vec::new();

        for (i, layer) in self.layers.iter().rev().enumerate() {
            let idx = self.layers.len() - 1 - i;
            let mut layer_deltas = Vec::new();

            if i == 0 {
                for (o, t) in output.iter().zip(&target) {
                    let z = layer.neurons[layer_deltas.len()].last_z;
                    let d = (o - t) * sigmoid_derivative(z);
                    layer_deltas.push(d);
                }
            } else {
                let next_layer = &self.layers[idx + 1];
                for (j, neuron) in layer.neurons.iter().enumerate() {
                    let z = neuron.last_z;
                    let mut sum = 0.0;
                    for (k, next_neuron) in next_layer.neurons.iter().enumerate() {
                        sum += next_neuron.weights[j] * deltas[0][k];
                    }
                    layer_deltas.push(sum * sigmoid_derivative(z));
                }
            }

            deltas.insert(0, layer_deltas);
        }

        for (layer, layer_deltas) in self.layers.iter_mut().zip(deltas) {
            for (neuron, delta) in layer.neurons.iter_mut().zip(layer_deltas) {
                neuron.update_weights(delta, learning_rate);
            }
        }
	}

	pub fn input_size(&self) -> usize {
		todo!("Implement introspection in the nn input dimension")
	}

	pub fn output_size(&self) -> usize {
		todo!("Implement introspection in the nn output dimension")
	}

	pub fn hidden_layers_size(&self) -> usize {
		todo!("Implement introspection in the number of hiden layers")
	}
}
