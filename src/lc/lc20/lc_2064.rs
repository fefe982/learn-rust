// https://leetcode.com/problems/minimized-maximum-of-products-distributed-to-any-store/
// 2064. Minimized Maximum of Products Distributed to Any Store
pub struct Solution;
impl Solution {
    fn distribute(n: i32, quantities: &Vec<i32>, max: i32) -> bool {
        let mut sum = 0;
        for &q in quantities {
            sum += (q + max - 1) / max;
            if sum > n {
                return false;
            }
        }
        true
    }
    pub fn minimized_maximum(n: i32, quantities: Vec<i32>) -> i32 {
        let mut min = 1;
        let mut max = 100000;
        if Self::distribute(n, &quantities, min) {
            return min;
        }
        while min + 1 < max {
            let mid = (min + max) / 2;
            if Self::distribute(n, &quantities, mid) {
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
    fn test_minimized_maximum() {
        assert_eq!(Solution::minimized_maximum(6, vec![11, 6]), 3);
        assert_eq!(Solution::minimized_maximum(7, vec![15, 10, 10]), 5);
        assert_eq!(Solution::minimized_maximum(1, vec![100000]), 100000);
    }
}
