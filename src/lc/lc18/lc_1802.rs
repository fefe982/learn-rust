// https://leetcode.com/problems/maximum-value-at-a-given-index-in-a-bounded-array/
// 1802. Maximum Value at a Given Index in a Bounded Array
pub struct Solution;
impl Solution {
    fn sum(s: i32, e: i32) -> i64 {
        let s = s as i64;
        let e = e as i64;
        (s + e) * (e - s + 1) / 2 + if s <= 0 { (2 - s) * (1 - s) / 2 } else { 0 }
    }
    fn low_value(n: i32, index: i32, val: i32) -> i64 {
        Self::sum(val - index, val) + Self::sum(val - n + index + 1, val) - val as i64
    }
    pub fn max_value(n: i32, index: i32, max_sum: i32) -> i32 {
        let mut low = max_sum / n;
        let mut high = low + std::cmp::max(index, n - index - 1) / 2;
        let max_sum = max_sum as i64;
        while Self::low_value(n, index, high) <= max_sum {
            low = high;
            high += 1;
        }
        while low + 1 < high {
            let mid = (low + high) / 2;
            if Self::low_value(n, index, mid) <= max_sum {
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
    fn max_value() {
        assert_eq!(Solution::max_value(8257285, 4828516, 850015631), 29014);
        assert_eq!(Solution::max_value(4, 2, 6), 2);
        assert_eq!(Solution::max_value(6, 1, 10), 3);
    }
}
