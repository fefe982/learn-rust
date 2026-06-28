// https://leetcode.com/problems/minimum-addition-to-make-integer-beautiful/
// 2457. Minimum Addition to Make Integer Beautiful
pub struct Solution;
impl Solution {
    pub fn make_integer_beautiful(n: i64, target: i32) -> i64 {
        let mut pow10 = 1;
        while pow10 <= n {
            pow10 *= 10;
        }
        pow10 /= 10;
        let mut sum = 0;
        let mut t = 0;
        let target = target as i64;
        while pow10 > 0 {
            let d = n / pow10 % 10;
            if sum + d == target && n % pow10 == 0 {
                return 0;
            }
            if sum + d >= target {
                t += pow10 * 10;
                return t - n;
            }
            t += d * pow10;
            sum += d;
            pow10 /= 10;
        }
        0
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn make_integer_beautiful() {
        assert_eq!(Solution::make_integer_beautiful(16, 6), 4);
        assert_eq!(Solution::make_integer_beautiful(467, 6), 33);
        assert_eq!(Solution::make_integer_beautiful(1, 1), 0);
    }
}
