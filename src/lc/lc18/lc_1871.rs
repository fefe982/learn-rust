// https://leetcode.com/problems/jump-game-vii/
// 1871. Jump Game VII
pub struct Solution;
impl Solution {
    pub fn can_reach(s: String, min_jump: i32, max_jump: i32) -> bool {
        let bytes = s.as_bytes();
        let n = bytes.len();
        if bytes[n - 1] != b'0' {
            return false;
        }

        let min_jump = min_jump as usize;
        let max_jump = max_jump as usize;

        let mut reachable = vec![false; n];
        reachable[0] = true;

        // `window_count` tracks how many reachable indices fall in
        // [i - max_jump, i - min_jump] for current i.
        let mut window_count = 0i32;

        for i in 1..n {
            if i >= min_jump && reachable[i - min_jump] {
                window_count += 1;
            }
            if i > max_jump && reachable[i - max_jump - 1] {
                window_count -= 1;
            }

            if bytes[i] == b'0' && window_count > 0 {
                reachable[i] = true;
            }
        }

        reachable[n - 1]
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_can_reach() {
        assert_eq!(Solution::can_reach("011010".to_string(), 2, 3), true);
        assert_eq!(Solution::can_reach("01101110".to_string(), 2, 3), false);
    }
}
