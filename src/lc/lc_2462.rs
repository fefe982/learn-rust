// https://leetcode.com/problems/total-cost-to-hire-k-workers/
// 2462. Total Cost to Hire K Workers
pub struct Solution;
impl Solution {
    pub fn total_cost(costs: Vec<i32>, k: i32, candidates: i32) -> i64 {
        let mut h = std::collections::BinaryHeap::new();
        let mut c = candidates as usize;
        let mut bc = (costs.len() - c).max(c);
        for i in 0..c {
            h.push(std::cmp::Reverse((costs[i], i)));
        }
        for i in bc..costs.len() {
            h.push(std::cmp::Reverse((costs[i], i)));
        }
        let mut sum = 0;
        for _ in 0..k {
            let std::cmp::Reverse((cost, i)) = h.pop().unwrap();
            sum += cost as i64;
            if c < bc {
                if i < c {
                    h.push(std::cmp::Reverse((costs[c], c)));
                    c += 1;
                } else {
                    bc -= 1;
                    h.push(std::cmp::Reverse((costs[bc], bc)));
                }
            }
        }
        sum
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn total_cost() {
        assert_eq!(Solution::total_cost(vec![17, 12, 10, 2, 7, 2, 11, 20, 8], 3, 4), 11);
        assert_eq!(Solution::total_cost(vec![1, 2, 4, 1], 3, 3), 4);
    }
}
