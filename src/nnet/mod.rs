#![allow(dead_code)]

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
		let nn = network::NeuronalNetwork::new(&[2, 4, 3]);
		let input = vec![0.5,-0.2];
		let output = nn.predict(input);
		println!("Prediction: {:?}", output);
	}
}