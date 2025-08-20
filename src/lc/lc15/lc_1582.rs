// https://leetcode.com/problems/special-positions-in-a-binary-matrix/
// 1582. Special Positions in a Binary Matrix

pub struct Solution;

impl Solution {
    pub fn num_special(mat: Vec<Vec<i32>>) -> i32 {
        let mut sum = 0;
        const _INIT_: i32 = -1;
        const _ILLEGAL_: i32 = -2;
        let mut spr = vec![_INIT_; mat.len()];
        let mut spc = vec![0; mat[0].len()];
        for (r, vc) in mat.iter().enumerate() {
            for (c, elem) in vc.iter().enumerate() {
                if *elem == 1 {
                    if spr[r] == _INIT_ {
                        spr[r] = c as i32
                    } else if spr[r] >= 0 {
                        spr[r] = _ILLEGAL_
                    }
                    spc[c] += 1
                }
            }
        }
        for c in spr.iter() {
            if *c >= 0 && spc[*c as usize] == 1 {
                sum += 1
            }
        }
        sum
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn num_special() {
        assert_eq!(
            Solution::num_special(vec![vec![1, 0, 0], vec![0, 0, 1], vec![1, 0, 0]]),
            1
        );
        assert_eq!(
            Solution::num_special(vec![vec![1, 0, 0], vec![0, 1, 0], vec![0, 0, 1]]),
            3
        );
    }
}
