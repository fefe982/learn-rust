// https://leetcode.com/problems/unit-conversion-i/
// 3528. Unit Conversion I
pub struct Solution;
impl Solution {
    pub fn base_unit_conversions(conversions: Vec<Vec<i32>>) -> Vec<i32> {
        let n = conversions.len() + 1;
        let mut m = vec![(0, 0); n];
        for c in conversions {
            m[c[1] as usize] = (c[0], c[2]);
        }
        let mut v = vec![0; n];
        v[0] = 1;
        for i in 1..n {
            if v[i] != 0 {
                continue;
            }
            let mut s = vec![];
            s.push(i);
            while let Some(&j) = s.last() {
                if v[j] != 0 {
                    break;
                } else {
                    s.push(m[j].0 as usize);
                }
            }
            let mut val = v[s.pop().unwrap()] as i64;
            while let Some(j) = s.pop() {
                val = val * (m)[j].1 as i64 % 1_000_000_007;
                v[j] = val as i32;
            }
        }
        v
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn base_unit_conversions() {
        assert_eq!(
            Solution::base_unit_conversions(vec_vec![[0, 1, 2], [1, 2, 3]]),
            vec![1, 2, 6]
        );
        assert_eq!(
            Solution::base_unit_conversions(vec_vec![
                [0, 1, 2],
                [0, 2, 3],
                [1, 3, 4],
                [1, 4, 5],
                [2, 5, 2],
                [4, 6, 3],
                [5, 7, 4]
            ]),
            vec![1, 2, 3, 8, 10, 6, 30, 24]
        );
    }
}
