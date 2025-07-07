// https://leetcode.com/problems/building-boxes/
// 1739. Building Boxes
pub struct Solution;
impl Solution {
    fn search<F>(mut l: i64, mut r: i64, mut f: F) -> i64
    where
        F: FnMut(i64) -> bool,
    {
        if f(r) {
            return r;
        }
        if !f(l) {
            return l;
        }
        while l + 1 < r {
            let mid = (l + r) / 2;
            if f(mid) {
                l = mid;
            } else {
                r = mid;
            }
        }
        r
    }
    pub fn minimum_boxes(n: i32) -> i32 {
        let b1 = Self::search(1, (n as f64).sqrt() as i64 + 1, |x| {
            x * (x + 1) * (x + 2) / 6 <= n as i64
        }) - 1;
        let n_left = n as i64 - (b1 * (b1 + 1) * (b1 + 2) / 6);
        let nb = b1 * (b1 + 1) / 2;
        if n_left == 0 {
            nb as i32
        } else {
            let b2 = Self::search(1, n_left, |x| x * (x + 1) / 2 < n_left);
            (nb + b2) as i32
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_minimum_boxes() {
        assert_eq!(Solution::minimum_boxes(37910271), 186146);
        assert_eq!(Solution::minimum_boxes(3), 3);
        assert_eq!(Solution::minimum_boxes(4), 3);
        assert_eq!(Solution::minimum_boxes(10), 6);
    }
}
