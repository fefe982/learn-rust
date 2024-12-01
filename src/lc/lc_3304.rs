// https://leetcode.com/problems/find-the-k-th-character-in-string-game-i/
// 3304. Find the K-th Character in String Game I
pub struct Solution;
impl Solution {
    pub fn kth_character(k: i32) -> char {
        (b'a' + ((k - 1).count_ones() % 26) as u8) as char
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn kth_character() {
        assert_eq!(Solution::kth_character(5), 'b');
        assert_eq!(Solution::kth_character(10), 'c');
    }
}
