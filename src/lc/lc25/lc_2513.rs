// https://leetcode.com/problems/minimize-the-maximum-of-two-arrays/
// 2513. Minimize the Maximum of Two Arrays
pub struct Solution;
impl Solution {
    pub fn minimize_set(divisor1: i32, divisor2: i32, unique_cnt1: i32, unique_cnt2: i32) -> i32 {
        let gcd = |mut a: i64, mut b: i64| -> i64 {
            loop {
                if b == 0 {
                    return a;
                }
                let t = b;
                b = a % b;
                a = t;
            }
        };
        let divisor1 = divisor1 as i64;
        let divisor2 = divisor2 as i64;
        let divisor = divisor1 * divisor2 / gcd(divisor1, divisor2);
        let unique_cnt1 = unique_cnt1 as i64;
        let unique_cnt2 = unique_cnt2 as i64;
        let check = |x: i64| -> bool {
            let d = x / divisor;
            let d1 = x / divisor1 - d;
            let d2 = x / divisor2 - d;
            let nd = x - d1 - d2 - d;
            nd >= (unique_cnt1 - d2).max(0) + (unique_cnt2 - d1).max(0)
        };
        let mut min = unique_cnt1 + unique_cnt2;
        if check(min) {
            return min as i32;
        }
        let mut max = min * 2;
        while min + 1 < max {
            let mid = (min + max) / 2;
            if check(mid) {
                max = mid;
            } else {
                min = mid;
            }
        }
        max as i32
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn minimize_set() {
        assert_eq!(Solution::minimize_set(2, 7, 1, 3), 4);
        assert_eq!(Solution::minimize_set(3, 5, 2, 1), 3);
        assert_eq!(Solution::minimize_set(2, 4, 8, 2), 15);
    }
}
