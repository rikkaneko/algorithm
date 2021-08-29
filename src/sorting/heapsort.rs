use crate::data_structures::heap::MaxHeap;

/// Heapsort
pub fn sort<T: Ord + Clone>(arr: &mut [T]) {
	let mut heap = MaxHeap::from_arr(arr.to_vec());
	heap.build_heap();
	for i in (1..heap.size).rev() {
		heap.arr.swap(0, i);
		heap.size -= 1;
		heap.heapify(0);
	}
	arr.clone_from_slice(&heap.arr);
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
