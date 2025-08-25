// https://leetcode.com/problems/count-the-number-of-houses-at-a-certain-distance-ii/
// 3017. Count the Number of Houses at a Certain Distance II
pub struct Solution;
impl Solution {
    pub fn count_of_pairs(n: i32, x: i32, y: i32) -> Vec<i64> {
        let (x, y) = if x > y {
            (y as usize, x as usize)
        } else {
            (x as usize, y as usize)
        };
        let n = n as usize;
        let mut res = vec![0; n];
        let lp = y - x + 1;
        let mut len = n;
        if lp > 2 {
            for i in 0..lp / 2 - 1 {
                res[i] += lp as i64 * 2;
                res[i + 1] += (i + 1).min(x - 1) as i64 * 4 + (i + 1).min(n - y) as i64 * 4;
                if x - 1 >= i + 1 {
                    res[i + 1] -= 2;
                }
                if n - y >= i + 1 {
                    res[i + 1] -= 2;
                }
            }
            let mut m = 0;
            if lp % 2 == 0 {
                res[lp / 2 - 1] += lp as i64;
                res[lp / 2] +=
                    ((lp / 2).min(x - 1) as i64 * 4 - 2).max(0) + ((lp / 2).min(n - y) as i64 * 4 - 2).max(0);
                m = 2;
            } else {
                res[lp / 2 - 1] += lp as i64 * 2;
                res[lp / 2] += (lp / 2).min(x - 1) as i64 * 4 + (lp / 2).min(n - y) as i64 * 4;
            }
            if x - 1 >= lp / 2 {
                res[lp / 2] -= 2;
            }
            if n - y >= lp / 2 {
                res[lp / 2] -= 2;
            }
            for i in 2..x {
                if x - i >= lp / 2 {
                    res[lp / 2 + i - 1] += (lp / 2) as i64 * 4 - 2 - m;
                } else {
                    res[lp / 2 + i - 1] += (x - i) as i64 * 4 - m;
                }
            }
            for i in 2..n - y + 1 {
                if n - y + 1 - i >= lp / 2 {
                    res[lp / 2 + i - 1] += (lp / 2).min(n - y + 1 - i) as i64 * 4 - 2 - m;
                } else {
                    res[lp / 2 + i - 1] += (lp / 2).min(n - y + 1 - i) as i64 * 4 - m;
                }
            }
            res[0] -= 2;
            len = n + 2 - lp;
        }
        for i in 0..len - 1 {
            res[i] += (len - 1 - i) as i64 * 2;
        }
        res
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_count_of_pairs() {
        assert_eq!(Solution::count_of_pairs(6, 1, 5), vec![12, 14, 4, 0, 0, 0]);
        assert_eq!(Solution::count_of_pairs(6, 1, 4), vec![12, 10, 6, 2, 0, 0]);
        assert_eq!(Solution::count_of_pairs(5, 2, 5), vec![10, 8, 2, 0, 0]);
        assert_eq!(Solution::count_of_pairs(5, 1, 3), vec![10, 6, 4, 0, 0]);
        assert_eq!(Solution::count_of_pairs(4, 1, 4), vec![8, 4, 0, 0]);
        assert_eq!(Solution::count_of_pairs(2, 1, 2), vec![2, 0]);
        assert_eq!(Solution::count_of_pairs(3, 1, 3), vec![6, 0, 0]);
        assert_eq!(Solution::count_of_pairs(5, 2, 4), vec![10, 8, 2, 0, 0]);
        assert_eq!(Solution::count_of_pairs(4, 1, 1), vec![6, 4, 2, 0]);
    }
}
