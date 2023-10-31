// https://leetcode.com/problems/smallest-missing-genetic-value-in-each-subtree/
// 2003. Smallest Missing Genetic Value in Each Subtree
pub struct Solution;
impl Solution {
    pub fn smallest_missing_value_subtree(parents: Vec<i32>, nums: Vec<i32>) -> Vec<i32> {
        let mut children = vec![vec![]; nums.len()];
        let mut nums = nums;
        for (i, &p) in parents.iter().enumerate() {
            if p != -1 {
                children[p as usize].push(i);
            }
        }
        let mut h: std::collections::BinaryHeap<std::cmp::Reverse<i32>> =
            std::collections::BinaryHeap::new();
        let mut res = vec![1; nums.len()];
        let mut has_one = nums.len();
        for (i, &n) in nums.iter().enumerate() {
            if n == 1 {
                has_one = i;
            }
        }
        if has_one < nums.len() {
            let mut missing = 1;
            loop {
                h.push(std::cmp::Reverse(nums[has_one]));
                while let Some(&pp) = h.peek() {
                    if pp.0 == missing {
                        missing += 1;
                        h.pop();
                    } else {
                        break;
                    }
                }
                let mut q = vec![has_one];
                while let Some(qq) = q.pop() {
                    if nums[qq] == -1 {
                        continue;
                    }
                    for &i in &children[qq] {
                        if nums[i] == -1 {
                            continue;
                        }
                        q.push(i);
                        if nums[i] > missing {
                            h.push(std::cmp::Reverse(nums[i]));
                        } else {
                            missing += 1;
                            while let Some(&pp) = h.peek() {
                                if pp.0 == missing {
                                    missing += 1;
                                    h.pop();
                                } else {
                                    break;
                                }
                            }
                        }
                    }
                    nums[qq] = -1;
                }
                res[has_one] = missing;
                nums[has_one] = -1;
                if parents[has_one] == -1 {
                    break;
                }
                has_one = parents[has_one] as usize;
            }
        }
        res
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_smallest_missing_value_subtree() {
        assert_eq!(
            Solution::smallest_missing_value_subtree(vec![-1, 0, 0, 2], vec![1, 2, 3, 4]),
            vec![5, 1, 1, 1]
        );
        assert_eq!(
            Solution::smallest_missing_value_subtree(
                vec![-1, 0, 1, 0, 3, 3],
                vec![5, 4, 6, 2, 1, 3]
            ),
            vec![7, 1, 1, 4, 2, 1]
        );
        assert_eq!(
            Solution::smallest_missing_value_subtree(
                vec![-1, 2, 3, 0, 2, 4, 1],
                vec![2, 3, 4, 5, 6, 7, 8]
            ),
            vec![1, 1, 1, 1, 1, 1, 1]
        );
    }
}
