// https://leetcode.com/problems/circular-permutation-in-binary-representation/
// 1238. Circular Permutation in Binary Representation
pub struct Solution;
impl Solution {
    pub fn circular_permutation(n: i32, start: i32) -> Vec<i32> {
        let mut ans = Vec::with_capacity(1 << n);
        fn push(ans: &mut Vec<i32>, n: i32, mut start: i32) -> i32 {
            if n == 0 {
                ans.push(start);
                return start;
            }
            start = push(ans, n - 1, start);
            push(ans, n - 1, start ^ (1 << (n - 1)))
        }
        push(&mut ans, n, start);
        ans
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    fn check(n: i32, start: i32) {
        let ans = Solution::circular_permutation(n, start);
        assert_eq!(ans.len(), 1 << n);
        assert_eq!(ans[0], start);
        for i in 0..ans.len() {
            let xor = ans[i] ^ ans[(i + 1) % ans.len()];
            assert_eq!(xor.count_ones(), 1);
        }
    }
    #[test]
    fn circular_permutation() {
        check(2, 3);
        check(3, 2);
    }
}
