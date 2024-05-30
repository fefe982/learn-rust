// https://leetcode.com/problems/count-triplets-that-can-form-two-arrays-of-equal-xor/
// 1442. Count Triplets That Can Form Two Arrays of Equal XOR
pub struct Solution;
impl Solution {
    pub fn count_triplets(arr: Vec<i32>) -> i32 {
        let mut m = std::collections::HashMap::<i32, (usize, usize)>::new();
        m.insert(0, (1, 0));
        let mut xor = 0;
        let mut ans = 0;
        for (a, i) in arr.into_iter().zip(1..) {
            xor ^= a;
            if let Some(&(cnt, sz)) = m.get(&xor) {
                ans += cnt * (i - 1) - sz;
                m.insert(xor, (cnt + 1, sz + i));
            } else {
                m.insert(xor, (1, i));
            }
        }
        ans as i32
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_count_triplets() {
        assert_eq!(Solution::count_triplets(vec![2, 3, 1, 6, 7]), 4);
        assert_eq!(Solution::count_triplets(vec![1, 1, 1, 1, 1]), 10);
    }
}
