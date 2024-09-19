// https://leetcode.com/problems/different-ways-to-add-parentheses/
// 241. Different Ways to Add Parentheses
pub struct Solution;
impl Solution {
    pub fn diff_ways_to_compute(expression: String) -> Vec<i32> {
        let mut nums = vec![];
        let mut op = vec![];
        let mut last_op = true;
        for c in expression.chars() {
            match c {
                '+' | '-' | '*' => {
                    op.push(c);
                    last_op = true;
                }
                '0'..='9' => {
                    if last_op {
                        nums.push(c.to_digit(10).unwrap() as i32);
                    } else {
                        let num = nums.last_mut().unwrap();
                        *num = *num * 10 + c.to_digit(10).unwrap() as i32;
                    }
                    last_op = false;
                }
                _ => {}
            }
        }
        let n = nums.len();
        let mut vals = vec![vec![vec![]; n]; n];
        let mut v = vec![];
        for len in 0..n {
            for i in 0..n - len {
                let j = i + len;
                if i == j {
                    vals[i][j].push(nums[i]);
                } else {
                    for k in i..j {
                        for &v1 in &vals[i][k] {
                            for &v2 in &vals[k + 1][j] {
                                match op[k] {
                                    '+' => v.push(v1 + v2),
                                    '-' => v.push(v1 - v2),
                                    '*' => v.push(v1 * v2),
                                    _ => {}
                                }
                            }
                        }
                    }
                    std::mem::swap(&mut vals[i][j], &mut v);
                }
            }
        }
        std::mem::swap(&mut v, &mut vals[0][n - 1]);
        v
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn diff_ways_to_compute() {
        assert_sort_eq!(Solution::diff_ways_to_compute("2-1-1".to_string()), vec![0, 2]);
        assert_sort_eq!(
            Solution::diff_ways_to_compute("2*3-4*5".to_string()),
            vec![-34, -14, -10, -10, 10]
        );
    }
}
