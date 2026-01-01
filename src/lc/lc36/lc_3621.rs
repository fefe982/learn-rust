// https://leetcode.com/problems/number-of-integers-with-popcount-depth-equal-to-k-i/
// 3621. Number of Integers With Popcount Depth Equal to K I
pub struct Solution;
impl Solution {
    pub fn popcount_depth(n: i64, k: i32) -> i64 {
        if k == 0 {
            return 1;
        }
        if k == 1 {
            return (i64::BITS - n.leading_zeros()) as i64 - 1;
        }
        let n = n + 1;
        let ln = (i64::BITS - n.leading_zeros()) as usize;
        let mut ks = vec![0; ln];
        for i in 2..ln {
            ks[i] = ks[i.count_ones() as usize] + 1;
        }
        let mut c = vec![vec![1; ln]; ln];
        for i in 2..ln {
            for j in 1..i {
                c[i][j] = c[i - 1][j] + c[i - 1][j - 1];
            }
        }
        let mut ans = 0;
        let mut n = n;
        let mut i = 0;
        while n > 0 {
            let bn = (i64::BITS - n.leading_zeros()) as usize - 1;
            for j in 0..=bn {
                if ks[i + j] == k - 1 {
                    ans = ans + c[bn][j]
                }
            }
            n ^= 1 << bn;
            i += 1;
        }
        ans
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn popcount_depth() {
        assert_eq!(Solution::popcount_depth(4, 1), 2);
        assert_eq!(Solution::popcount_depth(7, 2), 3);
    }
}
