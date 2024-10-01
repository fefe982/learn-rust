// https://leetcode.com/problems/check-if-array-pairs-are-divisible-by-k/
// 1497. Check If Array Pairs Are Divisible by K
pub struct Solution;
impl Solution {
    pub fn can_arrange(arr: Vec<i32>, k: i32) -> bool {
        let mut cnt = vec![0; k as usize];
        for n in arr {
            cnt[((n % k + k) % k) as usize] += 1;
        }
        if cnt[0] % 2 != 0 {
            return false;
        }
        for i in 1..((k as usize + 1) / 2) {
            if cnt[i] != cnt[k as usize - i] {
                return false;
            }
        }
        true
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_can_arrange() {
        assert_eq!(Solution::can_arrange(vec![-1, -1, -1, -1, 2, 2, -2, -2], 3), false);
        assert_eq!(Solution::can_arrange(vec![1, 2, 3, 4, 5, 10, 6, 7, 8, 9], 5), true);
        assert_eq!(Solution::can_arrange(vec![1, 2, 3, 4, 5, 6], 7), true);
        assert_eq!(Solution::can_arrange(vec![1, 2, 3, 4, 5, 6], 10), false);
    }
}
