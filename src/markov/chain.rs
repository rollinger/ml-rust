#![allow(dead_code)]
#![allow(unused_imports)]

use std::collections::HashMap;
use rand::prelude::*;

struct MarkovChain {
	chain: HashMap<char, Vec<char>>,
	start_chars: Vec<char>,
}

impl MarkovChain {
	fn new() -> Self {
		MarkovChain {
			chain: HashMap::new(),
			start_chars: Vec::new(),
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_interface() {
		let mc = MarkovChain::new();
		assert!(mc.chain.is_empty())
	}
}