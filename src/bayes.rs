use std::collections::HashMap;
use serde::{Serialize, Deserialize};
use std::fs::File;
use std::io::{BufReader, BufWriter};

#[derive(Serialize, Deserialize)]
pub struct NaiveBayes {
	class_counts: HashMap<String, usize>,
	word_counts: HashMap<String, HashMap<String, usize>>,
	total_words: HashMap<String, usize>,
}

impl NaiveBayes {
	pub fn new() -> Self {
		NaiveBayes {
			class_counts: HashMap::new(),
			word_counts: HashMap::new(),
			total_words: HashMap::new(),
		}
	}

	pub fn save_to_file(&self, path: &str, pretty: bool) -> std::io::Result<()> {
		let file = File::create(path)?;
		let writer = BufWriter::new(file);
		if pretty {
			serde_json::to_writer_pretty(writer, self)?;
		} else {
			serde_json::to_writer(writer, self)?;
		}
		return Ok(());
	}

	pub fn load_from_file(path: &str) -> std::io::Result<Self> {
		let file = File::open(path)?;
		let reader = BufReader::new(file);
		let nb = serde_json::from_reader(reader)?;
		return Ok(nb);
	}

	pub fn train(&mut self, label: String, token_vec: Vec<&str>) {
		// Increment or insert label into class_counts and word_counts
		*self.class_counts.entry(label.clone()).or_insert(0) += 1;
		let word_map = self.word_counts.entry(label.clone()).or_insert_with(HashMap::new);
		for token in token_vec {
			// Increment or insert the token in word_map and update total_words
			*word_map.entry(token.to_string()).or_insert(0) += 1;
			*self.total_words.entry(label.clone()).or_insert(0) += 1;
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
		let log_prior = (*self.class_counts.get(label).unwrap_or(&1) as f64).ln();
		return log_prior;
	}

	pub fn has_label(&self, label: &str) -> bool {
		return self.class_counts.contains_key(label);
	}

	pub fn score(&self, label: &String, token_vec: &Vec<&str>) -> Option<f64> {
		/* Returns the score for a label given a vecor of tokens */
		if !self.has_label(label) {return None;}

		let log_prior: f64 = self.log_prior(label);
		let mut log_likelihood: f64 = 0.0;

		for token in token_vec {
			let word_freq = self.word_counts.get(label).and_then(|t| t.get(*token)).unwrap_or(&0);
			let total = self.total_words.get(label).unwrap_or(&1);
			let probability = (*word_freq as f64 + 1.0)  / (*total as f64 + 1.0);
			log_likelihood += probability.ln();
		}
		return Some(log_prior + log_likelihood);
	}

	pub fn classification_table(&self, token_vec: &Vec<&str>) -> HashMap<String, f64> {
		/* Calculates the scores of all labels given the token_vec */
		let mut table: HashMap<String, f64> = HashMap::new();
		for label in self.class_counts.keys() {
			match self.score(label, &token_vec) {
				Some(score) => { table.insert(label.to_string(), score); },
				None => (),
			}
		}
		return table;
	}

	pub fn predict(&self, input: &Vec<&str>) -> Option<String> {
		/* predict a vector of string belonging to a label */
		let mut best_label: Option<String> = None;
		let mut best_score = f64::NEG_INFINITY;

		for label in self.class_counts.keys() {
			let log_prior = (*self.class_counts.get(label).unwrap_or(&1) as f64).ln();
			let mut log_likelihood = 0.0;

			for word in input {
				// calculates the probability of each input word; add to log_likelihood
				let word_freq = self.word_counts.get(label).and_then(|m| m.get(*word)).unwrap_or(&0);
				let total = self.total_words.get(label).unwrap_or(&1);
				let prob = (*word_freq as f64 + 1.0) / (*total as f64 + 1.0);
				log_likelihood += prob.ln();
			}
			
			// Select better match based on prior + likelihood
			let score = log_prior + log_likelihood;
			if score > best_score {
				best_score = score;
				best_label = Some(label.clone());
			}
		}

		return best_label
	}

}