// https://leetcode.com/problems/eliminate-maximum-number-of-monsters/
// 1921. Eliminate Maximum Number of Monsters
pub struct Solution;
impl Solution {
    pub fn eliminate_maximum(dist: Vec<i32>, speed: Vec<i32>) -> i32 {
        let mut t = dist
            .into_iter()
            .zip(speed.into_iter())
            .map(|(d, s)| (d + s - 1) / s)
            .collect::<Vec<_>>();
        t.sort_unstable();
        for (j, tt) in t.into_iter().chain(vec![0].into_iter()).enumerate() {
            if tt as usize <= j {
                return j as i32;
            }
        }
        0
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn eliminate_maximum() {
        assert_eq!(Solution::eliminate_maximum(vec![1, 3, 4], vec![1, 1, 1]), 3);
        assert_eq!(
            Solution::eliminate_maximum(vec![1, 1, 2, 3], vec![1, 1, 1, 1]),
            1
        );
    }
}
