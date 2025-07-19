pub fn stem(s: &str, unicode: bool) -> Vec<(char, usize)> {
	/* Returns a frequency table of element in the sequence s. 
	@params
	- s: the sequence to stem
	- unicode: treating the elements as unicode (true) or raw bytes (false)
	*/
	let mut freq: Vec<(char, usize)> = Vec::new();
	if unicode {
		freq = stem_unicode_sequence(s);
	} else {
		freq = stem_byte_sequence(s);
	}
	return freq;
}


fn stem_byte_sequence(s: &str, freq: Vec<(char, usize)>) -> Vec<(char, usize)> {
	// Treat string as byte-chars and return a bytewise frequency table
	for c in s.bytes() {
		let c = c as char;
		match freq.iter_mut().find(|(ch, _)| *ch == c) {
			Some((_, count)) => *count += 1,
			None => freq.push((c,1)),
		}
	}
	freq
}

fn stem_unicode_sequence(s: &str, freq: Vec<(char, usize)>) -> Vec<(char, usize)> {
	// Treats string as unicode and returns a unicode frequency table
	for c in s.chars() {
		match freq.iter_mut().find(|(ch, _)| *ch == c) {
			Some((_, count)) => *count += 1,
			None => freq.push((c, 1)),
		}
	}
	freq
}



#[cfg(test)]
mod tests {
    use super::*;

	#[test]
    fn test_unicode() {
		s = String::from("aabbbccccd");
		freq = stem(&s, true);
		assert_eq!(freq.entry('a'), 2);
	}

	#[test]
    fn test_bytes() {}
}