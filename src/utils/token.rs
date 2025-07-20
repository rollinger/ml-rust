#[allow(unused)]
pub fn explode_usn(sequence: &str, n: usize) -> Vec<String> {
	/* Explodes a unicode sequence into n-sized windows and returns a vector of strings*/
	todo!("Implement sequence explode by n")
}

#[allow(unused)]
pub fn explode_usd(sequence: &str, delim: &str, include: bool) -> Vec<String> {
	/* Explodes a unicode sequence into delimiter-bounded windows and returns a vector of string.
	if include is true, the delimiter is kept */
	todo!("Implement sequence explode by delimiter")
}

#[cfg(tests)]
mod tests {
	use super::*;

	#[test]
	fn test_explode_usn() {}

	#[test]
	fn test_explode_usd() {}
}