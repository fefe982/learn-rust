// https://leetcode.com/problems/k-th-smallest-in-lexicographical-order/
// 440. K-th Smallest in Lexicographical Order
pub struct Solution;
impl Solution {
    fn count(power: i32, n: i32, i: i32) -> (i32, i32, i32) {
        let p = n / (power / 10);
        if p > i {
            ((power - 1) / 9, power, n)
        } else if p == i {
            (
                (power - 1) / 9 - (power / 10 - (n - i * (power / 10) + 1)),
                power / 10,
                power / 10 - 1,
            )
        } else {
            Self::count(power / 10, power / 10 - 1, i)
        }
    }
    fn find_kth(prefix: i32, mut power: i32, mut n: i32, mut k: i32) -> i32 {
        if prefix != 0 {
            if k == 1 {
                return prefix;
            }
            k -= 1;
        }
        for i in 0..=9 {
            if prefix == 0 && i == 0 {
                continue;
            }
            let (cnt, pi, ni) = Self::count(power, n, i);
            if cnt < k {
                k -= cnt;
                power = pi;
                n = ni;
            } else {
                let p = n / (power / 10);
                if p > i {
                    return Self::find_kth(prefix * 10 + i, power / 10, power / 10 - 1, k);
                } else if p == i {
                    return Self::find_kth(prefix * 10 + i, power / 10, n - i * (power / 10), k);
                } else {
                    unreachable!()
                }
            }
        }
        0
    }
    pub fn find_kth_number(n: i32, k: i32) -> i32 {
        let mut power = 1;
        while power <= n {
            power *= 10;
        }
        Self::find_kth(0, power, n, k)
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn find_kth_number() {
        assert_eq!(Solution::find_kth_number(13, 2), 10);
        assert_eq!(Solution::find_kth_number(1, 1), 1);
    }
}
