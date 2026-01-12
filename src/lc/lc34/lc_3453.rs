// https://leetcode.com/problems/separate-squares-i/
// 3453. Separate Squares I
pub struct Solution;
impl Solution {
    pub fn separate_squares(squares: Vec<Vec<i32>>) -> f64 {
        let mut low = i32::MAX;
        let mut high: i32 = i32::MIN;
        let mut total = 0;
        for square in &squares {
            low = low.min(square[1]);
            high = high.max(square[1] + square[2]);
            total += square[2] as i64 * square[2] as i64;
        }
        let get_area = |sep: i32| -> i64 {
            let mut t = 0;
            for square in &squares {
                if sep >= square[1] + square[2] {
                    t += square[2] as i64 * square[2] as i64;
                } else if sep > square[1] {
                    t += square[2] as i64 * (sep - square[1]) as i64;
                }
            }
            t
        };
        while low + 1 < high {
            let mid = (low + high) / 2;
            let area = get_area(mid);
            if area < (total + 1) / 2 {
                low = mid;
            } else {
                high = mid;
            }
        }
        let ahigh = get_area(high);
        if total % 2 == 0 && ahigh == total / 2 {
            return high as f64;
        }
        let alow = get_area(low);
        low as f64 + (total as f64 / 2.0 - alow as f64) / (ahigh - alow) as f64
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    use assert_approx_eq::assert_approx_eq;
    #[test]
    fn separate_squares() {
        assert_approx_eq!(Solution::separate_squares(vec_vec![[0, 0, 1], [2, 2, 1]]), 1.0, 1e-5);
        assert_approx_eq!(
            Solution::separate_squares(vec_vec![[0, 0, 2], [1, 1, 1]]),
            1.1666666666666667,
            1e-5
        );
    }
}
