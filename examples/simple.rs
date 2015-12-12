extern crate to_default;

fn main() {
	use to_default::*;
	let mut val = vec![ 1, 2 ];
	assert_eq!(val.to_default(), &[1, 2]);
	assert_eq!(val, Vec::new())
}
