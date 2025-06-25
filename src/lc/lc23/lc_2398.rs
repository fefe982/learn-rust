// https://leetcode.com/problems/maximum-number-of-robots-within-budget/
// 2398. Maximum Number of Robots Within Budget
pub struct Solution;
impl Solution {
    pub fn maximum_robots(charge_times: Vec<i32>, running_costs: Vec<i32>, budget: i64) -> i32 {
        let mut ans = 0;
        let mut i = 0;
        let mut j = 0;
        let mut sum = 0;
        let mut h = std::collections::BinaryHeap::new();
        let get_cost = |h: &mut std::collections::BinaryHeap<(i32, usize)>, i: usize, j: usize, sum: i64| {
            while let Some(&(top, itop)) = h.peek() {
                if itop < i {
                    h.pop();
                } else {
                    return top as i64 + (j - i) as i64 * sum;
                }
            }
            0
        };
        while j < charge_times.len() {
            while j < charge_times.len() {
                sum += running_costs[j] as i64;
                h.push((charge_times[j], j));
                j += 1;
                if get_cost(&mut h, i, j, sum) <= budget {
                    ans = ans.max(j - i);
                } else {
                    break;
                }
            }
            while i < j {
                sum -= running_costs[i] as i64;
                i += 1;
                if get_cost(&mut h, i, j, sum) <= budget {
                    ans = ans.max(j - i);
                    break;
                }
            }
        }
        ans as i32
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_maximum_robots() {
        assert_eq!(
            Solution::maximum_robots(vec![3, 6, 1, 3, 4], vec![2, 1, 3, 4, 5], 25),
            3
        );
        assert_eq!(Solution::maximum_robots(vec![11, 12, 19], vec![10, 8, 7], 19), 0);
    }
}
