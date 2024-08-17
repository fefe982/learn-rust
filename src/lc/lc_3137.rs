// https://leetcode.com/problems/minimum-number-of-operations-to-make-word-k-periodic/
// 3137. Minimum Number of Operations to Make Word K-Periodic
pub struct Solution;
impl Solution {
    pub fn minimum_operations_to_make_k_periodic(word: String, k: i32) -> i32 {
        let mut cnt = std::collections::HashMap::<Vec<u8>, i32>::new();
        let word = word.as_bytes();
        let n = word.len();
        let k = k as usize;
        let c = n / k;
        let mut max = 1;
        for i in 0..c {
            let v = cnt.entry(word[i * k..(i + 1) * k].to_vec()).or_default();
            *v += 1;
            max = max.max(*v);
        }
        c as i32 - max
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn minimum_operations_to_make_k_periodic() {
        assert_eq!(
            Solution::minimum_operations_to_make_k_periodic("leetcodeleet".to_string(), 4),
            1
        );
        assert_eq!(
            Solution::minimum_operations_to_make_k_periodic("leetcoleet".to_string(), 2),
            3
        );
    }
}
