pub fn left_child(idx: usize) -> usize { 2 * idx + 1 }

pub fn right_child(idx: usize) -> usize { 2 * idx + 2 }

pub fn max_heapify<T: Ord>(arr: &mut [T], size: usize, mut idx: usize) {
	loop {
		let left = left_child(idx);
		let right = right_child(idx);
		let mut largest = idx;
		if left < size && arr[left] > arr[largest] { largest = left; }
		if right < size && arr[right] > arr[largest] { largest = right; }
		if largest != idx {
			arr.swap(idx, largest);
			idx = largest;
		} else { break }
	}
}

pub fn build_maxheap<T: Ord>(arr: &mut [T]) {
	let size = arr.len();
	for idx in (0..((size - 1) / 2)).rev() {
		max_heapify(arr, size, idx);
	}
}

/// Heapsort
pub fn sort<T: Ord>(arr: &mut [T]) {
	build_maxheap(arr);
	let mut size = arr.len();
	for i in (1..size).rev() {
		arr.swap(0, i);
		size -= 1;
		max_heapify(arr, size, 0);
	}
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
