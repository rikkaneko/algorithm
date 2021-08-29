/// Selection sort
pub fn sort<T: Ord>(arr: &mut [T]) {
	for i in 0..arr.len() {
		let mut smallest = i;
		for j in i+1..arr.len() {
			if arr[smallest] > arr[j] { smallest = j; }
		}
		arr.swap(smallest, i);
	}
}

#[cfg(test)]
mod tests {
	use super::sort;
	use crate::sorting::shuffle::shuffle;
	
	#[test]
	fn test_sort() {
		let mut arr = [10, 9, 8, 7, 6, 5, 4, 3, 2, 1];
		shuffle(&mut arr);
		sort(&mut arr);
		assert_eq!(arr, [1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
	}
}
