// https://leetcode.cn/problems/PLYXKQ/
// LCR 040. 最大矩形
pub struct Solution;
impl Solution {
    pub fn maximal_rectangle(matrix: Vec<String>) -> i32 {
        if matrix.is_empty() {
            return 0;
        }
        let mut heights = vec![0; matrix[0].len()];
        let mut max_area = 0;
        for line in matrix {
            for (idx, c) in line.chars().enumerate() {
                if c == '0' {
                    heights[idx] = 0;
                } else {
                    heights[idx] += 1;
                }
            }
            let mut stack = vec![(0, 0)];
            for (&h, idx) in heights.iter().chain([0].iter()).zip(1..) {
                let mut start = idx;
                while stack.last().unwrap().0 > h {
                    let last = stack.pop().unwrap();
                    max_area = std::cmp::max(max_area, last.0 * (idx - last.1));
                    start = last.1
                }
                if stack.last().unwrap().0 < h {
                    stack.push((h, start));
                }
            }
        }
        max_area
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn maximal_rectangle() {
        assert_eq!(
            Solution::maximal_rectangle(vec_str!["10100", "10111", "11111", "10010"]),
            6
        );
        assert_eq!(Solution::maximal_rectangle(vec_str![]), 0);
        assert_eq!(Solution::maximal_rectangle(vec_str!["0"]), 0);
        assert_eq!(Solution::maximal_rectangle(vec_str!["1"]), 1);
        assert_eq!(Solution::maximal_rectangle(vec_str!["00"]), 0);
    }
}
