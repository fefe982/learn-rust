// https://leetcode.com/problems/expression-add-operators/
// 282. Expression Add Operators
pub struct Solution;
#[derive(Clone, Copy)]
enum Op {
    ADD,
    SUB,
    MUL,
    NONE,
}
impl Solution {
    fn add_op(
        nums: &[u8],
        target: i64,
        prev: i64,
        add: i64,
        mul: i64,
        last_op: Op,
        exp: &mut Vec<u8>,
        res: &mut Vec<String>,
    ) {
        if nums.len() == 0 {
            let r = match last_op {
                Op::ADD => add + prev,
                Op::SUB => add - prev,
                Op::MUL => add + mul * prev,
                Op::NONE => prev,
            };
            if r == target {
                res.push(String::from_utf8(exp.clone()).unwrap());
            }
            return;
        }
        let nd = nums[0];
        if prev != 0 {
            exp.push(nd + b'0');
            Self::add_op(
                &nums[1..],
                target,
                prev * 10 + nd as i64,
                add,
                mul,
                last_op,
                exp,
                res,
            );
            exp.pop();
        }
        let l = exp.len();
        exp.push(b'*');
        exp.push(nd + b'0');
        let nd = nd as i64;
        match last_op {
            Op::ADD => Self::add_op(&nums[1..], target, nd, add, prev, Op::MUL, exp, res),
            Op::SUB => Self::add_op(&nums[1..], target, nd, add, -prev, Op::MUL, exp, res),
            Op::MUL => Self::add_op(&nums[1..], target, nd, add, mul * prev, Op::MUL, exp, res),
            Op::NONE => Self::add_op(&nums[1..], target, nd, 0, prev, Op::MUL, exp, res),
        }
        exp[l] = b'+';
        match last_op {
            Op::ADD => Self::add_op(&nums[1..], target, nd, add + prev, 1, Op::ADD, exp, res),
            Op::SUB => Self::add_op(&nums[1..], target, nd, add - prev, 1, Op::ADD, exp, res),
            Op::MUL => Self::add_op(
                &nums[1..],
                target,
                nd,
                add + mul * prev,
                1,
                Op::ADD,
                exp,
                res,
            ),
            Op::NONE => Self::add_op(&nums[1..], target, nd, prev, 1, Op::ADD, exp, res),
        }
        exp[l] = b'-';
        match last_op {
            Op::ADD => Self::add_op(&nums[1..], target, nd, add + prev, 1, Op::SUB, exp, res),
            Op::SUB => Self::add_op(&nums[1..], target, nd, add - prev, 1, Op::SUB, exp, res),
            Op::MUL => Self::add_op(
                &nums[1..],
                target,
                nd,
                add + mul * prev,
                1,
                Op::SUB,
                exp,
                res,
            ),
            Op::NONE => Self::add_op(&nums[1..], target, nd, prev, 1, Op::SUB, exp, res),
        }
        exp.pop();
        exp.pop();
    }
    pub fn add_operators(num: String, target: i32) -> Vec<String> {
        let mut res = Vec::new();
        let num: Vec<u8> = num.as_bytes().iter().map(|x| x - b'0').collect();
        let mut exp = vec![num[0] + b'0'];
        Self::add_op(
            &num[1..],
            target as i64,
            num[0] as i64,
            0,
            1,
            Op::NONE,
            &mut exp,
            &mut res,
        );
        res
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn add_operators() {
        assert_eq!(
            Solution::add_operators(String::from("105"), 5),
            vec_str!["10-5", "1*0+5"]
        );
        assert_eq!(
            Solution::add_operators(String::from("123"), 6),
            vec_str!["1*2*3", "1+2+3"]
        );
        assert_eq!(
            Solution::add_operators(String::from("232"), 8),
            vec_str!["2*3+2", "2+3*2"]
        );
        assert_eq!(
            Solution::add_operators(String::from("3456237490"), 9191),
            Vec::<String>::new()
        )
    }
}
