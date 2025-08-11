// https://leetcode.com/problems/maximum-sum-queries/
// 2736. Maximum Sum Queries
pub struct Solution;
impl Solution {
    pub fn maximum_sum_queries(nums1: Vec<i32>, nums2: Vec<i32>, queries: Vec<Vec<i32>>) -> Vec<i32> {
        let mut nums = nums1.into_iter().zip(nums2.into_iter()).collect::<Vec<_>>();
        nums.sort_unstable_by_key(|&(a, b)| std::cmp::Reverse((a, b)));
        let mut queries = queries
            .into_iter()
            .enumerate()
            .map(|(i, q)| (std::cmp::Reverse((q[0], q[1])), i))
            .collect::<Vec<_>>();
        queries.sort();
        let mut res = vec![0; queries.len()];
        let mut nidx = 0;
        let mut stack = vec![];
        for (qr, i) in queries {
            let q = qr.0;
            while nidx < nums.len() && nums[nidx].0 >= q.0 {
                let mut push = true;
                while let Some(&(tx, ty)) = stack.last() {
                    if nums[nidx].0 + nums[nidx].1 >= tx + ty {
                        stack.pop();
                    } else {
                        break;
                    }
                }
                if let Some(&(_, ty)) = stack.last() {
                    if nums[nidx].1 < ty {
                        push = false;
                    }
                }
                if push {
                    stack.push(nums[nidx]);
                }
                nidx += 1;
            }
            let yidx = stack.partition_point(|&(_, ty)| ty < q.1);
            if yidx == stack.len() {
                res[i] = -1;
            } else {
                res[i] = stack[yidx].0 + stack[yidx].1;
            }
        }
        res
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn test_maximum_sum_queries() {
        assert_eq!(
            Solution::maximum_sum_queries(vec![4, 3, 1, 2], vec![2, 4, 9, 5], vec_vec![[4, 1], [1, 3], [2, 5]]),
            vec![6, 10, 7]
        );
        assert_eq!(
            Solution::maximum_sum_queries(vec![3, 2, 5], vec![2, 3, 4], vec_vec![[4, 4], [3, 2], [1, 1]]),
            vec![9, 9, 9]
        );
        assert_eq!(
            Solution::maximum_sum_queries(vec![2, 1], vec![2, 3], vec_vec![[3, 3]]),
            vec![-1]
        );
    }
}
