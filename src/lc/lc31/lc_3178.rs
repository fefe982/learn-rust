// https://leetcode.com/problems/find-the-child-who-has-the-ball-after-k-seconds/
// 3178. Find the Child Who Has the Ball After K Seconds
pub struct Solution;
impl Solution {
    pub fn number_of_child(n: i32, k: i32) -> i32 {
        let k = k % ((n - 1) * 2);
        if k < n {
            k
        } else {
            (n - 1) * 2 - k
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn number_of_child() {
        assert_eq!(Solution::number_of_child(3, 5), 1);
        assert_eq!(Solution::number_of_child(5, 6), 2);
        assert_eq!(Solution::number_of_child(4, 2), 2);
    }
}
