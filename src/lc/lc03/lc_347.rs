// https://leetcode.com/problems/top-k-frequent-elements/
// 347. Top K Frequent Elements
pub struct Solution;
impl Solution {
    pub fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let mut cnt = std::collections::HashMap::<i32, i32>::new();
        for n in nums {
            *cnt.entry(n).or_default() += 1;
        }
        let mut q = std::collections::BinaryHeap::new();
        for (n, c) in cnt {
            q.push((c, n));
        }
        let mut res = Vec::new();
        for _ in 0..k {
            res.push(q.pop().unwrap().1)
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn top_k_frequent() {
        assert_eq!(
            Solution::top_k_frequent(vec![1, 1, 1, 2, 2, 3], 2),
            vec![1, 2]
        );
        assert_eq!(Solution::top_k_frequent(vec![1], 1), vec![1]);
    }
}
