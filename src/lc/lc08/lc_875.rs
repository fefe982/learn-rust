// https://leetcode.com/problems/koko-eating-bananas/solutions/
// 875. Koko Eating Bananas
pub struct Solution;
impl Solution {
    fn count(piles: &Vec<i32>, m: i32) -> i32 {
        let mut h = 0;
        for &p in piles.iter() {
            h += (p - 1) / m + 1;
        }
        h
    }
    pub fn min_eating_speed(piles: Vec<i32>, h: i32) -> i32 {
        let mut sum: i64 = 0;
        let mut max: i32 = 0;
        for &p in piles.iter() {
            sum += p as i64;
            if p > max {
                max = p;
            }
        }
        if h as usize == piles.len() {
            return max;
        }
        let mut low = (sum / h as i64) as i32;
        if low == 0 {
            return 1;
        }
        let mut high = max;
        if Self::count(&piles, low) <= h {
            return low;
        }
        while low + 1 < high {
            let mid = (low + high) / 2;
            if Self::count(&piles, mid) <= h {
                high = mid;
            } else {
                low = mid;
            }
        }
        high
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn min_eating_speed() {
        assert_eq!(Solution::min_eating_speed(vec![3, 6, 7, 11], 8), 4);
        assert_eq!(Solution::min_eating_speed(vec![30, 11, 23, 4, 20], 5), 30);
        assert_eq!(Solution::min_eating_speed(vec![30, 11, 23, 4, 20], 6), 23);
    }
}
