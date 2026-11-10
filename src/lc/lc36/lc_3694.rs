// https://leetcode.com/problems/distinct-points-reachable-after-substring-removal/
// 3694. Distinct Points Reachable After Substring Removal
pub struct Solution;
impl Solution {
    pub fn distinct_points(s: String, k: i32) -> i32 {
        let s = s.as_bytes();
        let mut ans = 1;
        let mut x = 0;
        let mut y = 0;
        let mut set = std::collections::HashSet::new();
        set.insert((0, 0));
        let k = k as usize;
        for i in k..s.len() {
            match s[i] {
                b'L' => {
                    x -= 1;
                }
                b'R' => {
                    x += 1;
                }
                b'U' => {
                    y += 1;
                }
                b'D' => {
                    y -= 1;
                }
                _ => {}
            }
            match s[i - k] {
                b'L' => {
                    x += 1;
                }
                b'R' => {
                    x -= 1;
                }
                b'U' => {
                    y -= 1;
                }
                b'D' => {
                    y += 1;
                }
                _ => {}
            }
            if set.insert((x, y)) {
                ans += 1;
            }
        }
        ans
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn distinct_points() {
        assert_eq!(Solution::distinct_points("LUL".to_string(), 1), 2);
        assert_eq!(Solution::distinct_points("UDLR".to_string(), 4), 1);
        assert_eq!(Solution::distinct_points("UU".to_string(), 1), 1);
    }
}
