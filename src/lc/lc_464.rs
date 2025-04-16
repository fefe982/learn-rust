// https://leetcode.com/problems/can-i-win/
// 464. Can I Win
pub struct Solution;
impl Solution {
    fn trial(imask: usize, total: i32, dp: &mut Vec<Option<bool>>) -> bool {
        if let Some(b) = dp[imask] {
            return b;
        }
        let mut mask = imask;
        while mask > 0 {
            let t = (usize::BITS - mask.leading_zeros()) as i32;
            if t >= total || !Self::trial(imask ^ (1 << (t - 1)), total - t, dp) {
                dp[imask] = Some(true);
                return true;
            }
            mask ^= 1 << (t - 1);
        }
        dp[imask] = Some(false);
        false
    }
    pub fn can_i_win(max_choosable_integer: i32, desired_total: i32) -> bool {
        if max_choosable_integer * (max_choosable_integer + 1) / 2 < desired_total {
            return false;
        }
        Self::trial(
            (1 << max_choosable_integer) - 1,
            desired_total,
            &mut vec![None; 1 << max_choosable_integer],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn can_i_win() {
        assert_eq!(Solution::can_i_win(5, 50), false);
        assert_eq!(Solution::can_i_win(10, 11), false);
        assert_eq!(Solution::can_i_win(10, 0), true);
        assert_eq!(Solution::can_i_win(10, 1), true);
    }
}
