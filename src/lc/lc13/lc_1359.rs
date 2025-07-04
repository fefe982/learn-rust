// https://leetcode.com/problems/count-all-valid-pickup-and-delivery-options/
// 1359. Count All Valid Pickup and Delivery Options
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
    fn val(self) -> i32 {
        self.i as i32
    }
}
impl std::ops::MulAssign<i32> for IMod {
    fn mul_assign(&mut self, rhs: i32) {
        self.i = (self.i * rhs as i64) % MOD;
    }
}
impl Solution {
    pub fn count_orders(n: i32) -> i32 {
        let mut ans = IMod::new(1);
        for i in 1..=n {
            ans *= i;
            ans *= 2 * i - 1;
        }
        ans.val()
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        assert_eq!(Solution::count_orders(1), 1);
        assert_eq!(Solution::count_orders(2), 6);
        assert_eq!(Solution::count_orders(3), 90);
    }
}
