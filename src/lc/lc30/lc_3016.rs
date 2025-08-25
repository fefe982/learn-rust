// https://leetcode.com/problems/minimum-number-of-pushes-to-type-word-ii/
// 3016. Minimum Number of Pushes to Type a Word II
pub struct Solution;
impl Solution {
    pub fn minimum_pushes(word: String) -> i32 {
        let mut cnt = vec![0; 26];
        for c in word.chars() {
            cnt[(c as u8 - b'a') as usize] += 1;
        }
        cnt.sort_unstable();
        cnt.into_iter()
            .rev()
            .enumerate()
            .fold(0, |acc, (i, c)| acc + c * (i as i32 / 8 + 1))
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_minimum_pushes() {
        assert_eq!(Solution::minimum_pushes("abcde".to_string()), 5);
        assert_eq!(Solution::minimum_pushes("xyzxyzxyzxyz".to_string()), 12);
        assert_eq!(Solution::minimum_pushes("aabbccddeeffgghhiiiiii".to_string()), 24);
    }
}
