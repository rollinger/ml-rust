enum Interpretation {
	Unicode,
	Bytes,
}

pub fn distance(s1: &str, s2: &str) -> usize {
	/* Wrapper for unicode_edit_distance() assuming unicode interpretation of the string */
	return distance_unicode(s1, s2);
}

pub fn distance_unicode(s1: &str, s2: &str) -> usize {
	/* Wrapper for unicode_edit_distance() assuming unicode interpretation of the string */
	return levenshtein_edit_distance(s1, s2, Interpretation::Unicode);
}

pub fn distance_byte(s1: &str, s2: &str) -> usize {
	/* Wrapper for byte_edit_distance assuming a strict byte interpretation of the string */
	return levenshtein_edit_distance(s1, s2, Interpretation::Bytes);
}

fn levenshtein_edit_distance(s1: &str, s2: &str, mode: Interpretation) -> usize {
	/* Calculates the levenshtein edit distance between s1 to s2.
	Returns number of edits.  */
	match mode {
		Interpretation::Unicode => {
			let max_length = s1.chars().count().max(s2.chars().count());
			return count_edits(s1.chars(), s2.chars(), max_length);
		},
		Interpretation::Bytes => {
			let max_length = s1.len().max(s2.len());
			return count_edits(s1.bytes(), s2.bytes(), max_length);
		}
	}
}

fn count_edits<I,J,T>(mut iter1: I, mut iter2: J, max_length: usize) -> usize 
where
	I: Iterator<Item = T>,
	J: Iterator<Item = T>,
	T: PartialEq,
{
	/* Expects two iterators with PartialEq trait and a max_length. 
	Returns the number of edits s1 -> s2 
	*/
	let mut distance: usize = 0;
	for _ in 0..max_length {
		if iter1.next() != iter2.next() {
			distance += 1;
		}
	}
	return distance
}

#[cfg(test)]
mod tests {
    use super::*;

	#[test]
	fn test_parameters() {
		let s1 = "hello"; // stack string ref
		let s2 = String::from("hello"); // heap string ref
		assert_eq!(distance(s1, &s2), 0);
		assert_eq!(s1, "hello", "s1 became out of scope.");
		assert_eq!(s2, "hello", "s2 became out of scope.");
	}

    #[test]
    fn test_edit_distance_unicode() {
		let source = "hello";
		let alter1 = "hello";
		assert_eq!(distance(&source, &alter1), 0);
		let alter2 = "Hello";
		assert_eq!(distance(&source, &alter2), 1);
		let alter3 = "helloabcdef";
		assert_eq!(distance(&source, &alter3), 6);
		let alter4 = "hey";
		assert_eq!(distance(&source, &alter4), 3);
		let alter5 = "hello 😊!";
		assert_eq!(distance(&source, &alter5), 3);
	}
	
	#[test]
	fn test_edit_distance_bytes() {
		let source = "hello";
		let alter1 = "hello";
		assert_eq!(distance_byte(&source, &alter1), 0);
		let alter2 = "hell";
		assert_eq!(distance_byte(&source, &alter2), 1);
		let alter3 = "helloo";
		assert_eq!(distance_byte(&source, &alter3), 1);
		let alter4 = "hello😊";
		assert_eq!(distance_byte(&source, &alter4), 4);
		let alter4 = "hello 😊 world";
		assert_eq!(distance_byte(&source, &alter4), 4+7);
	}

	#[test]
	fn test_distance_handle_string() {
		let a = "hello world!";
		let b = &String::from("hello!");
		assert_eq!(distance(&a, &b), 7);
		assert_eq!(distance(b, a), 7);

		assert_eq!(a.len(), 12);
    	assert_eq!(b.len(), 6);
	}

	#[test]
	fn test_distance_under_different_length() {
		let a = "abc";
		let b = "abcxyz";
		let c = "";
		assert_eq!(distance(a, b), 3);
		assert_eq!(distance(b, a), 3);
		assert_eq!(distance(a, c), 3);
		assert_eq!(distance(c, a), 3);
		assert_eq!(distance(c, c), 0);
	}

}