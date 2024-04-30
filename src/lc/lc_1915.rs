// https://leetcode.com/problems/number-of-wonderful-substrings/
// 1915. Number of Wonderful Substrings
pub struct Solution;
impl Solution {
    pub fn wonderful_substrings(word: String) -> i64 {
        let mut cnt = vec![0i64; ((1 << 10) - 1) + 1];
        cnt[0] = 1;
        let mut prefix = 0;
        let mut total = 0;
        for c in word.chars() {
            prefix ^= 1 << (c as u8 - b'a');
            total += cnt[prefix];
            for i in 0..10 {
                total += cnt[prefix ^ (1 << i)];
            }
            cnt[prefix] += 1;
        }
        total
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_wonderful_substrings() {
        assert_eq!(Solution::wonderful_substrings("aba".to_string()), 4);
        assert_eq!(Solution::wonderful_substrings("aabb".to_string()), 9);
        assert_eq!(Solution::wonderful_substrings("he".to_string()), 2);
    }
}
