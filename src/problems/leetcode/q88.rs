// 88. Merge Sorted Array
struct Solution;

impl Solution {
  pub fn merge(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
    // nums1 offset
    let mut i = 0;
    // nums2 offset
    let mut j = 0;
    let len = (m + n) as usize;
    let mut result = vec![0; len];
    for k in 0..len {
      if i >= m as usize {
        result[k] = nums2[j];
        j += 1;
      } else if j >= n as usize {
        result[k] = nums1[i];
        i += 1;
      } else if nums1[i] >= nums2[j] {
        result[k] = nums2[j as usize];
        j += 1;
      } else {
        result[k] = nums1[i as usize];
        i += 1;
      }
    }
    // Write the result to nums1
    for k in 0..len {
      nums1[k] = result[k];
    }
  }
}


#[cfg(test)]
mod tests {
  use super::Solution;

	#[test]
	fn test() {
    let mut nums1 = Vec::from([1, 2, 3, 0, 0, 0]);
    let mut nums2 = Vec::from([2, 5, 6]);
    let m = 3;
    let n = 3;
    Solution::merge(&mut nums1, m, &mut nums2, n);
    println!("#1 {:?}", nums1);
    assert_eq!(&nums1[..], &[1, 2, 2, 3, 5, 6]);

    let mut nums1 = Vec::from([1]);
    let mut nums2 = Vec::from([]);
    let m = 1;
    let n = 0;
    Solution::merge(&mut nums1, m, &mut nums2, n);
    println!("#2 {:?}", nums1);
    assert_eq!(&nums1[..], &[1]);

    let mut nums1 = Vec::from([0]);
    let mut nums2 = Vec::from([1]);
    let m = 0;
    let n = 1;
    Solution::merge(&mut nums1, m, &mut nums2, n);
    println!("#3 {:?}", nums1);
    assert_eq!(&nums1[..], &[1]);
	}
}