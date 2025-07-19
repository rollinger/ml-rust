pub fn distance(s1: &str, s2: &str) -> usize {
	/* Returns the Levenshtein Distance assuming unicode interpretation of the string */
	return unicode_edit_distance(s1, s2);
}

pub fn unicode_edit_distance(s1: &str, s2: &str) -> usize {
	/* 
	Unicode-wise Levenshtein edit distance. 
	Returns the number of edits needed to transform s1 to s2.
	*/
	let mut distance: usize = 0;
	let mut s1_chars = s1.chars();
	let mut s2_chars = s2.chars();
	let max_length = s1.chars().count().max(s2.chars().count());

	for _ in 0..max_length {
		let c1 = s1_chars.next();
		let c2 = s2_chars.next();
		if c1 != c2 {
			distance += 1;
		}
	}

	distance
}


pub fn byte_edit_distance(s1: &str, s2: &str) -> usize {
	let mut distance: usize = 0;
	let s1_bytes = s1.as_bytes();
	let s2_bytes = s2.as_bytes();
	let max_length = s1.len().max(s2.len());

	for idx in 0..max_length {
		let b1 = s1_bytes.get(idx).copied();
		let b2 = s2_bytes.get(idx).copied();
		if b1 != b2 {
			distance += 1;
		}
	}
	distance
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_edit_distance_unicode() {
		let source = "hello";
		let alter1 = "hello";
		assert_eq!(unicode_edit_distance(&source, &alter1), 0);
		let alter2 = "Hello";
		assert_eq!(unicode_edit_distance(&source, &alter2), 1);
		let alter3 = "helloabcdef";
		assert_eq!(unicode_edit_distance(&source, &alter3), 6);
		let alter4 = "hey";
		assert_eq!(unicode_edit_distance(&source, &alter4), 3);
		let alter5 = "hello 😊!";
		assert_eq!(unicode_edit_distance(&source, &alter5), 3);
	}
	
	#[test]
	fn test_edit_distance_bytes() {
		let source = "hello";
		let alter1 = "hello";
		assert_eq!(byte_edit_distance(&source, &alter1), 0);
		let alter2 = "hell";
		assert_eq!(byte_edit_distance(&source, &alter2), 1);
		let alter3 = "helloo";
		assert_eq!(byte_edit_distance(&source, &alter3), 1);
		let alter4 = "hello😊";
		assert_eq!(byte_edit_distance(&source, &alter4), 4);
		let alter4 = "hello 😊 world";
		assert_eq!(byte_edit_distance(&source, &alter4), 4+7);
	}

	#[test]
	fn test_distance_handle_string() {
		let a = "hello world!";
		let b = &String::from("hello!");
		assert_eq!(unicode_edit_distance(&a, &b), 7);
		assert_eq!(unicode_edit_distance(b, a), 7);

		assert_eq!(a.len(), 12);
    	assert_eq!(b.len(), 6);
	}
}