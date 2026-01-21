// https://leetcode.com/problems/minimum-pair-removal-to-sort-array-ii/
// 3510. Minimum Pair Removal to Sort the Array II
pub struct Solution;
impl Solution {
    pub fn minimum_pair_removal(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let mut next = vec![0; n];
        next[n - 1] = usize::MAX;
        let mut prev = vec![0; n];
        prev[0] = usize::MAX;
        let mut nums = nums.into_iter().map(|x| x as i64).collect::<Vec<_>>();
        let mut q = std::collections::BinaryHeap::new();
        let mut dec = 0;
        let mut cnt = 0;
        for i in 0..n - 1 {
            q.push(std::cmp::Reverse((nums[i] + nums[i + 1], i)));
            next[i] = i + 1;
            prev[i + 1] = i;
            if nums[i] > nums[i + 1] {
                dec += 1;
            }
        }
        while dec > 0 {
            let Some(std::cmp::Reverse((x, i))) = q.pop() else {
                break;
            };
            if next[i] == usize::MAX || nums[i] + nums[next[i]] != x {
                continue;
            }
            cnt += 1;
            if nums[i] > nums[next[i]] {
                dec -= 1;
            }
            if prev[i] != usize::MAX && nums[prev[i]] > nums[i] {
                dec -= 1;
            }
            if next[next[i]] != usize::MAX && nums[next[i]] > nums[next[next[i]]] {
                dec -= 1;
            }
            nums[i] += nums[next[i]];
            let ni = next[i];
            next[i] = next[ni];
            if next[i] != usize::MAX {
                prev[next[i]] = i;
                if nums[i] > nums[next[i]] {
                    dec += 1;
                }
                q.push(std::cmp::Reverse((nums[i] + nums[next[i]], i)));
                next[ni] = usize::MAX;
            }
            if prev[i] != usize::MAX {
                if nums[prev[i]] > nums[i] {
                    dec += 1;
                }
                q.push(std::cmp::Reverse((nums[prev[i]] + nums[i], prev[i])));
            }
        }
        cnt
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn minimum_pair_removal() {
        assert_eq!(Solution::minimum_pair_removal(vec![5, 2, 3, 1]), 2);
        assert_eq!(Solution::minimum_pair_removal(vec![1, 2, 2]), 0);
    }
}
