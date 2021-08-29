pub struct MaxHeap<T: Ord> {
	pub arr: Vec<T>,
	pub size: usize
}

pub struct MinHeap<T: Ord> {
	pub arr: Vec<T>,
	pub size: usize
}

pub fn parent(idx: usize) -> usize {
	if idx > 0 { (idx - 1) / 2 } else { 0 }
}

pub fn left_child(idx: usize) -> usize {
	2 * idx + 1
}

pub fn right_child(idx: usize) -> usize {
	2 * idx + 2
}

/// Max-Heap Implementation
impl<T: Ord> MaxHeap<T> {
	pub fn from_arr(arr: Vec<T>) -> MaxHeap<T> {
		let size = arr.len();
		MaxHeap {
			arr,
			size
		}
	}
	
	pub fn new() -> MaxHeap<T> {
		MaxHeap {
			arr: Vec::new(),
			size: 0
		}
	}
	
	/// Iterative Max-Heapify
	pub fn heapify(&mut self, mut idx: usize) -> &mut Self {
		loop {
			let left = left_child(idx);
			let right = right_child(idx);
			let mut largest = idx;
			if left < self.size && self.arr[left] > self.arr[largest] { largest = left; }
			if right < self.size && self.arr[right] > self.arr[largest] { largest = right; }
			if largest != idx {
				self.arr.swap(idx, largest);
				idx = largest;
			} else { break; }
		}
		self
	}
	
	/// Build Max-Heap
	pub fn build_heap(&mut self) -> &mut Self {
		for idx in (0..((self.size - 1) / 2)).rev() {
			self.heapify(idx);
		}
		self
	}
	
	/// Extract (remove) the maximum element from the heap
	pub fn extract_max(&mut self) -> Result<T, ()> {
		if self.size == 0 {
			return Err(())
		}
		let val = self.arr.swap_remove(0);
		self.size -= 1;
		self.heapify(0);
		Ok(val)
	}
	
	pub fn swim(&mut self, mut idx: usize) {
		while idx > 0 && self.arr[parent(idx)] < self.arr[idx] {
			self.arr.swap(parent(idx), idx);
			idx = parent(idx);
		}
	}
	
	pub fn insert_key(&mut self, key: T) -> &mut Self {
		self.arr.push(key);
		self.size += 1;
		self.swim(self.size - 1);
		self
	}
	
	pub fn max(&self) -> Result<&T, ()> {
		if self.size < 1 {
			return Err(())
		}
		Ok(&self.arr[0])
	}
	
	pub fn empty(&self) -> bool {
		self.size == 0
	}
}

impl<T: Ord> MinHeap<T> {
	pub fn from_arr(arr: Vec<T>) -> MinHeap<T> {
		let size = arr.len();
		MinHeap {
			arr,
			size
		}
	}
	
	pub fn new() -> MinHeap<T> {
		MinHeap {
			arr: Vec::new(),
			size: 0
		}
	}
	
	/// Iterative Max-Heapify
	pub fn heapify(&mut self, mut idx: usize) -> &mut Self {
		loop {
			let left = left_child(idx);
			let right = right_child(idx);
			let mut smallest = idx;
			if left < self.size && self.arr[left] < self.arr[smallest] { smallest = left; }
			if right < self.size && self.arr[right] < self.arr[smallest] { smallest = right; }
			if smallest != idx {
				self.arr.swap(idx, smallest);
				idx = smallest;
			} else { break; }
		}
		self
	}
	
	/// Build Max-Heap
	pub fn build_heap(&mut self) -> &mut Self {
		for idx in (0..((self.size - 1) / 2)).rev() {
			self.heapify(idx);
		}
		self
	}
	
	/// Extract (remove) the maximum element from the heap
	pub fn extract_min(&mut self) -> Result<T, ()> {
		if self.size == 0 {
			return Err(())
		}
		let val = self.arr.swap_remove(0);
		self.size -= 1;
		self.heapify(0);
		Ok(val)
	}
	
	pub fn swim(&mut self, mut idx: usize) {
		while idx > 0 && self.arr[parent(idx)] > self.arr[idx] {
			self.arr.swap(parent(idx), idx);
			idx = parent(idx);
		}
	}
	
	pub fn insert_key(&mut self, key: T) -> &mut Self {
		self.arr.push(key);
		self.size += 1;
		self.swim(self.size - 1);
		self
	}
	
	pub fn min(&self) -> Result<&T, ()> {
		if self.size < 1 {
			return Err(())
		}
		Ok(&self.arr[0])
	}
	
	pub fn empty(&self) -> bool {
		self.size == 0
	}
}

#[cfg(test)]
mod tests {
	use super::{MaxHeap, MinHeap};
	
	#[test]
	fn test_max_heap() -> Result<(), ()> {
		let mut heap = MaxHeap::new();
		let mut result = Vec::new();
		heap.insert_key(1)
			.insert_key(4)
			.insert_key(-100)
			.insert_key(100)
			.insert_key(0)
			.insert_key(20)
			.insert_key(4);
		while !heap.empty() {
			result.push(heap.extract_max()?)
		}
		assert_eq!(result, vec![100, 20, 4, 4, 1, 0, -100]);
		Ok(())
	}
	
	#[test]
	fn test_min_heap() -> Result<(), ()> {
		let mut heap = MinHeap::new();
		let mut result = Vec::new();
		heap.insert_key(8)
			.insert_key(-100)
			.insert_key(-120)
			.insert_key(7)
			.insert_key(0)
			.insert_key(8)
			.insert_key(4);
		while !heap.empty() {
			result.push(heap.extract_min()?)
		}
		assert_eq!(result, vec![-120, -100, 0, 4, 7, 8, 8]);
		Ok(())
	}
}
