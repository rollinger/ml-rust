use std::collections::HashMap;

pub fn stem(s: &str, unicode: bool) -> HashMap<char, usize> {
	/* Returns a frequency table of element in the sequence s. 
	@params
	- s: the sequence to stem
	- unicode: treating the elements as unicode (true) or raw bytes (false)

	@return
	- HashMap of frequency counts.
	*/

	//let mut freq: Vec<(char, usize)> = Vec::new();

	let mut freq: HashMap<char, usize> = HashMap::new();
	if unicode {
		freq = stem_unicode_sequence(s, freq);
	} else {
		freq = stem_byte_sequence(s, freq);
	}
	return freq;
}


fn stem_byte_sequence(s: &str, mut freq: HashMap<char, usize>) -> HashMap<char, usize> {
	// Treat string as byte-chars and return a bytewise frequency table
	for c in s.bytes() {
		let c = c as char;
		*freq.entry(c).or_insert(0) += 1;
	}
	freq
}

fn stem_unicode_sequence(s: &str, mut freq: HashMap<char, usize>) -> HashMap<char, usize> {
	// Treats string as unicode and returns a unicode frequency table
	for c in s.chars() {
		*freq.entry(c).or_insert(0) += 1;
	}
	freq
}



#[cfg(test)]
mod tests {
    use super::*;

	#[test]
    fn test_interface() {
		let s = String::from("aabbbccccd");
		let freq = stem(&s, true);
		assert!(!freq.is_empty(), "stem() did not return a HashMap")
	}

	#[test]
    fn test_unicode_stem() {
		let s = String::from("aabbbccccd");
		let freq = stem(&s, true);
		assert_eq!(freq.get(&'a'), Some(&2));
		assert_eq!(freq.get(&'b'), Some(&3));
		assert_eq!(*freq.get(&'c').unwrap(), 4);
		assert_eq!(freq.get(&'d'), Some(&1));
	}

	#[test]
    fn test_byte_stem() {
		let s = String::from("äöü");
		let freq = stem(&s, false);
		// println!("{freq:#?}");
		// println!("{:?}", s.as_bytes());
		assert_eq!(freq.get(&'¶'), Some(&1));
		assert_eq!(freq.get(&'¼'), Some(&1));
		assert_eq!(freq.get(&'¤'), Some(&1));
		assert_eq!(freq.get(&'Ã'), Some(&3));
	}
}