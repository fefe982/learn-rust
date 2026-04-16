// https://leetcode.com/problems/maximum-number-of-removable-characters/
// 1898. Maximum Number of Removable Characters
pub struct Solution;
impl Solution {
    pub fn maximum_removals(s: String, p: String, removable: Vec<i32>) -> i32 {
        let s = s.as_bytes();
        let p = p.as_bytes();
        let removable_len = removable.len();
        let mut removed_at = vec![usize::MAX; s.len()];

        for (step, index) in removable.into_iter().enumerate() {
            removed_at[index as usize] = step;
        }

        let mut left = 0usize;
        let mut right = removable_len;
        while left < right {
            let mid = left + (right - left).div_ceil(2);
            if Self::is_subsequence_after_removals(s, p, &removed_at, mid) {
                left = mid;
            } else {
                right = mid - 1;
            }
        }

        left as i32
    }

    fn is_subsequence_after_removals(s: &[u8], p: &[u8], removed_at: &[usize], removals: usize) -> bool {
        let mut p_index = 0usize;

        for (s_index, &ch) in s.iter().enumerate() {
            if removed_at[s_index] < removals {
                continue;
            }
            if p_index < p.len() && ch == p[p_index] {
                p_index += 1;
                if p_index == p.len() {
                    return true;
                }
            }
        }

        p_index == p.len()
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn maximum_removals() {
        assert_eq!(
            Solution::maximum_removals("abcacb".to_string(), "ab".to_string(), vec![3, 1, 0]),
            2
        );
        assert_eq!(
            Solution::maximum_removals("abcbddddd".to_string(), "abcd".to_string(), vec![3, 2, 1, 4, 5, 6]),
            1
        );
        assert_eq!(
            Solution::maximum_removals("abcab".to_string(), "abc".to_string(), vec![0, 1, 2, 3, 4]),
            0
        );
    }
}
