// https://leetcode.com/problems/apply-operations-to-make-sum-of-array-greater-than-or-equal-to-k/description/?envType=problem-list-v2&envId=2sh4gvmh
// 3091. Apply Operations to Make Sum of Array Greater Than or Equal to K
pub struct Solution;
impl Solution {
    pub fn min_operations(k: i32) -> i32 {
        let mut n1 = (k as f64).sqrt() as i32;
        while n1 * n1 < k {
            n1 += 1;
        }
        let mut n2 = n1;
        while (n2 - 1) * n1 >= k {
            n2 -= 1;
        }
        n1 + n2 - 2
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn min_operations() {
        assert_eq!(Solution::min_operations(11), 5);
        assert_eq!(Solution::min_operations(1), 0);
    }
}
