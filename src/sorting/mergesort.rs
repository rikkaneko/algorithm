const INSORT_CUTOFF: usize = 15;

/// Merge two sorted array
fn merge<T: Ord + Clone>(arr: &mut [T], aux: &mut [T], lo: usize, mid: usize, hi: usize) {
	aux[lo..=hi].clone_from_slice(&arr[lo..=hi]);
	// current position of the first and second array
	let mut i = lo;
	let mut j = mid + 1;
	for k in lo..=hi {
		// Merge the remaining elements
		// As aux array is used, we cannot assign the last elements to infinity like page 31
		if i > mid {
			arr[k] = aux[j].clone();
			j += 1;
		} else if j > hi {
			arr[k] = aux[i].clone();
			i += 1;
		// Compare and select the smaller one
		} else if aux[i] >= aux[j] {
			arr[k] = aux[j].clone();
			j += 1;
		} else {
			arr[k] = aux[i].clone();
			i += 1;
		}
		
	}
}
/// Mergesort (Recursive)
fn __sort<T: Ord + Clone>(arr: &mut [T], aux: &mut [T], lo: usize, hi: usize) {
	if lo < hi {
		// Use Insertion sort to sort small subarray instead
		if hi - lo <= INSORT_CUTOFF {
			super::insertion_sort::sort_v2(&mut arr[lo..=hi]);
			return;
		}
		// Better alternative of mid = (lo + hi) / 2
		let mid = lo + (hi - lo) / 2;
		// Divide the array slice to arr[lo..mid] and arr[mid+1..hi]
		__sort(arr, aux, lo, mid);
		__sort(arr, aux, mid + 1, hi);
		// Skip merge if the subarray already in place
		if arr[mid] > arr[mid + 1] {
			merge(arr, aux, lo, mid, hi);
		}
		
	}
}

/// Mergesort
pub fn sort<T: Ord + Clone>(arr: &mut [T]) {
	let mut aux: Vec<T> = arr.to_vec();
	__sort(arr, &mut aux, 0, arr.len() - 1);
}

#[cfg(test)]
mod tests {
	use super::sort;
	use crate::sorting::shuffle::shuffle;
	
	#[test]
	fn test_sort() {
		let mut arrx = [
			[10, 9, 8, 7, 6, 5, 4, 3, 2, 1],
			[4, 9, 10, 8, 9, 5, 1, 2, 3, 7],
			[0, 0, 1, 0, 1, 1, 0, 1, 0, 1]
		];
		for arr in arrx.iter_mut() {
			shuffle(arr);
			sort(arr);
		}
		assert_eq!(arrx[0], [1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
		assert_eq!(arrx[1], [1, 2, 3, 4, 5, 7, 8, 9, 9, 10]);
		assert_eq!(arrx[2], [0, 0, 0, 0, 0, 1, 1, 1, 1, 1]);
	}
}
