// https://leetcode.com/problems/unique-binary-search-trees/
// 96. Unique Binary Search Trees
pub struct Solution;
impl Solution {
    pub fn num_trees(n: i32) -> i32 {
        let mut v = [0; 20];
        v[0] = 1;
        v[1] = 1;
        for i in 2..=n as usize {
            for j in 0..i {
                v[i] += v[j] * v[i - j - 1];
            }
        }
        v[n as usize]
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn num_trees() {
        assert_eq!(Solution::num_trees(3), 5);
        assert_eq!(Solution::num_trees(1), 1);
        assert_eq!(Solution::num_trees(2), 2);
    }
}
