// 45. Jump Game II
struct Solution;

impl Solution {
  pub fn __jump(nums: &Vec<i32>, checked: &mut Vec<i32>, start: usize, end: usize) -> i32 {
    // Check if can reach end
    if end >= nums.len() - 1 {
      return 1;
    }
    let mut min = std::i32::MAX;
    for i in start..end + 1 {
      if nums[i] == 0 {
        continue;
      } else {
        // Skip checked position
        if checked[i] != std::i32::MAX && checked[i] != -1 {
          min = if checked[i] < min { checked[i] + 1 } else { min }
        // Cache position
        } else {
          let ret = Solution::__jump(nums, checked, i + 1, i + (nums[i] as usize));
          // Cache result;
          checked[i] = ret;
          if ret != -1 {
            min = if checked[i] < min { checked[i] + 1 } else { min }
          }
        }
      }
    }
    return if min != std::i32::MAX { min } else { -1 }
  }

  pub fn jump(nums: Vec<i32>) -> i32 {
    let mut checked = vec![std::i32::MAX; nums.len()];
    if nums.len() <= 1 {
      return 0;
    }
    return Solution::__jump(&nums, &mut checked, 1, nums[0] as usize);
  }
}

#[cfg(test)]
mod tests {
  use super::Solution;

  #[test]
  fn test() {
    let mut nums = Vec::from([2, 3, 1, 1, 4]);
    let n = Solution::jump(nums);
    println!("#1 n={:?}", n);
    assert_eq!(n, 2);

    let mut nums = Vec::from([2, 3, 0, 1, 4]);
    let n = Solution::jump(nums);
    println!("#2 n={:?}", n);
    assert_eq!(n, 2);

    let mut nums = Vec::from([1, 2, 3]);
    let n = Solution::jump(nums);
    println!("#2 n={:?}", n);
    assert_eq!(n, 2);
  }
}