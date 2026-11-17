// https://leetcode.com/problems/minimum-time-to-complete-all-deliveries/
// 3733. Minimum Time to Complete All Deliveries
pub struct Solution;
impl Solution {
    pub fn minimum_time(d: Vec<i32>, r: Vec<i32>) -> i64 {
        fn base(n: i64, r: i64) -> i64 {
            let c = n / (r - 1) * r;
            let q = n % (r - 1);
            if q != 0 {
                c + q
            } else {
                c - 1
            }
        }
        let d0 = d[0] as i64;
        let d1 = d[1] as i64;
        let r0 = r[0] as i64;
        let r1 = r[1] as i64;
        let c0 = base(d0, r0);
        let c1 = base(d1, r1);
        let mut low = c0.max(c1);
        let mut high = c0 + c1 + r0.max(r1);
        fn gcd(a: i64, b: i64) -> i64 {
            if b == 0 {
                a
            } else {
                gcd(b, a % b)
            }
        }
        let rg = r0 / gcd(r0, r1) * r1;
        let check = |len: i64| -> bool {
            let charge0 = len / r0;
            let charge1 = len / r1;
            let charge01 = len / rg;
            let s01 = len - charge0 - charge1 + charge01;
            let s0 = len - charge0 - s01;
            let s1 = len - charge1 - s01;
            (d0 - s0).max(0) + (d1 - s1).max(0) <= s01
        };
        if check(low) {
            return low;
        }
        while low + 1 < high {
            let mid = (low + high) / 2;
            if check(mid) {
                high = mid;
            } else {
                low = mid;
            }
        }
        high
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn minimum_time() {
        assert_eq!(Solution::minimum_time(vec![3, 1], vec![2, 3]), 5);
        assert_eq!(Solution::minimum_time(vec![1, 3], vec![2, 2]), 7);
        assert_eq!(Solution::minimum_time(vec![2, 1], vec![3, 4]), 3);
    }
}
