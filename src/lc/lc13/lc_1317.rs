// https://leetcode.com/problems/convert-integer-to-the-sum-of-two-no-zero-integers/
// 1317. Convert Integer to the Sum of Two No-Zero Integers
pub struct Solution;
impl Solution {
    pub fn get_no_zero_integers(n: i32) -> Vec<i32> {
        let mut i = 1;
        'i: loop {
            let mut j = n - i;
            let mut add = 1;
            while j != 0 {
                if j % 10 == 0 {
                    let mut ii = i + add;
                    if i > add {
                        add *= 10;
                        while ii > add {
                            ii -= add;
                            add *= 10;
                        }
                    } else {
                        add /= 10;
                        while add > i {
                            ii += add;
                            add /= 10;
                        }
                    }
                    i = ii;
                    continue 'i;
                }
                j /= 10;
                add *= 10;
            }
            return vec![i, n - i];
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    fn check_zero(mut n: i32) {
        while n != 0 {
            assert_ne!(n % 10, 0);
            n /= 10;
        }
    }
    fn check(n: i32) {
        let res = Solution::get_no_zero_integers(n);
        assert_eq!(res.len(), 2);
        assert!(res[0] > 0);
        assert!(res[1] > 0);
        assert_eq!(res[0] + res[1], n);
        check_zero(res[0]);
        check_zero(res[1]);
    }
    #[test]
    fn get_no_zero_integers() {
        check(1057);
        check(1010);
        check(2);
        check(11);
    }
}
