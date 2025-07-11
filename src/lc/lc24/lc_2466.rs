// https://leetcode.com/problems/count-ways-to-build-good-strings/
// 2466. Count Ways To Build Good Strings
pub struct Solution;
const MODULO: i64 = 1000000007;
#[derive(Copy, Clone)]
struct IMod {
    i: i32,
}
impl IMod {
    fn new(i: i32) -> Self {
        Self { i }
    }
}
impl std::ops::Add<IMod> for IMod {
    type Output = IMod;
    fn add(self, rhs: IMod) -> IMod {
        return IMod::new(((self.i as i64 + rhs.i as i64) % MODULO) as i32);
    }
}
impl Solution {
    pub fn count_good_strings(low: i32, high: i32, zero: i32, one: i32) -> i32 {
        let mut sum = IMod::new(0);
        let mut v = vec![IMod::new(0); (high + 1) as usize];
        v[0].i = 1;
        let m = std::cmp::min(zero, one);
        for i in m..=high {
            if i >= zero {
                v[i as usize] = v[i as usize] + v[(i - zero) as usize];
            }
            if i >= one {
                v[i as usize] = v[i as usize] + v[(i - one) as usize];
            }
            if i >= low {
                sum = sum + v[i as usize];
            }
        }
        sum.i
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn count_good_strings() {
        assert_eq!(Solution::count_good_strings(3, 3, 1, 1), 8);
        assert_eq!(Solution::count_good_strings(2, 3, 1, 2), 5);
    }
}
