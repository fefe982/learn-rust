// https://leetcode.com/problems/find-closest-person/
// 3516. Find Closest Person
pub struct Solution;
impl Solution {
    pub fn find_closest(x: i32, y: i32, z: i32) -> i32 {
        match (x - z).abs().cmp(&(y - z).abs()) {
            std::cmp::Ordering::Less => 1,
            std::cmp::Ordering::Greater => 2,
            std::cmp::Ordering::Equal => 0,
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn find_closest() {
        assert_eq!(Solution::find_closest(2, 7, 4), 1);
        assert_eq!(Solution::find_closest(2, 5, 6), 2);
        assert_eq!(Solution::find_closest(1, 5, 3), 0);
    }
}
