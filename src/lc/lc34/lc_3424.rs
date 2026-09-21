// https://leetcode.com/problems/minimum-cost-to-make-arrays-identical/
// 3424. Minimum Cost to Make Arrays Identical
pub struct Solution;
impl Solution {
    pub fn min_cost(arr: Vec<i32>, brr: Vec<i32>, k: i64) -> i64 {
        let mut sum1 = 0;
        for (&a, &b) in arr.iter().zip(brr.iter()) {
            sum1 += (a - b).abs() as i64;
        }
        let mut arr = arr;
        let mut brr = brr;
        arr.sort();
        brr.sort();
        let mut sum2 = 0;
        for (&a, &b) in arr.iter().zip(brr.iter()) {
            sum2 += (a - b).abs() as i64;
        }
        sum1.min(sum2 + k)
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn min_cost() {
        assert_eq!(Solution::min_cost(vec![-7, 9, 5], vec![7, -2, -5], 2), 13);
        assert_eq!(Solution::min_cost(vec![2, 1], vec![2, 1], 0), 0);
    }
}
