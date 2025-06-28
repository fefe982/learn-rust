// https://leetcode.com/problems/maximum-area-of-a-piece-of-cake-after-horizontal-and-vertical-cuts/
// 1465. Maximum Area of a Piece of Cake After Horizontal and Vertical Cuts
pub struct Solution;
impl Solution {
    fn get_max(l: i32, mut c: Vec<i32>) -> i64 {
        c.sort();
        c.into_iter()
            .chain([l].into_iter())
            .fold((0, 0), |(max, last), cur| (max.max(cur - last), cur))
            .0 as i64
    }
    pub fn max_area(h: i32, w: i32, horizontal_cuts: Vec<i32>, vertical_cuts: Vec<i32>) -> i32 {
        (Solution::get_max(h, horizontal_cuts) * Solution::get_max(w, vertical_cuts) % 1000000007)
            as i32
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_max_area() {
        assert_eq!(Solution::max_area(5, 4, vec![1, 2, 4], vec![1, 3]), 4);
        assert_eq!(Solution::max_area(5, 4, vec![3, 1], vec![3]), 6);
        assert_eq!(Solution::max_area(5, 4, vec![3], vec![3]), 9);
    }
}
