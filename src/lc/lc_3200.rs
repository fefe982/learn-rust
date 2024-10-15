// https://leetcode.com/problems/maximum-height-of-a-triangle/
// 3200. Maximum Height of a Triangle
pub struct Solution;
impl Solution {
    fn get_rb(n: i32) -> (i32, i32) {
        let eve = n / 2;
        let odd = n - eve;
        let eve_cnt = eve * (eve + 1);
        let odd_cnt = odd * odd;
        if eve_cnt > odd_cnt {
            (odd_cnt, eve_cnt)
        } else {
            (eve_cnt, odd_cnt)
        }
    }
    pub fn max_height_of_triangle(red: i32, blue: i32) -> i32 {
        let rb = if red > blue { (blue, red) } else { (red, blue) };
        let mut low = 1;
        let mut high = ((2 * (red + blue)) as f64).sqrt().ceil() as i32;
        while low + 1 < high {
            let mid = (low + high) / 2;
            let rbm = Self::get_rb(mid);
            if rbm.0 <= rb.0 && rbm.1 <= rb.1 {
                low = mid;
            } else {
                high = mid;
            }
        }
        low
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_max_height_of_triangle() {
        assert_eq!(Solution::max_height_of_triangle(58, 56), 14);
        assert_eq!(Solution::max_height_of_triangle(2, 4), 3);
        assert_eq!(Solution::max_height_of_triangle(2, 1), 2);
        assert_eq!(Solution::max_height_of_triangle(1, 1), 1);
        assert_eq!(Solution::max_height_of_triangle(10, 1), 2);
    }
}
