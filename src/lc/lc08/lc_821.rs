// https://leetcode.com/problems/shortest-distance-to-a-character/
// 821. Shortest Distance to a Character
pub struct Solution;
impl Solution {
    pub fn shortest_to_char(s: String, c: char) -> Vec<i32> {
        let len = s.len();
        let mut result = vec![i32::MAX; len];
        let mut prev = usize::MAX;
        for (i, ch) in s.chars().enumerate() {
            if ch == c {
                prev = i;
                result[i] = 0;
            } else if prev != usize::MAX {
                result[i] = (i - prev) as i32;
            }
        }
        prev = usize::MAX;
        for (i, ch) in s.chars().rev().enumerate() {
            if ch == c {
                prev = i;
            } else if prev != usize::MAX {
                result[len - 1 - i] = result[len - 1 - i].min((i - prev) as i32);
            }
        }
        result
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn shortest_to_char() {
        assert_eq!(
            Solution::shortest_to_char("loveleetcode".to_string(), 'e'),
            [3, 2, 1, 0, 1, 0, 0, 1, 2, 2, 1, 0]
        );
        assert_eq!(Solution::shortest_to_char("aaab".to_string(), 'b'), [3, 2, 1, 0]);
    }
}
