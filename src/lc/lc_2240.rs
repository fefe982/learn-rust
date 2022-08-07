// https://leetcode.com/problems/number-of-ways-to-buy-pens-and-pencils/
// 2240. Number of Ways to Buy Pens and Pencils

pub struct Solution;
impl Solution {
    pub fn ways_to_buy_pens_pencils(total: i32, cost1: i32, cost2: i32) -> i64 {
        let mut sum: i64 = 0;
        let big = std::cmp::max(cost1, cost2);
        let small = std::cmp::min(cost1, cost2);
        for i in 0..=total / big {
            sum += ((total - big * i) / small + 1) as i64
        }
        sum
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn ways_to_buy_pens_pencils() {
        assert_eq!(Solution::ways_to_buy_pens_pencils(20, 10, 5), 9);
        assert_eq!(Solution::ways_to_buy_pens_pencils(5, 10, 10), 1);
    }
}
