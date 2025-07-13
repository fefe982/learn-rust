// https://leetcode.com/problems/mirror-reflection/
// 858. Mirror Reflection
pub struct Solution;
impl Solution {
    pub fn mirror_reflection(p: i32, q: i32) -> i32 {
        let mut a = p;
        let mut b = q;
        let gcd = loop {
            if a % b == 0 {
                break b;
            }
            a = a % b;
            if b % a == 0 {
                break a;
            }
            b = b % a;
        };
        let lcm = (p * q) / gcd;
        if lcm / p % 2 == 0 {
            0
        } else if lcm / q % 2 == 0 {
            2
        } else {
            1
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn mirror_reflection() {
        assert_eq!(Solution::mirror_reflection(2, 1), 2);
        assert_eq!(Solution::mirror_reflection(3, 1), 1);
    }
}
