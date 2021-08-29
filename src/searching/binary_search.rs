/// Binary search
pub fn search<T: Ord>(arr: &[T], key: &T, mut lo: usize, mut hi: usize) -> Result<usize, ()> {
	let mut mid: usize;
	while lo < hi {
		mid = lo + (hi - lo) / 2;
		#[allow(clippy::comparison_chain)]
		if &arr[mid] < key { lo = mid + 1; }
		else if &arr[mid] > key { hi = mid; }
		else { return Ok(mid); }
	}
	Err(())
}

// Nearest Binary search
pub fn search_nearset<T: Ord>(arr: &[T], key: &T, mut lo: usize, mut hi: usize) -> usize {
	// Return the lowest index if key smaller than the smallest element
	if &arr[lo] > key { return lo; }
	// Return the first index after the current range
	if &arr[hi - 1] < key { return hi; }
	let mut mid: usize;
	while lo < hi {
		mid = lo + (hi - lo) / 2;
		#[allow(clippy::comparison_chain)]
		if &arr[mid] < key { lo = mid + 1; }
		else if &arr[mid] > key { hi = mid; }
		else { return mid; }
	}
	hi
}

#[cfg(test)]
mod tests {
	use super::{ search, search_nearset };
	
	#[test]
	fn test_binary_search() {
		let arr = [12, 13, 19, 21, 23, 45];
		assert_eq!(search(&arr, &45, 0, arr.len()), Ok(5));
		assert_eq!(search(&arr, &12, 0, arr.len()), Ok(0));
		assert_eq!(search(&arr, &48, 0, arr.len()), Err(()));
		assert_eq!(search(&arr, &11, 0, arr.len()), Err(()));
		assert_eq!(search_nearset(&arr, &45, 0, arr.len()), 5);
		assert_eq!(search_nearset(&arr, &12, 0, arr.len()), 0);
		assert_eq!(search_nearset(&arr, &48, 0, arr.len()), 6);
		assert_eq!(search_nearset(&arr, &11, 0, arr.len()), 0);
	}
}
