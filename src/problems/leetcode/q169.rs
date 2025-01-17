// 169. Majority Element
struct Solution;

impl Solution {
  // Boyer–Moore majority vote algorithm
  pub fn majority_element(nums: Vec<i32>) -> i32 {
    let mut target = nums[0];
    let mut count = 1;
    // First pass: Find majority
    for i in 1..nums.len() {
      if count == 0 {
        target = nums[i];
        count = 1;
      } else if target == nums[i] {
        count += 1;
      } else {
        count -= 1;
      }
    }

    // Second pass: Verify majority (if not exists)
    // count = 0;
    // for i in 0..nums.len() {
    //   if nums[i] == target {
    //     count += 1;
    //   }
    // }
    // if count < nums.len() / 2 {
    //   // No result 
    // }

    return target;
  }

  // Sort the input then use the value in the middle
  pub fn majority_element_v1(mut nums: Vec<i32>) -> i32 {
    // Time Complexity: O(nlgn)
    // Space Complexity: O(n)
    nums.sort();
    return nums[nums.len() / 2];
  }

  // Use hashmap to record the occurrence
  pub fn majority_element_v2(nums: Vec<i32>) -> i32 {
    // Time Complexity: O(n)
    // Space Complexity: O(k) where k is the number of unique elements
    use std::collections::HashMap;
    let mut counter = HashMap::<i32, usize>::new();
    let mut target = nums[0];
    let mut max_count = 0;
    for k in 0..nums.len() {
      counter.entry(nums[k])
        .and_modify(|c| {
          *c += 1;
          if *c > max_count {
            max_count = *c;
            target = nums[k];
          }
        })
        .or_insert(1);
    }
    return target;
  }
}

#[cfg(test)]
mod tests {
  use super::Solution;

	#[test]
	fn test() {
    let nums = Vec::from([3, 2, 3]);
    let n = Solution::majority_element(nums);
    println!("#1 n={:?}", n);
    assert_eq!(n, 3);

    let nums = Vec::from([3, 2, 3]);
    let n = Solution::majority_element_v1(nums);
    println!("#1 n={:?}", n);
    assert_eq!(n, 3);

    let nums = Vec::from([3, 2, 3]);
    let n = Solution::majority_element_v2(nums);
    println!("#1 n={:?}", n);
    assert_eq!(n, 3);

    let nums = Vec::from([2, 2, 1, 1, 1, 2, 2]);
    let n = Solution::majority_element(nums);
    println!("#1 n={:?}", n);
    assert_eq!(n, 2);

    let nums = Vec::from([2, 2, 1, 1, 1, 2, 2]);
    let n = Solution::majority_element_v1(nums);
    println!("#1 n={:?}", n);
    assert_eq!(n, 2);

    let nums = Vec::from([2, 2, 1, 1, 1, 2, 2]);
    let n = Solution::majority_element_v2(nums);
    println!("#1 n={:?}", n);
    assert_eq!(n, 2);
    
	}
}