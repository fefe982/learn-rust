// https://leetcode.com/problems/k-th-smallest-prime-fraction/
// 786. K-th Smallest Prime Fraction
pub struct Solution;
#[derive(Eq, Ord)]
struct Fraction(i32, i32);
impl std::cmp::PartialOrd for Fraction {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some((self.1 * other.0).cmp(&(self.0 * other.1)))
    }
}
impl std::cmp::PartialEq for Fraction {
    fn eq(&self, other: &Self) -> bool {
        self.0 * other.1 == self.1 * other.0
    }
}

impl Solution {
    pub fn kth_smallest_prime_fraction(arr: Vec<i32>, k: i32) -> Vec<i32> {
        let mut q = std::collections::BinaryHeap::new();
        for i in 1..arr.len() {
            q.push((Fraction(arr[0], arr[i]), 0usize));
        }
        for _ in 0..k - 1 {
            let (f, i) = q.pop().unwrap();
            if i + 1 < arr.len() && arr[i + 1] < f.1 {
                q.push((Fraction(arr[i + 1], f.1), i + 1));
            }
        }
        let f = q.pop().unwrap();
        vec![f.0 .0, f.0 .1]
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_kth_smallest_prime_fraction() {
        assert_eq!(Solution::kth_smallest_prime_fraction(vec![1, 2, 3, 5], 3), [2, 5]);
        assert_eq!(Solution::kth_smallest_prime_fraction(vec![1, 7], 1), [1, 7]);
    }
}
