// https://leetcode.com/problems/describe-the-painting/
// 1943. Describe the Painting
pub struct Solution;
impl Solution {
    pub fn split_painting(segments: Vec<Vec<i32>>) -> Vec<Vec<i64>> {
        let mut events = std::collections::BTreeMap::new();
        for segment in segments {
            let start = segment[0] as i64;
            let end = segment[1] as i64;
            let color = segment[2] as i64;
            *events.entry(start).or_insert(0) += color;
            *events.entry(end).or_insert(0) -= color;
        }
        let mut ans = Vec::new();
        let mut prev = 0;
        let mut color = 0;
        for (pos, delta) in events {
            if color > 0 {
                ans.push(vec![prev, pos, color]);
            }
            color += delta;
            prev = pos;
        }
        ans
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn test_split_painting() {
        assert_eq!(
            Solution::split_painting(vec_vec![[1, 4, 5], [4, 7, 7], [1, 7, 9]]),
            vec_vec![[1, 4, 14], [4, 7, 16]]
        );
        assert_eq!(
            Solution::split_painting(vec_vec![[1, 7, 9], [6, 8, 15], [8, 10, 7]]),
            vec_vec![[1, 6, 9], [6, 7, 24], [7, 8, 15], [8, 10, 7]]
        );
        assert_eq!(
            Solution::split_painting(vec_vec![[1, 4, 5], [1, 4, 7], [4, 7, 1], [4, 7, 11]]),
            vec_vec![[1, 4, 12], [4, 7, 12]]
        );
    }
}
