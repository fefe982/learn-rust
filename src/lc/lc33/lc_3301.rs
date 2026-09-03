// https://leetcode.com/problems/maximize-the-total-height-of-unique-towers/
// 3301. Maximize the Total Height of Unique Towers
pub struct Solution;
impl Solution {
    pub fn maximum_total_sum(maximum_height: Vec<i32>) -> i64 {
        let mut maximum_height = maximum_height;
        maximum_height.sort_unstable();
        let mut sum = maximum_height[maximum_height.len() - 1] as i64;
        for i in (1..maximum_height.len()).rev() {
            let h = maximum_height[i - 1].min(maximum_height[i] - 1);
            if h <= 0 {
                return -1;
            }
            sum += h as i64;
            maximum_height[i - 1] = h;
        }
        sum
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn maximum_total_sum() {
        assert_eq!(Solution::maximum_total_sum(vec![2, 3, 4, 3]), 10);
        assert_eq!(Solution::maximum_total_sum(vec![15, 10]), 25);
        assert_eq!(Solution::maximum_total_sum(vec![2, 2, 1]), -1);
    }
}
