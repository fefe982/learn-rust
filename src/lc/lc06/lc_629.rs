// https://leetcode.com/problems/k-inverse-pairs-array/
// 629. K Inverse Pairs Array
pub struct Solution;
impl Solution {
    pub fn k_inverse_pairs(n: i32, k: i32) -> i32 {
        let k = k as usize;
        let n = n as usize;
        let md = 1000000007;
        let mut v = vec![0; k + 1];
        v[0] = 1;
        for i in 2..=n {
            let mut nv = v.clone();
            nv[0] = 1;
            for ik in 1..=k {
                nv[ik] = (v[ik] + nv[ik - 1]) % md;
                if ik >= i {
                    nv[ik] = (nv[ik] + md - v[ik - i]) % md;
                }
            }
            v = nv;
        }
        v[k]
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn k_inverse_pairs() {
        assert_eq!(Solution::k_inverse_pairs(4, 6), 1);
        assert_eq!(Solution::k_inverse_pairs(3, 0), 1);
        assert_eq!(Solution::k_inverse_pairs(3, 1), 2);
    }
}
