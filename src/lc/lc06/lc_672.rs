// https://leetcode.com/problems/bulb-switcher-ii/
// 672. Bulb Switcher II
pub struct Solution;
impl Solution {
    pub fn flip_lights(n: i32, presses: i32) -> i32 {
        let mut n = n;
        if n > 4 {
            n = 4;
        }
        let mut lt = 1i32;
        let bt = [0b1111, 0b1010, 0b0101, 0b1001];
        let mask = (1 << n) - 1;
        let all = 1 << n;
        for _ in 0..presses {
            let mut nlt = 0;
            for i in 0..all {
                if lt & (1 << i) == 0 {
                    continue;
                }
                for &b in &bt {
                    let ni = (i ^ b) & mask;
                    nlt |= 1 << ni;
                }
            }
            if nlt == lt {
                break;
            }
            lt = nlt;
        }
        lt.count_ones() as i32
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn flip_lights() {
        assert_eq!(Solution::flip_lights(1, 1), 2);
        assert_eq!(Solution::flip_lights(2, 1), 3);
        assert_eq!(Solution::flip_lights(3, 1), 4);
    }
}
