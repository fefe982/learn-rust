// https://leetcode.com/problems/minimum-insertions-to-balance-a-parentheses-string/
// 1541. Minimum Insertions to Balance a Parentheses String
pub struct Solution;
impl Solution {
    pub fn min_insertions(s: String) -> i32 {
        let mut res = 0;
        let mut cl = 0;
        let mut cr = 0;
        for c in s.chars() {
            match c {
                '(' => {
                    if cr == 1 {
                        res += 1;
                        cr = 0;
                        if cl == 0 {
                            res += 1;
                        } else {
                            cl -= 1;
                        }
                    }
                    cl += 1;
                }
                ')' => {
                    if cr == 0 {
                        cr += 1;
                    } else {
                        cr -= 1;
                        if cl == 0 {
                            res += 1;
                        } else {
                            cl -= 1;
                        }
                    }
                }
                _ => unreachable!(),
            }
        }
        if cr == 1 {
            res += 1;
            if cl == 0 {
                res += 1;
            } else {
                cl -= 1;
            }
        }
        res + cl * 2
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn min_insertions() {
        assert_eq!(Solution::min_insertions("(()))".to_string()), 1);
        assert_eq!(Solution::min_insertions("())".to_string()), 0);
        assert_eq!(Solution::min_insertions("))())(".to_string()), 3);
    }
}
