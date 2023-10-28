// https://leetcode.com/problems/kth-smallest-number-in-multiplication-table/
// 668. Kth Smallest Number in Multiplication Table
pub struct Solution;
impl Solution {
    pub fn find_kth_number(m: i32, n: i32, k: i32) -> i32 {
        if k == 1 {
            return 1;
        }
        if k == m * n {
            return m * n;
        }
        let mut low = 1;
        let mut high = m * n;
        while low + 1 < high {
            let mid = (low + high) / 2;
            let mut count = 0;
            let mut i = 1;
            while i <= m {
                let c = (mid / i).min(n);
                if c == 0 {
                    break;
                }
                let ni = (mid / c).min(m);
                count += (ni - i + 1) * c;
                i = ni + 1;
            }
            if count < k {
                low = mid;
            } else {
                high = mid;
            }
        }
        high
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_find_kth_number() {
        assert_eq!(Solution::find_kth_number(45, 12, 471), 312);
        assert_eq!(Solution::find_kth_number(3, 3, 5), 3);
        assert_eq!(Solution::find_kth_number(2, 3, 6), 6);
    }
}
