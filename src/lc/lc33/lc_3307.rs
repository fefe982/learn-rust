// https://leetcode.com/problems/find-the-k-th-character-in-string-game-ii/
// 3307. Find the K-th Character in String Game II
pub struct Solution;
impl Solution {
    pub fn kth_character(k: i64, operations: Vec<i32>) -> char {
        let mut k = k - 1;
        let mut add = 0;
        let mut i = 0;
        while k > 0 {
            if k & 1 == 1 {
                add += operations[i];
            }
            k >>= 1;
            i += 1;
        }
        (b'a' + (add % 26) as u8) as char
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_kth_character() {
        assert_eq!(Solution::kth_character(5, vec![0, 0, 0]), 'a');
        assert_eq!(Solution::kth_character(10, vec![0, 1, 0, 1]), 'b');
    }
}
