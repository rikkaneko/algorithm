use rand::Rng;

/// Knuth shuffle
pub fn shuffle<T: Ord>(arr: &mut [T]) {
	let mut rng = rand::thread_rng();
	for i in 0..arr.len() {
		let r = rng.gen_range(0..=i);
		arr.swap(i, r);
	}
}

#[cfg(test)]
mod tests {
	use super::shuffle;
	
	#[test]
	fn test_shuffle() {
		let mut arr = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
		for _i in 0..10 {
			shuffle(&mut arr);
			println!("{:?}", arr);
		}
	}
}
