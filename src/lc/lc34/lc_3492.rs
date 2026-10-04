// https://leetcode.com/problems/maximum-containers-on-a-ship/
// 3492. Maximum Containers on a Ship
pub struct Solution;
impl Solution {
    pub fn max_containers(n: i32, w: i32, max_weight: i32) -> i32 {
        (n * n).min(max_weight / w)
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn max_containers() {
        assert_eq!(Solution::max_containers(2, 3, 15), 4);
        assert_eq!(Solution::max_containers(3, 5, 20), 4);
    }
}
