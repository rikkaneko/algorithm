use rand::Rng;
use super::insertion_sort::INSORT_CUTOFF;
/// Partition using Lomuto's Method
pub fn partition<T: Ord>(arr: &mut [T], lo: usize, hi: usize) -> usize {
	let mut i = lo;
	let mut rng = rand::thread_rng();
	// Select a random element as pivot
	arr.swap(rng.gen_range(lo..=hi), hi);
	// Partition to (lo..i)[i..hi:pivot)
	for j in lo..hi {
		if arr[j] < arr[hi] {
			arr.swap(i, j);
			i += 1;
		}
	}
	// (lo..i:pivot](i..hi]
	arr.swap(i, hi);
	i
}

/// Alternative Partition Scheme using Hoare's Method
pub fn partition_v2<T: Ord>(arr: &mut [T], lo: usize, hi: usize) -> usize {
	let mut rng = rand::thread_rng();
	arr.swap(rng.gen_range(lo..=hi), hi);
	let mut i = lo;
	let mut j = hi - 1;
	loop {
		// Use indexed value for pivot, instead of copies
		while arr[i] < arr[hi] { i += 1; }
		while j > lo && arr[j] > arr[hi] { j -= 1; }
		if i < j {
			arr.swap(i, j);
			// Special Thanks to @aoibird for the concept
			// Original implantation using do..while.. loop has the implication of i++, j--
			i += 1;
			j -= 1;
		} else {
			// Swap with j if the pivot is at lo, swap with i if the pivot is at hi
			arr.swap(i, hi);
			return i;
		}
	}
}

/// Quicksort (Recursive)
fn __sort<T: Ord>(arr: &mut [T], lo: usize, hi: usize) {
	if lo < hi {
		// Use Insertion sort to sort small subarray instead
		if hi - lo <= INSORT_CUTOFF {
			super::insertion_sort::sort_v2(&mut arr[lo..=hi]);
			return;
		}
		let p = partition(arr, lo, hi);
		if p > lo { __sort(arr, lo, p - 1); } // Recursive sort [lo..i:pivot)
		if p < hi { __sort(arr, p + 1, hi); } // Recursive sort (i:pivot..hi]
	}
}

/// Quicksort
pub fn sort<T: Ord>(arr: &mut [T]) {
	// Shuffle before sorting, select random pivot instead
	// super::shuffle::shuffle(arr);
	__sort(arr, 0, arr.len() - 1);
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

