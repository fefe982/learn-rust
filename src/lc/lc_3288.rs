// https://leetcode.com/problems/length-of-the-longest-increasing-path/
// 3288. Length of the Longest Increasing Path in a Matrix
pub struct Solution;
impl Solution {
    pub fn max_path_length(coordinates: Vec<Vec<i32>>, k: i32) -> i32 {
        let mut res = 0;
        let pos = (coordinates[k as usize][0], coordinates[k as usize][1]);
        let mut coordinates = coordinates;
        coordinates.sort_by_key(|v| (v[0], -v[1]));
        let mut q = vec![];
        for c in coordinates {
            if (c[0] < pos.0 && c[1] < pos.1) || (c[0] > pos.0 && c[1] > pos.1) {
                let p = q.partition_point(|&x| x < c[1]);
                if p == q.len() {
                    q.push(c[1]);
                } else {
                    q[p] = c[1];
                }
            } else if c[0] == pos.0 && c[1] == pos.1 {
                res = q.len() as i32 + 1;
                q.clear();
            }
        }
        res + q.len() as i32
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn test_max_path_length() {
        assert_eq!(
            Solution::max_path_length(vec_vec![[3, 1], [2, 2], [4, 1], [0, 0], [5, 3]], 1),
            3
        );
        assert_eq!(Solution::max_path_length(vec_vec![[2, 1], [7, 0], [5, 6]], 2), 2)
    }
}
