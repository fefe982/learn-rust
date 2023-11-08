// https://leetcode.com/problems/determine-if-a-cell-is-reachable-at-a-given-time/
// 2849. Determine if a Cell Is Reachable at a Given Time
pub struct Solution;
impl Solution {
    pub fn is_reachable_at_time(sx: i32, sy: i32, fx: i32, fy: i32, t: i32) -> bool {
        let dx = (sx - fx).abs();
        let dy = (sy - fy).abs();
        if dx == 0 && dy == 0 {
            t == 0 || t >= 2
        } else {
            t >= dx + dy - dx.min(dy)
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_is_reachable_at_time() {
        assert_eq!(Solution::is_reachable_at_time(1, 2, 1, 2, 1), false);
        assert_eq!(Solution::is_reachable_at_time(2, 4, 7, 7, 6), true);
        assert_eq!(Solution::is_reachable_at_time(3, 1, 7, 3, 3), false);
    }
}
