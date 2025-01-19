// 27. Remove Element

struct Solution;

impl Solution {
  pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
    // target index
    let mut i: usize = 0;
    
    for j in 0..nums.len() {
      if nums[j] != val {
        nums[i] = nums[j];
        i += 1;
      }
    }
    return i as i32;
  }
}


#[cfg(test)]
mod tests {
  use super::Solution;

	#[test]
	fn test() {
    let mut nums1 = Vec::from([3, 2, 2, 3]);
    let val = 3;
    let n = Solution::remove_element(&mut nums1, val);
    println!("#1 nums1={:?}, n={:?}", nums1, n);
    nums1.truncate(n as usize);
    assert_eq!(&nums1[..], &[2, 2]);

    let mut nums1 = Vec::from([0, 1, 2, 2, 3, 0, 4, 2]);
    let val = 2;
    let n = Solution::remove_element(&mut nums1, val);
    println!("#2 nums1={:?}, n={:?}", nums1, n);
    nums1.truncate(n as usize);
    assert_eq!(&nums1[..], &[0, 1, 3, 0, 4]);
	}
}