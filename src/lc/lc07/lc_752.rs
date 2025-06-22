// https://leetcode.com/problems/open-the-lock/
// 752. Open the Lock
pub struct Solution;
impl Solution {
    pub fn open_lock(deadends: Vec<String>, target: String) -> i32 {
        let mut v = vec![vec![vec![vec![false; 10]; 10]; 10]; 10];
        fn parse(str: String) -> Vec<usize> {
            str.chars().map(|c| c as usize - '0' as usize).collect::<Vec<_>>()
        }
        let target = parse(target);
        if target == vec![0; 4] {
            return 0;
        }
        for deadend in deadends {
            let n = parse(deadend);
            v[n[0]][n[1]][n[2]][n[3]] = true;
        }
        if v[0][0][0][0] {
            return -1;
        }
        v[0][0][0][0] = true;
        let mut q = std::collections::VecDeque::<(Vec<usize>, i32)>::new();
        q.push_back((vec![0; 4], 0));
        while let Some((mut c, step)) = q.pop_front() {
            for i in 0..4 {
                c[i] = (c[i] + 1) % 10;
                if c == target {
                    return step + 1;
                }
                if !v[c[0]][c[1]][c[2]][c[3]] {
                    v[c[0]][c[1]][c[2]][c[3]] = true;
                    q.push_back((c.clone(), step + 1));
                }
                c[i] = (c[i] + 8) % 10;
                if c == target {
                    return step + 1;
                }
                if !v[c[0]][c[1]][c[2]][c[3]] {
                    v[c[0]][c[1]][c[2]][c[3]] = true;
                    q.push_back((c.clone(), step + 1));
                }
                c[i] = (c[i] + 1) % 10;
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
    fn test_open_lock() {
        assert_eq!(
            Solution::open_lock(vec_str!["0201", "0101", "0102", "1212", "2002"], "0000".to_string()),
            0
        );
        assert_eq!(Solution::open_lock(vec_str!["0000"], "8888".to_string()), -1);
        assert_eq!(
            Solution::open_lock(vec_str!["0201", "0101", "0102", "1212", "2002"], "0202".to_string()),
            6
        );
        assert_eq!(Solution::open_lock(vec_str!["8888"], "0009".to_string()), 1);
        assert_eq!(
            Solution::open_lock(
                vec_str!["8887", "8889", "8878", "8898", "8788", "8988", "7888", "9888"],
                "8888".to_string()
            ),
            -1
        );
    }
}
