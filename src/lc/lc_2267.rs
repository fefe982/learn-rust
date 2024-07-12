// https://leetcode.com/problems/check-if-there-is-a-valid-parentheses-string-path/
// 2267. Check if There Is a Valid Parentheses String Path
pub struct Solution;
impl Solution {
    pub fn has_valid_path(grid: Vec<Vec<char>>) -> bool {
        let m = grid.len();
        let n = grid[0].len();
        if (m + n) % 2 == 0 {
            return false;
        }
        if grid[0][0] == ')' || grid[m - 1][n - 1] == '(' {
            return false;
        }
        let mut v = vec![std::collections::HashSet::new(); n];
        v[0].insert(1);
        let mut max = (m + n - 2) as i32;
        let mut line_max = max;
        for j in 1..n {
            line_max = line_max - 1;
            if let Some(&c) = v[j - 1].iter().next() {
                if grid[0][j] == '(' {
                    if c + 1 <= line_max {
                        v[j].insert(c + 1);
                    } else {
                        break;
                    }
                } else if c > 0 {
                    v[j].insert(c - 1);
                } else {
                    break;
                }
            }
        }
        for i in 1..m {
            max -= 1;
            line_max = max;
            for j in 0..n {
                let mut m = std::collections::HashSet::new();
                for &c in &v[j] {
                    if grid[i][j] == '(' {
                        if c + 1 <= line_max {
                            m.insert(c + 1);
                        }
                    } else if c > 0 {
                        m.insert(c - 1);
                    }
                }
                if j > 0 {
                    for &c in &v[j - 1] {
                        if grid[i][j] == '(' {
                            if c + 1 <= line_max {
                                m.insert(c + 1);
                            }
                        } else if c > 0 {
                            m.insert(c - 1);
                        }
                    }
                }
                v[j] = m;
                line_max -= 1;
            }
        }
        v[n - 1].contains(&0)
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn test_has_valid_path() {
        assert_eq!(
            Solution::has_valid_path(vec_vec_chr![
                ["(", "(", "("],
                [")", "(", ")"],
                ["(", "(", ")"],
                ["(", "(", ")"]
            ]),
            true
        );
        assert_eq!(Solution::has_valid_path(vec_vec_chr![[")", ")"], ["(", "("]]), false);
    }
}
