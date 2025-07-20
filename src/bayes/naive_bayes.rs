use std::collections::HashMap;
use std::fs::File;
use std::io::{BufReader, BufWriter};
use serde::{Serialize, Deserialize};

use crate::persist::json::JsonPersist;

#[derive(Serialize, Deserialize)]
pub struct NaiveBayes {
	train_counts: HashMap<String, usize>,
	token_counts: HashMap<String, HashMap<String, usize>>,
	total_counts: HashMap<String, usize>,
}

impl JsonPersist for NaiveBayes {}

impl NaiveBayes {
	pub fn new() -> Self {
		NaiveBayes {
			train_counts: HashMap::new(),
			token_counts: HashMap::new(),
			total_counts: HashMap::new(),
		}
	}

	pub fn train(&mut self, label: String, token_vec: Vec<&str>) {
		// Increment or insert label into class_counts and token_counts
		*self.train_counts.entry(label.clone()).or_insert(0) += 1;
		let word_map = self.token_counts.entry(label.clone()).or_insert_with(HashMap::new);
		for token in token_vec {
			// Increment or insert the token in word_map and update total_counts
			*word_map.entry(token.to_string()).or_insert(0) += 1;
			*self.total_counts.entry(label.clone()).or_insert(0) += 1;
		}
	}

	pub fn bulk_train(&mut self, documents: Vec<(String, Vec<&str>)>) {
		/* Train the classifier on a vector of label, words_vector */
		for (label, words) in documents {
			self.train(label, words);
		}
	}

	fn log_prior(&self, label: &String) -> f64 {
		/* Return the log_prior probability for a label */
		let log_prior = (*self.train_counts.get(label).unwrap_or(&1) as f64).ln();
		return log_prior;
	}

	pub fn has_label(&self, label: &str) -> bool {
		return self.train_counts.contains_key(label);
	}

	pub fn score(&self, label: &String, token_vec: &Vec<&str>) -> Option<f64> {
		/* Returns the score for a label given a vecor of tokens */
		if !self.has_label(label) {return None;}

		let log_prior: f64 = self.log_prior(label);
		let mut log_likelihood: f64 = 0.0;

		for token in token_vec {
			let word_freq = self.token_counts.get(label).and_then(|t| t.get(*token)).unwrap_or(&0);
			let total = self.total_counts.get(label).unwrap_or(&1);
			let probability = (*word_freq as f64 + 1.0)  / (*total as f64 + 1.0);
			log_likelihood += probability.ln();
		}
		return Some(log_prior + log_likelihood);
	}

	pub fn classify(&self, token_vec: &Vec<&str>) -> Option<String> {
		/* classifies a vector of string belonging to a label */
		let mut best_label: Option<String> = None;
		let mut best_score = f64::NEG_INFINITY;

		for label in self.train_counts.keys() {
			match self.score(label, &token_vec) {
				Some(score) => { 
					if score > best_score {
						best_score = score;
						best_label = Some(label.clone());
					}
				},
				None => (),
			}
		}

		return best_label
	}

	pub fn classification_table(&self, token_vec: &Vec<&str>) -> HashMap<String, f64> {
		/* Calculates the scores of all labels given the token_vec */
		let mut table: HashMap<String, f64> = HashMap::new();
		for label in self.train_counts.keys() {
			match self.score(label, &token_vec) {
				Some(score) => { table.insert(label.to_string(), score); },
				None => (),
			}
		}
		return table;
	}

}

#[cfg(test)]
mod tests {
    use super::*;

	#[test]
    fn test_interface() {
		let nb: NaiveBayes = NaiveBayes::new();
		assert!(nb.train_counts.is_empty(), "NaiveBayes::new() did not have a train_counts or is not empty.");
		assert!(nb.token_counts.is_empty(), "NaiveBayes::new() did not have a token_counts or is not empty.");
		assert!(nb.total_counts.is_empty(), "NaiveBayes::new() did not have a total_counts or is not empty.");
	}

	#[test]
	fn test_train() {
		let mut nb: NaiveBayes = NaiveBayes::new();
		nb.train("A".to_string(), vec!["aaa", "bbb", "ccc", "ddd"]);
		nb.train("B".to_string(), vec!["ddd", "eee"]);
		assert_eq!(nb.train_counts.get("A"), Some(&1));
		assert_eq!(nb.train_counts.get("B"), Some(&1));
		nb.train("B".to_string(), vec!["eee", "fff"]);
		assert_eq!(nb.train_counts.get("B"), Some(&2));

		let training_data = vec![
			("A".to_string(), vec!["bbb", "xxx", "yyy"]),
			("B".to_string(), vec!["eee", "yyy", "zzz"]),
			("C".to_string(), vec!["iii", "jjj", "kkk", "lll"]),
			("B".to_string(), vec!["eee", "ddd", "zzz"]),
		];
		nb.bulk_train(training_data);
		assert_eq!(nb.train_counts.get("A"), Some(&2));
		assert_eq!(nb.train_counts.get("B"), Some(&4));
		assert_eq!(nb.train_counts.get("C"), Some(&1));
		assert_eq!(nb.train_counts.get("X"), None);

	}

	#[test]
	fn test_classify() {
		let mut nb: NaiveBayes = NaiveBayes::new();
		let training_data = vec![
			("spam".to_string(), vec!["buy", "cheap", "meds"]),
			("spam".to_string(), vec!["cheap", "pills", "offer"]),
			("nospam".to_string(), vec!["offer", "let's", "meet", "for", "lunch", "tonight"]),
			("nospam".to_string(), vec!["see", "you", "at", "the", "party", "tonight"]),
		];
		nb.bulk_train(training_data);
		let spam_vector = vec!["cheap","meds","offer"];
		println!("{:?}", nb.classification_table(&spam_vector));
		assert_eq!(nb.classify(&spam_vector), Some("spam".to_string()));

		let nospam_vector = vec!["meet","lunch","tonight"];
		println!("{:?}", nb.classification_table(&nospam_vector));
		assert_eq!(nb.classify(&nospam_vector), Some("nospam".to_string()));

		let ambiguous_vector = vec!["offer", "meet", "for", "lunch", "tonight"];
		println!("{:?}", nb.classification_table(&ambiguous_vector));
		assert_eq!(nb.classify(&ambiguous_vector), Some("nospam".to_string()));
	}


}