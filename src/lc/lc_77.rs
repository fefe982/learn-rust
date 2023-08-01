// https://leetcode.com/problems/combinations/
// 77. Combinations
pub struct Solution;
impl Solution {
    pub fn combine(n: i32, k: i32) -> Vec<Vec<i32>> {
        let mut res = vec![];
        res.push((1..=k).collect::<Vec<i32>>());
        loop {
            let mut v = res.last().unwrap().clone();
            let mut next = 0;
            while let Some(vn) = v.pop() {
                if vn + (k - v.len() as i32 - 1) < n {
                    next = vn + 1;
                    break;
                }
            }
            if next == 0 {
                return res;
            }
            while v.len() < k as usize {
                v.push(next);
                next += 1;
            }
            res.push(v);
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn combine() {
        assert_eq!(
            Solution::combine(4, 2),
            vec_vec![[1, 2], [1, 3], [1, 4], [2, 3], [2, 4], [3, 4]]
        );
        assert_eq!(Solution::combine(1, 1), vec_vec![[1]]);
    }
}
