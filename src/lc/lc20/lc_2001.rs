// https://leetcode.com/problems/number-of-pairs-of-interchangeable-rectangles/
// 2001. Number of Pairs of Interchangeable Rectangles
pub struct Solution;
impl Solution {
    pub fn interchangeable_rectangles(rectangles: Vec<Vec<i32>>) -> i64 {
        use std::collections::HashMap;
        let mut map = HashMap::new();
        let gcd = |mut a: i32, mut b: i32| {
            while b != 0 {
                let t = a % b;
                a = b;
                b = t;
            }
            a
        };
        for rect in rectangles {
            let g = gcd(rect[0], rect[1]);
            let ratio = (rect[0] / g, rect[1] / g);
            *map.entry(ratio).or_insert(0) += 1;
        }
        let mut ans = 0;
        for &count in map.values() {
            ans += count * (count - 1) / 2;
        }
        ans
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn test_interchangeable_rectangles() {
        assert_eq!(
            Solution::interchangeable_rectangles(vec_vec![[4, 8], [3, 6], [10, 20], [15, 30]]),
            6
        );
        assert_eq!(Solution::interchangeable_rectangles(vec_vec![[4, 5], [7, 8]]), 0);
    }
}
