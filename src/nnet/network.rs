use super::layer::Layer;

pub struct NeuronalNetwork {
	layers: Vec<Layer>,
}

impl NeuronalNetwork {
	pub fn new(sizes: &[usize]) -> Self {
		let mut layers = Vec::new();
		for w in sizes.windows(2) {
			layers.push(Layer::new(w[1],w[0]));
		}
		return Self { layers };
	}

	pub fn predict(&self, mut inputs: Vec<f64>) -> Vec<f64> {
		for layer in &self.layers {
			inputs = layer.forward(&inputs);
		}
		return inputs;
	}
}
