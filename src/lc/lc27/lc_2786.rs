// https://leetcode.com/problems/visit-array-positions-to-maximize-score/
// 2786. Visit Array Positions to Maximize Score
pub struct Solution;
impl Solution {
    pub fn max_score(nums: Vec<i32>, x: i32) -> i64 {
        let mut max = nums[0] as i64;
        let x = x as i64;
        let fill = i64::MIN + x;
        let mut pre = vec![fill, fill];
        pre[max as usize % 2] = max;
        for n in nums.into_iter().skip(1) {
            let n = n as i64;
            let parity = (n % 2) as usize;
            let add = pre[parity].max(pre[1 - parity] - x);
            let cur = n + add;
            max = max.max(cur);
            pre[parity] = pre[parity].max(cur);
        }
        max
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_max_score() {
        assert_eq!(Solution::max_score(vec![85, 12], 79), 85);
        assert_eq!(
            Solution::max_score(vec![8, 50, 65, 85, 8, 73, 55, 50, 29, 95, 5, 68, 52, 79], 74),
            470
        );
        assert_eq!(Solution::max_score(vec![2, 3, 6, 1, 9, 2], 5), 13);
        assert_eq!(Solution::max_score(vec![2, 4, 6, 8], 3), 20);
    }
}
