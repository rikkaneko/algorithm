use crate::sorting::quicksort::partition;

fn __select<T: Ord + Clone>(arr: &mut [T], lo: usize, hi: usize, order: usize) -> T {
	if lo == hi { return arr[lo].clone() }
	let p = partition(arr, lo, hi);
	let k = p - lo + 1;
	if order == k { arr[p].clone() }
	else if order < k { __select(arr, lo, p - 1, order) }
	else { __select(arr, p + 1, hi, order - k) }
}

/// Quickselect (Randomized-Select)
pub fn select<T: Ord + Clone>(arr: &[T], order: usize) -> T {
	if order == 0 || order > arr.len() { panic!("Order out of bound.") }
	let mut t = arr.to_vec();
	__select(&mut t, 0, arr.len() - 1, order)
}

#[cfg(test)]
mod tests {
	use super::select;
	use crate::sorting::quicksort::sort;
	
	#[test]
	fn test_select() {
		let arr = [9, 4, 6, 0, 1, -10, 15, 5, 7, 10, 1, 4];
		let mut result = arr;
		sort(&mut result);
		for i in 1..=10 {
			assert_eq!(select(&arr, i), result[i - 1], "[i = {}]", i);
		}
	}
}
