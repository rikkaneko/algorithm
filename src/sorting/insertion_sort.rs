pub const INSORT_CUTOFF: usize = 15;

/// Insertion sort
pub fn sort<T: Ord>(arr: &mut [T]) {
	for i in 1..arr.len() {
		let mut j = i;
		while j > 0 && arr[j - 1] > arr[j] {
			arr.swap(j - 1, j);
			j -= 1;
		}
	}
}

/// Insertion sort with binary search optimization
pub fn sort_v2<T: Ord>(arr: &mut [T]) {
	for i in 1..arr.len() {
		use crate::searching::binary_search::search_nearset;
		let pos = search_nearset(arr, &arr[i], 0, i);
		if pos < i {
			arr[pos..i+1].rotate_right(1);
		}
	}
}

#[cfg(test)]
mod tests {
	use super::{sort, sort_v2};
	use crate::sorting::shuffle::shuffle;
	
	#[test]
	fn test_sort() {
		let mut arr = [10, 9, 8, 7, 6, 5, 4, 3, 2, 1];
		shuffle(&mut arr);
		sort(&mut arr); // Test insertion sort
		assert_eq!(arr, [1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
		shuffle(&mut arr);
		sort_v2(&mut arr); // Test insertion sort with binary search
		assert_eq!(arr, [1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
		
	}
}
