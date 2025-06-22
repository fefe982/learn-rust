// https://leetcode.com/problems/number-of-longest-increasing-subsequence/
// 673. Number of Longest Increasing Subsequence
pub struct Solution;
impl Solution {
    pub fn find_number_of_lis(nums: Vec<i32>) -> i32 {
        let mut m = std::collections::BTreeMap::new();
        m.insert(i32::MIN, 1);
        let mut q = vec![m];
        for n in nums {
            let idx = q.partition_point(|x| *x.iter().next().unwrap().0 < n);
            let c: i32 = q[idx - 1].range(..n).map(|(_, &c)| c).sum();
            if idx >= q.len() {
                let mut m = std::collections::BTreeMap::new();
                m.insert(n, c);
                q.push(m);
            } else {
                *q[idx].entry(n).or_default() += c;
            }
        }
        q.last().unwrap().values().sum()
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn find_number_of_lis() {
        assert_eq!(
            Solution::find_number_of_lis(vec![1, 2, 4, 3, 5, 4, 7, 2]),
            3
        );
        assert_eq!(Solution::find_number_of_lis(vec![1, 3, 5, 4, 7]), 2);
        assert_eq!(Solution::find_number_of_lis(vec![2, 2, 2, 2, 2]), 5);
    }
}
