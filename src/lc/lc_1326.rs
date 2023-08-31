// https://leetcode.com/problems/minimum-number-of-taps-to-open-to-water-a-garden/description/
// 1326. Minimum Number of Taps to Open to Water a Garden
pub struct Solution;
impl Solution {
    pub fn min_taps(n: i32, ranges: Vec<i32>) -> i32 {
        let mut rng = vec![0; ranges.len()];
        for (rn, i) in ranges.into_iter().zip(0..) {
            let min = (i - rn).max(0) as usize;
            rng[min] = rng[min].max(i + rn);
        }
        let mut reach = 0;
        let mut i = 0;
        let mut e = 0;
        let mut cnt = 0;
        loop {
            cnt += 1;
            while i < rng.len() && i <= e {
                reach = reach.max(rng[i]);
                i += 1;
            }
            e = reach as usize;
            if reach >= n {
                return cnt;
            }
            if i > reach as usize {
                return -1;
            }
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn min_taps() {
        assert_eq!(Solution::min_taps(5, vec![3, 4, 1, 1, 0, 0]), 1);
        assert_eq!(Solution::min_taps(3, vec![0, 0, 0, 0]), -1);
    }
}
