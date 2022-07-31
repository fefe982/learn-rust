// https://leetcode.com/problems/minimum-suffix-flips/
// 1529. Minimum Suffix Flips
pub struct Solution;
impl Solution {
    pub fn min_flips(target: String) -> i32 {
        let mut last = '0' as u8;
        let mut flip = 0;
        for &ch in target.as_bytes() {
            if ch != last {
                last = ch;
                flip += 1;
            }
        }
        flip
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn min_flips() {
        assert_eq!(Solution::min_flips("10111".to_owned()), 3);
        assert_eq!(Solution::min_flips("101".to_owned()), 3);
        assert_eq!(Solution::min_flips("0000000".to_owned()), 0);
    }
}
