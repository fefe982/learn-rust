// https://leetcode.com/problems/least-operators-to-express-number/
// 964. Least Operators to Express Number
pub struct Solution;
impl Solution {
    pub fn least_ops_express_target(x: i32, target: i32) -> i32 {
        Solution::least_ops_express_target_64(x as i64, target as i64) as i32
    }
    fn least_ops_express_target_64(x: i64, target: i64) -> i64 {
        if target < x {
            return (2 * target - 1).min(2 * (x - target));
        }
        if target == x {
            return 0;
        }
        let mut pow = x;
        let mut t = 0;
        while pow < target {
            pow *= x;
            t += 1;
        }
        if pow == target {
            return t;
        }
        (if pow - target >= target {
            i64::MAX
        } else {
            Solution::least_ops_express_target_64(x, pow - target) + t + 1
        })
        .min(Solution::least_ops_express_target_64(x, target - pow / x) + t)
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_least_ops_express_target() {
        assert_eq!(Solution::least_ops_express_target(79, 155800339), 45);
        assert_eq!(Solution::least_ops_express_target(3, 19), 5);
        assert_eq!(Solution::least_ops_express_target(5, 501), 8);
        assert_eq!(Solution::least_ops_express_target(100, 100000000), 3);
    }
}
