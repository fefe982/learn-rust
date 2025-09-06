// https://leetcode.com/problems/k-concatenation-maximum-sum/
// 1191. K-Concatenation Maximum Sum
pub struct Solution;
impl Solution {
    pub fn k_concatenation_max_sum(arr: Vec<i32>, k: i32) -> i32 {
        let mut min = 0;
        let mut max = 0;
        let mut sum = 0;
        let mut d = 0;
        for a in arr {
            sum += a as i64;
            d = d.max(sum - min);
            min = min.min(sum);
            max = max.max(sum);
        }
        (if k == 1 {
            d
        } else if sum <= 0 {
            d.max(max + sum - min)
        } else {
            max + sum - min + (k - 2) as i64 * sum
        } % 1_0000_0000_7) as i32
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn k_concatenation_max_sum() {
        assert_eq!(
            Solution::k_concatenation_max_sum(
                vec![10000, 10000, 10000, 10000, 10000, 10000, 10000, 10000, 10000, 10000],
                100000
            ),
            999999937
        );
        assert_eq!(Solution::k_concatenation_max_sum(vec![1, 2], 3), 9);
        assert_eq!(Solution::k_concatenation_max_sum(vec![1, -2, 1], 5), 2);
        assert_eq!(Solution::k_concatenation_max_sum(vec![-1, -2], 7), 0);
    }
}
