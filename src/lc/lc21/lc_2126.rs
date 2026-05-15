// https://leetcode.com/problems/destroying-asteroids/
// 2126. Destroying Asteroids
pub struct Solution;
impl Solution {
    pub fn asteroids_destroyed(mass: i32, asteroids: Vec<i32>) -> bool {
        let mut mass = mass as i64;
        let mut asteroids = asteroids;
        asteroids.sort_unstable();
        for a in asteroids {
            if mass < a as i64 {
                return false;
            }
            mass += a as i64;
        }
        true
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_asteroids_destroyed() {
        assert_eq!(Solution::asteroids_destroyed(10, vec![3, 9, 19, 5, 21]), true);
        assert_eq!(Solution::asteroids_destroyed(5, vec![4, 9, 23, 4]), false);
    }
}
