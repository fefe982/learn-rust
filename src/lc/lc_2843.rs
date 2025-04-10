// https://leetcode.cn/problems/count-symmetric-integers/
// 2843. Count Symmetric Pairs
pub struct Solution;
impl Solution {
    fn cnt2(d1: i32, d0: i32, s: i32, z: i32) -> i32 {
        let mut c = 0;
        if d1 > s {
            c = s + z;
        }
        if d1 <= s {
            let cc = d1 - (s - 9).max(1 - z) + 1;
            if d1 + d0 > s {
                c = cc;
            } else {
                c = cc - 1;
            }
        }
        c.max(0)
    }
    fn count(n: i32) -> i32 {
        if n <= 100 {
            return n / 11;
        }
        if n <= 1000 {
            return 9;
        }
        let n0 = n % 10;
        let n1 = n / 10 % 10;
        let n2 = n / 100 % 10;
        let n3 = n / 1000 % 10;
        let mut cnt = 9;
        for i in 1..=18 {
            let c1 = Self::cnt2(n3, n2, i, 0);
            cnt += c1 * (9.min(i) - (i - 9).max(0) + 1);
            if n2 + n3 == i {
                cnt += Self::cnt2(n1, n0, i, 1);
                if n1 + n0 == i {
                    cnt += 1;
                }
            }
            if c1 == 0 {
                break;
            }
        }
        cnt
    }
    pub fn count_symmetric_integers(low: i32, high: i32) -> i32 {
        let high = high.min(9999);
        Self::count(high) - Self::count(low - 1)
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn count_symmetric_integers() {
        assert_eq!(Solution::count_symmetric_integers(100, 1782), 44);
        assert_eq!(Solution::count_symmetric_integers(1, 100), 9);
        assert_eq!(Solution::count_symmetric_integers(1200, 1230), 4);
    }
}
