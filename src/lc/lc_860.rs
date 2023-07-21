// https://leetcode.com/problems/lemonade-change/
// 860. Lemonade Change
pub struct Solution;
impl Solution {
    pub fn lemonade_change(bills: Vec<i32>) -> bool {
        let mut c = [0, 0, 0];
        for b in bills {
            match b {
                5 => c[0] += 1,
                10 => {
                    if c[0] < 1 {
                        return false;
                    } else {
                        c[0] -= 1;
                        c[1] += 1;
                    }
                }
                20 => {
                    if c[1] >= 1 && c[0] >= 1 {
                        c[1] -= 1;
                        c[0] -= 1;
                        c[2] += 1;
                    } else if c[0] >= 3 {
                        c[0] -= 3;
                        c[2] += 1;
                    } else {
                        return false;
                    }
                }
                _ => unreachable!(),
            }
        }
        true
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn lemonade_change() {
        assert_eq!(
            Solution::lemonade_change(vec![5, 5, 10, 10, 5, 20, 5, 10, 5, 5]),
            true
        );
        assert_eq!(Solution::lemonade_change(vec![5, 5, 5, 10, 20]), true);
        assert_eq!(Solution::lemonade_change(vec![5, 5, 10, 10, 20]), false);
    }
}
