// https://leetcode.com/problems/solve-the-equation/
// 640. Solve the Equation
pub struct Solution;
impl Solution {
    fn parse(s: &str) -> (i32, i32) {
        let mut x = 0;
        let mut c = 0;
        let mut num = 0;
        let mut num_sign = 1;
        let mut is_num = false;
        for ch in s.chars() {
            match ch {
                '+' | '-' => {
                    c += num_sign * num;
                    num = 0;
                    num_sign = if ch == '+' { 1 } else { -1 };
                    is_num = false;
                }
                'x' => {
                    if is_num {
                        x += num_sign * num;
                    } else {
                        x += num_sign;
                    }
                    num = 0;
                    is_num = false;
                }
                _ => {
                    num = num * 10 + ch.to_digit(10).unwrap() as i32;
                    is_num = true;
                }
            }
        }
        if is_num {
            c += num_sign * num;
        }
        (x, c)
    }
    pub fn solve_equation(equation: String) -> String {
        let (left, right) = equation.split_once('=').unwrap();
        let (left, right) = (Self::parse(left), Self::parse(right));
        if left.0 == right.0 {
            if left.1 == right.1 {
                return "Infinite solutions".to_string();
            } else {
                return "No solution".to_string();
            }
        }
        format!("x={}", (right.1 - left.1) / (left.0 - right.0))
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn solve_equation() {
        assert_eq!(Solution::solve_equation("x+5-3+x=6+x-2".to_string()), "x=2".to_string());
        assert_eq!(
            Solution::solve_equation("x=x".to_string()),
            "Infinite solutions".to_string()
        );
        assert_eq!(Solution::solve_equation("2x=x".to_string()), "x=0".to_string());
    }
}
