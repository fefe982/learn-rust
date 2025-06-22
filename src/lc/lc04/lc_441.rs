// https://leetcode.com/problems/arranging-coins/
// 441. Arranging Coins
pub struct Solution;
impl Solution {
    pub fn arrange_coins(n: i32) -> i32 {
        if n >= 65535 * 32768 {
            return 65535;
        }
        let n = n as u32;
        let mut l = 1;
        let mut h = 65535;
        while l + 1 < h {
            let m = (l + h) / 2;
            if m * (m + 1) / 2 <= n {
                l = m;
            } else {
                h = m;
            }
        }
        l as i32
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn arrange_coins() {
        assert_eq!(Solution::arrange_coins(5), 2);
        assert_eq!(Solution::arrange_coins(8), 3);
    }
}
