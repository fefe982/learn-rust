// https://leetcode.com/problems/count-number-of-special-subsequences/
// 1955. Count Number of Special Subsequences
pub struct Solution;
impl Solution {
    pub fn count_special_subsequences(nums: Vec<i32>) -> i32 {
        let mut n0 = 0;
        let mut n1 = 0;
        let mut n2 = 0;
        let m = 1_0000_0000_7i64;
        for n in nums {
            if n == 0 {
                n0 = (n0 * 2 + 1) % m;
            } else if n == 1 {
                n1 = (n1 * 2 + n0) % m;
            } else {
                n2 = (n2 * 2 + n1) % m;
            }
        }
        n2 as i32
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_count_special_subsequences() {
        assert_eq!(Solution::count_special_subsequences(vec![0, 1, 2, 2]), 3);
        assert_eq!(Solution::count_special_subsequences(vec![2, 2, 0, 0]), 0);
        assert_eq!(Solution::count_special_subsequences(vec![0, 1, 2, 0, 1, 2]), 7);
    }
}
