// https://leetcode.com/problems/reach-a-number/
// 754. Reach a Number
pub struct Solution;
impl Solution {
    pub fn reach_number(target: i32) -> i32 {
        let target = target.abs();
        if target == 0 {
            return 0;
        }
        let mut n = (((1.0 + target as f64 * 8.0).sqrt() - 1.0) / 2.0) as i32;
        let mut sum = n * (n + 1) / 2;
        if sum + n + 1 <= target {
            n += 1;
            sum += n + 1;
        }
        if sum == target {
            return n;
        }
        if (sum + n + 1 - target) % 2 == 0 {
            return n + 1;
        }
        if (target - sum) % 2 == 1 {
            return n + 2;
        }
        n + 3
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn reach_number() {
        assert_eq!(Solution::reach_number(-1000000000), 44723);
        assert_eq!(Solution::reach_number(2), 3);
        assert_eq!(Solution::reach_number(3), 2);
    }
}
