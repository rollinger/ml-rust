#![allow(dead_code)]
use crate::persist::json::JsonPersist;

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
		let nn = network::NeuralNetwork::new(&[2, 4, 3]);
		let input = vec![0.5,-0.2];
		let output = nn.predict(input);
		println!("Prediction: {:?}", output);
	}

	#[test]
	fn test_save_load() {
		let nn = network::NeuralNetwork::new(&[2, 4, 3]);
		match nn.save_to_file("nn.json", true) {
			Ok(_) => println!("Saved."),
			Err(e) => eprintln!("Error: {}", e),
		}
	}
}