// 26. Remove Duplicates from Sorted Array
struct Solution;

impl Solution {
  pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
    let mut i: usize = 0;
    for k in 1..nums.len() {
      if nums[i] != nums[k] {
        i += 1;
        nums[i] = nums[k];
      }
    }
    return (i + 1) as i32;
  }
}

#[cfg(test)]
mod tests {
  use super::Solution;

	#[test]
	fn test() {
    let mut nums = Vec::from([1,1,2]);
    let n = Solution::remove_duplicates(&mut nums);
    println!("#1 nums={:?}, n={:?}", nums, n);
    assert_eq!(&nums[..2], &[1,2]);

    let mut nums = Vec::from([0, 0, 1, 1, 1, 2, 2, 3, 3, 4]);
    let n = Solution::remove_duplicates(&mut nums);
    println!("#1 nums={:?}, n={:?}", nums, n);
    assert_eq!(&nums[..5], &[0, 1, 2, 3 ,4]);
	}
}