// 189. Rotate Array

use std::usize;

struct Solution;

impl Solution {
  // Version 1: O(N) + O(N)
  pub fn rotate(nums: &mut Vec<i32>, k: i32) {
    let len = nums.len();
    let pos = k as usize % len;
    let orig = nums.clone();
    for i in 0..len {
      nums[(pos + i) % len] = orig[i];
    }
  }

  // Version 2: O(N^2) + O(1)
  pub fn rotate_v2(nums: &mut Vec<i32>, k: i32) {
    let len = nums.len();
    let pos = k as usize % len;
    for _ in 0..pos {
      let saved = nums[len-1];
      for j in (0..len-1).rev() {
        nums[j+1] = nums[j];
      }
      nums[0] = saved;
    }
  }
}

#[cfg(test)]
mod tests {
  use super::Solution;

  #[test]
  fn test() {
    let mut nums = Vec::from([1, 2, 3, 4, 5, 6, 7]);
    let k = 3;
    Solution::rotate(&mut nums, k);
    println!("#1.1 nums={:?}", nums);
    assert_eq!(&nums[..], &[5, 6, 7, 1, 2, 3, 4]);

    let mut nums = Vec::from([1, 2, 3, 4, 5, 6, 7]);
    let k = 3;
    Solution::rotate_v2(&mut nums, k);
    println!("#2.1 nums={:?}", nums);
    assert_eq!(&nums[..], &[5, 6, 7, 1, 2, 3, 4]);

    let mut nums = Vec::from([-1, -100, 3, 99]);
    let k = 2;
    Solution::rotate(&mut nums, k);
    println!("#1.2 nums={:?}", nums);
    assert_eq!(&nums[..], &[3, 99, -1, -100]);

    let mut nums = Vec::from([-1, -100, 3, 99]);
    let k = 2;
    Solution::rotate_v2(&mut nums, k);
    println!("#2.2 nums={:?}", nums);
    assert_eq!(&nums[..], &[3, 99, -1, -100]);
  }
}
