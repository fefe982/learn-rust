// https://leetcode.com/problems/count-the-number-of-computer-unlocking-permutations/
// 3577. Count the Number of Computer Unlocking Permutations
pub struct Solution;
impl Solution {
    pub fn count_permutations(complexity: Vec<i32>) -> i32 {
        let mut ans = 1;
        let c = complexity[0];
        for i in 1..complexity.len() {
            if complexity[i] <= c {
                return 0;
            }
            ans = ans * i as i64 % 1_000_000_007;
        }
        ans as i32
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn count_permutations() {
        assert_eq!(Solution::count_permutations(vec![1, 2, 3]), 2);
        assert_eq!(Solution::count_permutations(vec![3, 3, 3, 4, 4, 4]), 0);
    }
}
