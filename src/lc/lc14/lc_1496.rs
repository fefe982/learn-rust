// https://leetcode.com/problems/path-crossing/
// 1496. Path Crossing
pub struct Solution;
impl Solution {
    pub fn is_path_crossing(path: String) -> bool {
        let mut x = 0;
        let mut y = 0;
        let mut visited = std::collections::HashSet::new();
        visited.insert((0, 0));
        for c in path.chars() {
            match c {
                'N' => y += 1,
                'S' => y -= 1,
                'E' => x += 1,
                'W' => x -= 1,
                _ => unreachable!(),
            }
            if visited.contains(&(x, y)) {
                return true;
            }
            visited.insert((x, y));
        }
        false
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_is_path_crossing() {
        assert_eq!(Solution::is_path_crossing("NES".to_string()), false);
        assert_eq!(Solution::is_path_crossing("NESWW".to_string()), true);
    }
}
