// https://leetcode.com/problems/eliminate-maximum-number-of-monsters/
// 1921. Eliminate Maximum Number of Monsters
pub struct Solution;
impl Solution {
    pub fn eliminate_maximum(dist: Vec<i32>, speed: Vec<i32>) -> i32 {
        let mut t = Vec::with_capacity(dist.len());
        for (d, s) in dist.into_iter().zip(speed.into_iter()) {
            t.push((d + s - 1) / s);
        }
        t.sort_unstable();
        let l = t.len();
        for (j, tt) in t.into_iter().enumerate() {
            if tt as usize <= j {
                return j as i32;
            }
        }
        l as i32
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