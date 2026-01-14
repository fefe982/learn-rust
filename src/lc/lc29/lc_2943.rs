// https://leetcode.com/problems/maximize-area-of-square-hole-in-grid/
// 2943. Maximize the Total Beauty of the Garden
pub struct Solution;
impl Solution {
    pub fn maximize_square_hole_area(_n: i32, _m: i32, h_bars: Vec<i32>, v_bars: Vec<i32>) -> i32 {
        let mut h_bars = h_bars;
        let mut v_bars = v_bars;
        h_bars.sort();
        v_bars.sort();
        fn cnt(bars: &Vec<i32>) -> i32 {
            let mut cnt = 0;
            let mut max = 1;
            let mut last = -1;
            for &bar in bars {
                if bar == last + 1 {
                    cnt += 1;
                    max = max.max(cnt);
                } else {
                    cnt = 1;
                }
                last = bar;
            }
            max
        }
        let s = cnt(&h_bars).min(cnt(&v_bars));
        (s + 1) * (s + 1)
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn maximize_square_hole_area() {
        assert_eq!(Solution::maximize_square_hole_area(2, 1, vec![2, 3], vec![2]), 4);
        assert_eq!(Solution::maximize_square_hole_area(1, 1, vec![2], vec![2]), 4);
        assert_eq!(Solution::maximize_square_hole_area(2, 3, vec![2, 3], vec![2, 4]), 4);
    }
}
