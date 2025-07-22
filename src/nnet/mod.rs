#![allow(dead_code)]
use crate::persist::json::JsonPersist;
use std::path::Path;
use std::fs;

pub mod utils;
pub mod neuron;
pub mod layer;
pub mod network;

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_nnet() {
		// 2 input, 1 hidden layer of 4, 1 output layer of 3 neurons
		let mut nn = network::NeuralNetwork::new(&[2, 4, 3]);
		let input = vec![0.5,-0.2];
		let output = nn.predict(input);
		println!("Prediction: {:?}", output);
	}

	#[test]
	fn test_save_load() {
		let path = "nn.json".to_string();
		let nn1 = network::NeuralNetwork::new(&[2, 4, 3]);
		match nn1.save_to_file(&path, true) {
			Ok(_) => println!("Saved."),
			Err(e) => eprintln!("Error: {}", e),
		}
		// assert file for nn1 exists
		assert!(Path::new(&path).exists());
		
		let nn2 = network::NeuralNetwork::load_from_file(&path).expect("Error loading file");
		
		for (layer1, layer2) in nn1.layers.iter().zip(nn2.layers.iter()) {
			for (n1, n2) in layer1.neurons.iter().zip(layer2.neurons.iter()) {
				for (w1, w2) in n1.weights.iter().zip(n2.weights.iter()) {
					assert!(utils::approx_eq(*w1, *w2, 1e-10), "Weights differ: {} != {}", w1, w2);
				}
				assert!(utils::approx_eq(n1.bias, n2.bias, 1e-10), "Biases differ: {} != {}", n1.bias, n2.bias);
			}
		}
		// delete file
		fs::remove_file(&path).expect("Failed to delete test file.");
	}
}