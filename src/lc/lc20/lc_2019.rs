// https://leetcode.com/problems/the-score-of-students-solving-math-expression/
// 2019. The Score of Students Solving Math Expression
pub struct Solution;
impl Solution {
    pub fn score_of_students(s: String, answers: Vec<i32>) -> i32 {
        let s = s.as_bytes();
        let nop = s.len() / 2;
        let mut dp = vec![vec![std::collections::HashSet::new(); nop + 1]; nop + 1];
        for i in 0..=nop {
            dp[i][i].insert((s[i * 2] - b'0') as i32);
        }
        for i in 1..=nop {
            for j in 0..=nop - i {
                let mut res = std::collections::HashSet::new();
                for k in j..j + i {
                    let l = &dp[j][k];
                    let r = &dp[k + 1][j + i];
                    for ll in l {
                        for rr in r {
                            if s[k * 2 + 1] == b'+' {
                                if ll + rr <= 10000 {
                                    res.insert(ll + rr);
                                }
                            } else {
                                if ll * rr <= 10000 {
                                    res.insert(ll * rr);
                                }
                            }
                        }
                    }
                }
                dp[j][j + i] = res;
            }
        }
        let mut stk = vec![];
        let mut mul = false;
        for &c in s {
            if c.is_ascii_digit() {
                let nc = (c - b'0') as i32;
                if mul {
                    let top = stk.pop().unwrap();
                    stk.push(top * nc);
                    mul = false;
                } else {
                    stk.push(nc);
                }
            } else {
                mul = c == b'*';
            }
        }
        let res = stk.into_iter().sum::<i32>();
        answers.into_iter().fold(0, |acc, x| {
            if x == res {
                acc + 5
            } else if dp[0][nop].contains(&x) {
                acc + 2
            } else {
                acc
            }
        })
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_score_of_students() {
        assert_eq!(Solution::score_of_students("7+3*1*2".to_string(), vec![20, 13, 42]), 7);
        assert_eq!(
            Solution::score_of_students("3+5*2".to_string(), vec![13, 0, 10, 13, 13, 16, 16]),
            19
        );
        assert_eq!(
            Solution::score_of_students("6+0*1".to_string(), vec![12, 9, 6, 4, 8, 6]),
            10
        );
    }
}
