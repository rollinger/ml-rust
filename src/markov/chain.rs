#![allow(dead_code)]

use std::collections::HashMap;
use rand::prelude::*;
use serde::{Serialize, Deserialize};

use crate::persist::json::JsonPersist;

#[derive(Serialize, Deserialize)]
struct MarkovChain {
	chain: HashMap<char, Vec<char>>,
	start_chars: Vec<char>,
}

impl JsonPersist for MarkovChain {}

impl MarkovChain {
	pub fn new() -> Self {
		MarkovChain {
			chain: HashMap::new(),
			start_chars: Vec::new(),
		}
	}

	pub fn train(&mut self, input: &str) {
		/* Trains the chain on the input chars, considering a 2-char window at a time. */
		let chars: Vec<char> = input.chars().collect();
		if let Some(&first) = chars.first() {
			self.start_chars.push(first);
		}
		for window in chars.windows(2) {
			if let [current, next] = window {
				self.chain.entry(*current).or_insert_with(Vec::new).push(*next);
			}
		}
	}

	pub fn generate(&self, start: char, length: usize) -> String {
		/*  */
		let mut rng = thread_rng();
		let mut output = String::new();

		if self.start_chars.is_empty() {
			return output;
		}

		// Set the start as the current output position
		let mut current = start;
		output.push(current);

		// Generate the sequence length-1, choosing stochastically from the chain's next chars.
		for _ in 1..length {
			if let Some(next_chars) = self.chain.get(&current) {
				current = *next_chars.choose(&mut rng).unwrap();
				output.push(current)
			}
		}
		return output;
	}

	pub fn generate_from(&self, from: char, length: usize) -> String {
		/* Generates a sequence from char of length */
		return self.generate(from, length);
	}

	pub fn generate_from_rand(&self, length: usize) -> String {
		/* Generates a sequence from a random start char of length */
		let random_start = *self.start_chars.choose(&mut thread_rng()).unwrap();
		return self.generate(random_start, length);
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

	#[test]
	fn test_train() {
		let mut mc = MarkovChain::new();
		mc.train("abcdefghijklmnopqrstuvwxyz");
		assert_eq!(mc.chain.get(&'a'), Some(&vec!['b']));
		assert_eq!(mc.start_chars, vec!['a']);
		mc.train("xyzabc");
		assert_eq!(mc.chain.get(&'a'), Some(&vec!['b','b']));
		assert_eq!(mc.start_chars, vec!['a','x']);
	}

	#[test]
	fn test_generate() {
		let mut mc = MarkovChain::new();
		mc.train("abcdefghijklmnopqrstuvwxyz");
		mc.train("xyzabc");
		mc.train("abc def ghi jkl mno pqr stu vw xyza");
		let gen_1 = mc.generate_from_rand(6);
		assert_eq!(gen_1.len(), 6);
		let gen_2 = mc.generate_from('a', 3);
		assert_eq!(gen_2, "abc");
		let gen_2 = mc.generate_from('j', 3);
		assert_eq!(gen_2, "jkl");
		println!("{:?}", gen_1);
		println!("{:?}", gen_2);
		println!("{:?}", mc.chain);
		println!("{:?}", mc.start_chars);
	}

	fn test_empty_output() {
		let mut mc = MarkovChain::new();
		mc.train("abcdefghijklmnopqrstuvwxyz");
		mc.start_chars = vec![];
		let gen_1 = mc.generate_from_rand(6);
		assert_eq!(gen_1, "");
		let gen_2 = mc.generate_from('a', 6);
		assert_eq!(gen_2, "");
	}
}