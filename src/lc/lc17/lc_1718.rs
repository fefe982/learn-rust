// https://leetcode.com/problems/construct-the-lexicographically-largest-valid-sequence/
// 1718. Construct the Lexicographically Largest Valid Sequence
pub struct Solution;
impl Solution {
    fn backtrack(res: &mut Vec<i32>, visited: &mut Vec<bool>, n: usize, index: usize) -> bool {
        if index == res.len() {
            return true;
        }
        if res[index] != 0 {
            return Solution::backtrack(res, visited, n, index + 1);
        } else {
            for i in (1..=n).rev() {
                if !visited[i] {
                    if i > 1 {
                        if index + i < res.len() && res[index + i] == 0 {
                            res[index] = i as i32;
                            res[index + i] = i as i32;
                        } else {
                            continue;
                        }
                    } else {
                        res[index] = i as i32;
                    }
                    visited[i] = true;
                    if Solution::backtrack(res, visited, n, index + 1) {
                        return true;
                    }
                    res[index] = 0;
                    if i > 1 {
                        res[index + i] = 0;
                    }
                    visited[i] = false;
                }
            }
        }
        false
    }
    pub fn construct_distanced_sequence(n: i32) -> Vec<i32> {
        let n = n as usize;
        let mut res = vec![0; 2 * n - 1];
        let mut visited = vec![false; n + 1];
        Solution::backtrack(&mut res, &mut visited, n, 0);
        res
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_construct_distanced_sequence() {
        assert_eq!(Solution::construct_distanced_sequence(3), vec![3, 1, 2, 3, 2]);
        assert_eq!(
            Solution::construct_distanced_sequence(5),
            vec![5, 3, 1, 4, 3, 5, 2, 4, 2]
        );
    }
}
