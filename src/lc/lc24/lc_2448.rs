// https://leetcode.com/problems/minimum-cost-to-make-array-equal/
// 2448. Minimum Cost to Make Array Equal
pub struct Solution;
impl Solution {
    pub fn min_cost(nums: Vec<i32>, cost: Vec<i32>) -> i64 {
        let cost_all = cost.iter().fold(0i64, |x, &y| x + y as i64);
        let mut num_cost = nums
            .into_iter()
            .zip(cost.into_iter())
            .collect::<Vec<(i32, i32)>>();
        num_cost.sort_unstable_by_key(|&(x, _)| x);
        let mut cost_l = 0i64;
        let mut il = 0;
        let mut ie = 0;
        while cost_all - cost_l > cost_l {
            il = ie;
            while ie < num_cost.len() && num_cost[ie].0 == num_cost[il].0 {
                cost_l += num_cost[ie].1 as i64;
                ie += 1;
            }
        }
        let v = num_cost[il].0;
        num_cost
            .into_iter()
            .fold(0i64, |s, (n, c)| s + (n - v).abs() as i64 * c as i64)
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn min_cost() {
        assert_eq!(Solution::min_cost(vec![1, 3, 5, 2], vec![2, 3, 1, 14]), 8);
        assert_eq!(
            Solution::min_cost(vec![2, 2, 2, 2, 2], vec![4, 2, 8, 1, 3]),
            0
        );
    }
}
