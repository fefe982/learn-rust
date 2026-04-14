// https://leetcode.com/problems/shortest-distance-to-target-string-in-a-circular-array/
// 2515. Shortest Distance to Target String in a Circular Array
pub struct Solution;
impl Solution {
    pub fn closest_target(words: Vec<String>, target: String, start_index: i32) -> i32 {
        let n = words.len();
        let start_index = start_index as usize;
        for i in 0..n / 2 + 1 {
            if words[(start_index + i) % n] == target || words[(start_index + n - i) % n] == target {
                return i as i32;
            }
        }
        -1
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn closest_target() {
        assert_eq!(
            Solution::closest_target(
                vec_str!["hello", "i", "am", "leetcode", "hello"],
                "hello".to_string(),
                1
            ),
            1
        );
        assert_eq!(
            Solution::closest_target(vec_str!["a", "b", "leetcode"], "leetcode".to_string(), 0),
            1
        );
        assert_eq!(
            Solution::closest_target(vec_str!["i", "eat", "leetcode"], "ate".to_string(), 0),
            -1
        );
    }
}
