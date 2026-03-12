// https://leetcode.com/problems/minimum-number-of-seconds-to-make-mountain-height-zero/
// 3296. Minimum Number of Seconds to Make Mountain Height Zero
pub struct Solution;
impl Solution {
    pub fn min_number_of_seconds(mountain_height: i32, worker_times: Vec<i32>) -> i64 {
        let h = mountain_height as i64;
        let maxt = *worker_times.iter().max().unwrap() as i64;
        let mut max = h * (h + 1) / 2 * maxt;
        let check = |wt: i64| {
            let mut mt = 0;
            for &t in &worker_times {
                let t = t as i64;
                let k = wt / t;
                let mut n = ((k * 2) as f64).sqrt() as i64;
                if (n + 1) * n / 2 > k {
                    n -= 1;
                }
                mt += n;
            }
            mt >= h
        };
        if check(1) {
            return 1;
        }
        let mut min = 1;
        while min + 1 < max {
            let mid = (min + max) / 2;
            if check(mid) {
                max = mid;
            } else {
                min = mid;
            }
        }
        max
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn min_number_of_seconds() {
        assert_eq!(Solution::min_number_of_seconds(4, vec![2, 1, 1]), 3);
        assert_eq!(Solution::min_number_of_seconds(10, vec![3, 2, 2, 4]), 12);
    }
}
