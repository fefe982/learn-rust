// https://leetcode.com/problems/largest-combination-with-bitwise-and-greater-than-zero/
// 2275. Largest Combination With Bitwise AND Greater Than Zero
pub struct Solution;
impl Solution {
    pub fn largest_combination(candidates: Vec<i32>) -> i32 {
        let mut cnt = vec![0; 24];
        for c in candidates {
            for i in 0..24 {
                if c & (1 << i) > 0 {
                    cnt[i] += 1;
                }
            }
        }
        cnt.into_iter().max().unwrap_or(0)
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_largest_combination() {
        assert_eq!(Solution::largest_combination(vec![16, 17, 71, 62, 12, 24, 14]), 4);
        assert_eq!(Solution::largest_combination(vec![8, 8]), 2);
    }
}
