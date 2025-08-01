// https://leetcode.com/problems/cycle-length-queries-in-a-tree/
// 2509. Cycle Length Queries in a Tree
pub struct Solution;
impl Solution {
    pub fn cycle_length_queries(_n: i32, queries: Vec<Vec<i32>>) -> Vec<i32> {
        let mut res = Vec::with_capacity(queries.len());
        for mut q in queries {
            let tz0 = q[0].leading_zeros() as i32;
            let tz1 = q[1].leading_zeros() as i32;
            let mut r = 1;
            if tz0 > tz1 {
                r += tz0 - tz1;
                q[1] >>= tz0 - tz1;
            } else if tz1 > tz0 {
                r += tz1 - tz0;
                q[0] >>= tz1 - tz0;
            }
            while q[0] != q[1] {
                q[0] >>= 1;
                q[1] >>= 1;
                r += 2
            }
            res.push(r);
        }
        res
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn test_cycle_length_queries() {
        assert_eq!(
            Solution::cycle_length_queries(3, vec_vec![[5, 3], [4, 7], [2, 3]]),
            vec![4, 5, 3]
        );
        assert_eq!(Solution::cycle_length_queries(2, vec_vec![[1, 2]]), vec![2]);
    }
}
