// https://leetcode.com/problems/minimum-cost-for-cutting-cake-ii/
// 3219. Minimum Cost for Cutting a Cake II
pub struct Solution;
impl Solution {
    pub fn minimum_cost(_m: i32, _n: i32, horizontal_cut: Vec<i32>, vertical_cut: Vec<i32>) -> i64 {
        let mut cut = horizontal_cut
            .into_iter()
            .zip(std::iter::repeat(0))
            .chain(vertical_cut.into_iter().zip(std::iter::repeat(1)))
            .collect::<Vec<_>>();
        cut.sort_unstable();
        let mut cnt = vec![1, 1];
        let mut sum = 0;
        for (c, t) in cut.into_iter().rev() {
            cnt[t] += 1;
            sum += cnt[1 - t] as i64 * c as i64;
        }
        sum
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
