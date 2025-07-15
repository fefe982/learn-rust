// https://leetcode.com/problems/find-the-key-of-the-numbers/
// 3270. Find the Key of the Numbers
pub struct Solution;
impl Solution {
    pub fn generate_key(num1: i32, num2: i32, num3: i32) -> i32 {
        let mut key = 0;
        let mut pow = 1000;
        for _ in 0..4 {
            let a = num1 / pow % 10;
            let b = num2 / pow % 10;
            let c = num3 / pow % 10;
            key = key * 10 + a.min(b).min(c);
            pow /= 10;
        }
        key
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_generate_key() {
        assert_eq!(Solution::generate_key(1, 10, 1000), 0);
        assert_eq!(Solution::generate_key(987, 879, 798), 777);
        assert_eq!(Solution::generate_key(1, 2, 3), 1);
    }
}
