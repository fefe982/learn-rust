// https://leetcode.com/problems/range-addition-ii/
// 598. Range Addition II
pub struct Solution;
impl Solution {
    pub fn max_count(m: i32, n: i32, ops: Vec<Vec<i32>>) -> i32 {
        let lr = ops.into_iter().fold((m, n), |(l, r), v| (l.min(v[0]), r.min(v[1])));
        lr.0 * lr.1
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn max_count() {
        assert_eq!(Solution::max_count(3, 3, vec_vec![[2, 2], [3, 3]]), 4);
        assert_eq!(
            Solution::max_count(
                3,
                3,
                vec_vec![
                    [2, 2],
                    [3, 3],
                    [3, 3],
                    [3, 3],
                    [2, 2],
                    [3, 3],
                    [3, 3],
                    [3, 3],
                    [2, 2],
                    [3, 3],
                    [3, 3],
                    [3, 3]
                ]
            ),
            4
        );
        assert_eq!(Solution::max_count(3, 3, vec_vec![]), 9);
    }
}
