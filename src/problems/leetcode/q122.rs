// 121. Best Time to Buy and Sell Stock
struct Solution;

impl Solution {
  // O(N) solution
  pub fn max_profit(mut prices: Vec<i32>) -> i32 {
    let mut min_price = prices[0];
    let mut profit = 0;
    for i in 1..prices.len() {
      // Sell the stock
      if prices[i] > min_price {
        profit += prices[i] - min_price;
        min_price = prices[i];
      }
      // Buy the stock
      if prices[i] < min_price {
        min_price = prices[i];
      }
    }
    return profit;
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
    assert_eq!(n, 7);

    let mut nums = Vec::from([1, 2, 3, 4, 5]);
    let n = Solution::max_profit(nums);
    println!("#2 n={:?}", n);
    assert_eq!(n, 4);

    let mut nums = Vec::from([7, 6, 4, 3, 1]);
    let n = Solution::max_profit(nums);
    println!("#3 n={:?}", n);
    assert_eq!(n, 0);
  }
}
