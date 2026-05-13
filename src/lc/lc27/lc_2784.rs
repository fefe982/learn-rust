// https://leetcode.cn/problems/check-if-array-is-good/
// 2784. Check if Array Is Good
pub struct Solution;
impl Solution {
    pub fn is_good(nums: Vec<i32>) -> bool {
        let n = nums.len();
        if n < 2 {
            return false;
        }
        let mut cnt = vec![0; n];
        for &x in &nums {
            if x >= n as i32 {
                return false;
            }
            if x <= 0 {
                return false;
            }
            if x < n as i32 - 1 {
                if cnt[x as usize] > 0 {
                    return false;
                }
            } else {
                if cnt[n - 1] > 1 {
                    return false;
                }
            }
            cnt[x as usize] += 1;
        }
        true
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_is_good() {
        assert_eq!(Solution::is_good(vec![2, 1, 3]), false);
        assert_eq!(Solution::is_good(vec![1, 3, 3, 1]), true);
        assert_eq!(Solution::is_good(vec![1, 1]), true);
        assert_eq!(Solution::is_good(vec![3, 4, 4, 1, 2, 1]), false);
    }
}
