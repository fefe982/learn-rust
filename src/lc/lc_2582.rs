// https://leetcode.com/problems/pass-the-pillow/
// 2582. Pass the Pillow
pub struct Solution;
impl Solution {
    pub fn pass_the_pillow(n: i32, time: i32) -> i32 {
        let time = time % ((n - 1) * 2);
        if time < n {
            1 + time
        } else {
            1 + (n - 1) * 2 - time
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        assert_eq!(Solution::pass_the_pillow(4, 5), 2);
        assert_eq!(Solution::pass_the_pillow(3, 2), 3);
    }
}
