pub fn max(a: isize, b: isize) -> isize {
	if a > b { return a; } else {return b; }
}

pub fn min(a: isize, b:isize) -> isize {
	if a < b { return a; } else {return b; }
}

pub fn approx_eq(a:f64, b:f64, epsilon:f64) -> bool {
	/* Returns true if the absolute difference of a and be is smaller than epsilon
	Epsilon = 1e-10 is a good value to start.
	 */
	return (a-b).abs() < epsilon;
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

	#[test]
	fn test_approx_eq() {
		assert_eq!(approx_eq(1.0, 1.1, 1e-10), false);
	}
}