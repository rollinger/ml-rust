pub fn sigmoid(x: f64) -> f64 {
	/* Returns the sigmoid of x */
	return 1.0/(1.0 + (-x).exp());
}

pub fn dot(v1: &[f64], v2: &[f64]) -> f64 {
	/* Dot Product of two vectors fo f64 */
	return v1.iter().zip(v2.iter()).map(|(a,b)| a * b).sum();
}