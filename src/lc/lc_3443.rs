// https://leetcode.com/problems/maximum-manhattan-distance-after-k-changes/
// 3443. Maximum Manhattan Distance After K Changes
pub struct Solution;
impl Solution {
    pub fn max_distance(s: String, k: i32) -> i32 {
        let mut nc: i32 = 0;
        let mut sc: i32 = 0;
        let mut wc: i32 = 0;
        let mut ec: i32 = 0;
        let mut max = 0;
        for c in s.chars() {
            match c {
                'N' => nc += 1,
                'S' => sc += 1,
                'W' => wc += 1,
                'E' => ec += 1,
                _ => {}
            }
            max = max.max((nc - sc).abs() + (wc - ec).abs() + (nc.min(sc) + wc.min(ec)).min(k) * 2);
        }
        max
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn max_distance() {
        assert_eq!(Solution::max_distance("NWSE".to_string(), 1), 3);
        assert_eq!(Solution::max_distance("NSWWEW".to_string(), 3), 6);
    }
}
