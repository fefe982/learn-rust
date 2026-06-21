// https://leetcode.com/problems/maximum-total-value/
// 3971. Maximum Total Value
pub struct Solution;
impl Solution {
    pub fn max_total_value(value: Vec<i32>, decay: Vec<i32>, m: i32) -> i32 {
        let check = |x: i32| -> bool {
            let mut total = 0;
            for (&v, &d) in value.iter().zip(decay.iter()) {
                if v >= x {
                    total += ((v - x) / d) + 1;
                    if total > m {
                        return false;
                    }
                }
            }
            return true;
        };
        let mut low = 0;
        if !check(0) {
            let mut l = 0;
            let mut r = value.iter().max().unwrap() + 1;
            while l + 1 < r {
                let mid = l + (r - l) / 2;
                if check(mid) {
                    r = mid;
                } else {
                    l = mid;
                }
            }
            low = l;
        }
        let mut total = 0;
        let mut m = m;
        for (&v, &d) in value.iter().zip(decay.iter()) {
            if v > low {
                let k = ((v - low - 1) / d) + 1;
                let vv = v as i64;
                let kk = k as i64;
                let dd = d as i64;
                total += (vv * 2 - dd * (kk - 1)) * kk;
                m -= k;
            }
        }
        total /= 2;
        total += (low as i64) * (m as i64);
        (total % 1000000007) as i32
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_max_total_value() {
        assert_eq!(Solution::max_total_value(vec![3, 7, 5], vec![4, 6, 3], 2), 12);
        assert_eq!(Solution::max_total_value(vec![6, 5, 4], vec![2, 1, 1], 4), 19);
        assert_eq!(Solution::max_total_value(vec![7, 2, 2], vec![3, 2, 1], 2), 11);
        assert_eq!(Solution::max_total_value(vec![4, 3], vec![5, 4], 5), 7);
    }
}
