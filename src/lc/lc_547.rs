// https://leetcode.com/problems/number-of-provinces/
// 547. Number of Provinces
pub struct Solution;
impl Solution {
    pub fn find_circle_num(is_connected: Vec<Vec<i32>>) -> i32 {
        let mut n = 0;
        let l = is_connected.len();
        let mut visited = vec![false; l];
        for i in 0..l {
            if !visited[i] {
                n += 1;
                let mut q = vec![i];
                while let Some(c) = q.pop() {
                    if visited[c] {
                        continue;
                    }
                    visited[c] = true;
                    for j in 0..l {
                        if !visited[j] && is_connected[c][j] > 0 {
                            q.push(j);
                        }
                    }
                }
            }
        }
        n
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn find_circle_num() {
        assert_eq!(
            Solution::find_circle_num(vec_vec![[1, 1, 0], [1, 1, 0], [0, 0, 1]]),
            2
        );
        assert_eq!(
            Solution::find_circle_num(vec_vec![[1, 0, 0], [0, 1, 0], [0, 0, 1]]),
            3
        );
    }
}
