// https://leetcode.com/problems/minimum-cost-to-change-the-final-value-of-expression/
// 1896. Minimum Cost to Change the Final Value of Expression
pub struct Solution;
impl Solution {
    fn min_op(expr: &[u8], offset: usize, pair: &Vec<usize>) -> (i32, i32) {
        let mut start = expr.len() - 1;
        if expr[start] == b')' {
            start = pair[start + offset] - offset;
        }
        if start == 0 {
            if expr.len() > 1 {
                Self::min_op(&expr[1..expr.len() - 1], offset + 1, pair)
            } else {
                if expr[0] == b'1' {
                    (1, 1)
                } else if expr[0] == b'0' {
                    (0, 1)
                } else {
                    unreachable!()
                }
            }
        } else {
            let left = Self::min_op(&expr[0..start - 1], offset, pair);
            let right = Self::min_op(&expr[start..expr.len()], offset + start, pair);
            match (expr[start - 1], left.0, right.0) {
                (b'|', 0, 0) => (0, left.1.min(right.1)),
                (b'|', 1, 0) => (1, 1),
                (b'|', 0, 1) => (1, 1),
                (b'|', 1, 1) => (1, left.1.min(right.1) + 1),
                (b'&', 0, 0) => (0, left.1.min(right.1) + 1),
                (b'&', 1, 0) => (0, 1),
                (b'&', 0, 1) => (0, 1),
                (b'&', 1, 1) => (1, left.1.min(right.1)),
                _ => unreachable!(),
            }
        }
    }
    pub fn min_operations_to_flip(expression: String) -> i32 {
        let expr = expression.as_bytes();
        let mut pair = vec![0; expr.len()];
        let mut stack = vec![];
        for i in 0..expr.len() {
            if expr[i] == b'(' {
                stack.push(i);
            } else if expr[i] == b')' {
                let j = stack.pop().unwrap();
                pair[i] = j;
            }
        }
        Self::min_op(expr, 0, &pair).1
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_min_operation_to_flip() {
        assert_eq!(Solution::min_operations_to_flip("1&(0|1)".to_string()), 1);
        assert_eq!(Solution::min_operations_to_flip("(0&0)&(0&0&0)".to_string()), 3);
        assert_eq!(Solution::min_operations_to_flip("(0|(1|0&1))".to_string()), 1);
    }
}
