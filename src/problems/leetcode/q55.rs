

// 121. Best Time to Buy and Sell Stock
struct Solution;

impl Solution {
  pub fn __can_jump(nums: &Vec<i32>, checked: &mut Vec<bool>, start: usize, end: usize) -> bool {
    for i in start..end + 1 {
      if (i as i32) + nums[i] >= (nums.len() as i32) - 1 {
        return true;
      } else if nums[i] == 0 {
        continue;
      } else {
        // Skip checked position
        if checked[i] == true { continue; }
        if Solution::__can_jump(nums, checked, i + 1, i + (nums[i] as usize)) == false {
          checked[i] = true;
          continue;
        } else { return true; }
      }
    }
    return false;
  }

  pub fn can_jump(nums: Vec<i32>) -> bool {
    let mut checked = vec![false; nums.len()];
    if nums.len() == 0 {
      return false;
    }
    return Solution::__can_jump(&nums, &mut checked, 0, nums[0] as usize);
  }
}

#[cfg(test)]
mod tests {
  use super::Solution;

  #[test]
  fn test() {
    let mut nums = Vec::from([2, 3, 1, 1, 4]);
    let n = Solution::can_jump(nums);
    println!("#1 n={:?}", n);
    assert_eq!(n, true);

    let mut nums = Vec::from([3, 2, 1, 0, 4]);
    let n = Solution::can_jump(nums);
    println!("#2 n={:?}", n);
    assert_eq!(n, false);

    let mut nums = Vec::from([1, 2, 3]);
    let n = Solution::can_jump(nums);
    println!("#3 n={:?}", n);
    assert_eq!(n, true);

    let mut nums = Vec::from([3, 0, 8, 2, 0, 0, 1]);
    let n = Solution::can_jump(nums);
    println!("#4 n={:?}", n);
    assert_eq!(n, true);

    let mut nums = Vec::from([0, 1]);
    let n = Solution::can_jump(nums);
    println!("#5 n={:?}", n);
    assert_eq!(n, false);

    let mut nums = Vec::from([5, 9, 3, 2, 1, 0, 2, 3, 3, 1, 0, 0]);
    let n = Solution::can_jump(nums);
    println!("#6 n={:?}", n);
    assert_eq!(n, true);
  }
}
