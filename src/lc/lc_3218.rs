// https://leetcode.com/problems/minimum-cost-for-cutting-cake-i/
// 3218. Minimum Cost for Cutting a Cake I
pub struct Solution;
impl Solution {
    pub fn minimum_cost(m: i32, n: i32, horizontal_cut: Vec<i32>, vertical_cut: Vec<i32>) -> i32 {
        super::lc_3219::Solution::minimum_cost(m, n, horizontal_cut, vertical_cut) as i32
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_minimum_cost() {
        assert_eq!(Solution::minimum_cost(3, 2, vec![1, 3], vec![5]), 13);
        assert_eq!(Solution::minimum_cost(2, 2, vec![7], vec![4]), 15);
    }
}
