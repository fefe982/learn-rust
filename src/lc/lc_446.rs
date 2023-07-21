// https://leetcode.com/problems/arithmetic-slices-ii-subsequence/
// 446. Arithmetic Slices II - Subsequence
pub struct Solution;
impl Solution {
    pub fn number_of_arithmetic_slices(nums: Vec<i32>) -> i32 {
        let mut s = vec![std::collections::HashMap::<i64, i32>::new(); nums.len()];
        let mut tot = 0;
        for i in 1..nums.len() {
            let ni = nums[i];
            for j in 0..i {
                let diff = ni as i64 - nums[j] as i64;
                *s[i].entry(diff).or_default() += 1;
                if let Some(&c) = s[j].get(&diff) {
                    tot += c;
                    *s[i].entry(diff).or_default() += c;
                }
            }
        }
        tot
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn number_of_arithmetic_slices() {
        assert_eq!(
            Solution::number_of_arithmetic_slices(vec![0, 2000000000, -294967296]),
            0
        );
        assert_eq!(
            Solution::number_of_arithmetic_slices(vec![2, 4, 6, 8, 10]),
            7
        );
        assert_eq!(
            Solution::number_of_arithmetic_slices(vec![7, 7, 7, 7, 7]),
            16
        );
    }
}
