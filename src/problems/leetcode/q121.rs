// 121. Best Time to Buy and Sell Stock
struct Solution;

impl Solution {
  // Naive solution
  pub fn max_profit_v1(prices: Vec<i32>) -> i32 {
    let len = prices.len();
    let mut max = 0;
    for i in 0..len {
      for j in i..len {
        if prices[j] - prices[i] > max {
          max = prices[j] - prices[i];
        }
      }
    }
    return max;
  }

  // O(N) solution
  pub fn max_profit(mut prices: Vec<i32>) -> i32 {
    let mut max = 0;
    let mut min_price = prices[0];
    for i in prices {
      max = if i - min_price > max { i - min_price } else { max };
      min_price = if i < min_price { i } else { min_price };
    }

    return max;
  }
}

#[cfg(test)]
mod tests {
  use super::Solution;

  #[test]
  fn test() {
    let mut nums = Vec::from([7, 1, 5, 3, 6, 4]);
    let n = Solution::max_profit(nums);
    println!("#1 n={:?}", n);
    assert_eq!(n, 5);

    let mut nums = Vec::from([7, 6, 4, 3, 1]);
    let n = Solution::max_profit(nums);
    println!("#2 n={:?}", n);
    assert_eq!(n, 0);
  }
}
