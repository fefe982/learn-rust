// https://leetcode.com/problems/number-of-ways-to-stay-in-the-same-place-after-some-steps/
// 1269. Number of Ways to Stay in the Same Place After Some Steps
pub struct Solution;
const MOD: i64 = 10_0000_0007;
#[derive(Copy, Clone)]
struct IMod {
    i: i64,
}
impl IMod {
    fn new(i: i32) -> Self {
        IMod { i: i as i64 }
    }
    fn val(&self) -> i32 {
        self.i as i32
    }
}
impl std::ops::Add for IMod {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        IMod {
            i: (self.i + other.i) % MOD,
        }
    }
}
impl Solution {
    pub fn num_ways(steps: i32, arr_len: i32) -> i32 {
        let steps = steps as usize;
        let len = steps.min(arr_len as usize);
        if len == 1 {
            return 1;
        }
        let mut dp = vec![IMod::new(0); len + 1];
        dp[0] = IMod::new(1);
        dp[1] = IMod::new(1);
        for i in 1..steps {
            let mut last = dp[(steps - i).min(len)];
            for j in (1..(steps - i).min(len)).rev() {
                let save = dp[j];
                dp[j] = dp[j - 1] + dp[j] + last;
                last = save;
            }
            dp[0] = dp[0] + last;
        }
        dp[0].val() as i32
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_num_ways() {
        assert_eq!(Solution::num_ways(3, 2), 4);
        assert_eq!(Solution::num_ways(2, 4), 2);
        assert_eq!(Solution::num_ways(4, 2), 8);
    }
}
