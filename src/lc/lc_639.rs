// https://leetcode.com/problems/decode-ways-ii/
// 639. Decode Ways II
pub struct Solution;
const MOD: i64 = 1000000007;
#[derive(Copy, Clone)]
struct IMod {
    i: i64,
}
impl IMod {
    fn new(i: i32) -> Self {
        Self { i: i as i64 }
    }
    fn as_i32(&self) -> i32 {
        self.i as i32
    }
}
impl std::ops::Mul<i32> for IMod {
    type Output = Self;
    fn mul(self, rhs: i32) -> Self::Output {
        Self {
            i: (self.i * rhs as i64) % MOD,
        }
    }
}
impl std::ops::Add<IMod> for IMod {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self {
            i: (self.i + rhs.i) % MOD,
        }
    }
}
impl Solution {
    pub fn num_decodings(s: String) -> i32 {
        let mut c1 = IMod::new(1);
        let mut c2 = IMod::new(0);
        let mut last = '0';
        for c in s.chars() {
            let c0 = if c == '*' {
                c1 * 9
                    + c2 * if last == '*' {
                        15
                    } else if last == '1' {
                        9
                    } else if last == '2' {
                        6
                    } else {
                        0
                    }
            } else {
                (if c != '0' { c1 } else { IMod::new(0) })
                    + c2 * if last == '*' {
                        if c <= '6' {
                            2
                        } else {
                            1
                        }
                    } else if last == '1' || (last == '2' && c <= '6') {
                        1
                    } else {
                        0
                    }
            };
            last = c;
            c2 = c1;
            c1 = c0;
        }
        c1.as_i32()
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_num_decodings() {
        assert_eq!(Solution::num_decodings("*********".to_string()), 291868912);
        assert_eq!(Solution::num_decodings("**".to_string()), 96);
        assert_eq!(Solution::num_decodings("*".to_string()), 9);
        assert_eq!(Solution::num_decodings("1*".to_string()), 18);
        assert_eq!(Solution::num_decodings("2*".to_string()), 15);
    }
}
