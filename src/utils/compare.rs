pub fn max(a: isize, b: isize) -> isize {
	if a > b { return a; } else {return b; }
}

pub fn min(a: isize, b:isize) -> isize {
	if a < b { return a; } else {return b; }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_max() {
		assert_eq!(max(10, 20), 20);
		assert_eq!(max(20, 10), 20);
		assert_eq!(max(100, 100), 100);
		assert_eq!(max(-10, -20), -10);
	}

	#[test]
	fn test_min() {
		assert_eq!(min(10, 20), 10);
		assert_eq!(min(20, 10), 10);
		assert_eq!(min(100, 100), 100);
		assert_eq!(min(-10, -20), -20);
	}
}