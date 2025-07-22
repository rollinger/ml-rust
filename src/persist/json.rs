use serde::{Serialize, de::DeserializeOwned};
use std::fs::File;
use std::io::{BufReader, BufWriter, Result};

pub trait JsonPersist: Serialize + DeserializeOwned + Sized {
	/*
	Json persistance of struct fields.

	Note persisting f64 to json may entail small rounding differences.
	*/
	fn save_to_file(&self, path: &str, pretty:bool) -> Result<()> {
		let file = File::create(path)?;
		let writer = BufWriter::new(file);
		if pretty {
			serde_json::to_writer_pretty(writer, self)?;
		} else {
			serde_json::to_writer(writer, self)?;
		}
		return Ok(());
	}

	fn load_from_file(path: &str) -> Result<Self> {
		let file = File::open(path)?;
		let reader = BufReader::new(file);
		let obj = serde_json::from_reader(reader)?;
		return Ok(obj);
	}
}