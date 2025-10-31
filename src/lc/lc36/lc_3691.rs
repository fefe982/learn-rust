// https://leetcode.com/problems/maximum-total-subarray-value-ii/
// 3691. Maximum Total Subarray Value II
pub struct Solution;
impl Solution {
    pub fn max_total_value(nums: Vec<i32>, k: i32) -> i64 {
        let len = nums.len();
        let mut next_g = vec![len as i32; len];
        let mut next_l = vec![len as i32; len];
        let mut prev_g = vec![-1; len];
        let mut prev_l = vec![-1; len];
        let mut stack_g: Vec<usize> = vec![];
        let mut stack_l: Vec<usize> = vec![];
        for i in 0..len {
            while let Some(&top) = stack_g.last() {
                if nums[top] <= nums[i] {
                    next_g[top] = i as i32;
                    stack_g.pop();
                } else {
                    break;
                }
            }
            if let Some(&top) = stack_g.last() {
                prev_g[i] = top as i32;
            }
            stack_g.push(i);
            while let Some(&top) = stack_l.last() {
                if nums[top] > nums[i] {
                    next_l[top] = i as i32;
                    stack_l.pop();
                } else {
                    break;
                }
            }
            if let Some(&top) = stack_l.last() {
                prev_l[i] = top as i32;
            }
            stack_l.push(i);
        }
        let mut nums = nums
            .into_iter()
            .enumerate()
            .map(|(i, v)| (v, i))
            .collect::<Vec<(i32, usize)>>();
        nums.sort();
        let mut q = std::collections::BinaryHeap::new();
        q.push((nums[len - 1].0 - nums[0].0, 0, len - 1));
        let mut ans = 0;
        let mut k = k;
        while let Some((v, l, r)) = q.pop() {
            if v == 0 {
                break;
            }
            let il = nums[l].1;
            let ir = nums[r].1;
            let nr = next_g[ir].min(next_l[il]);
            let nl = prev_g[ir].max(prev_l[il]);
            let rr = ir.max(il) as i32;
            let rl = ir.min(il) as i32;
            let nn = (nr - rr).max(0) * (rl - nl).max(0);
            let take = nn.min(k);
            ans += take as i64 * v as i64;
            k -= take;
            if k <= 0 {
                break;
            }
            if l == 0 {
                q.push((nums[r - 1].0 - nums[l].0, l, r - 1))
            }
            q.push((nums[r].0 - nums[l + 1].0, l + 1, r));
        }
        ans
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn max_total_value() {
        assert_eq!(Solution::max_total_value(vec![1, 3, 2], 2), 4);
        assert_eq!(Solution::max_total_value(vec![4, 2, 5, 1], 3), 12);
    }
}
