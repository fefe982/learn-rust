// https://leetcode.com/problems/construct-target-array-with-multiple-sums/
// 1354. Construct Target Array With Multiple Sums
pub struct Solution;
impl Solution {
    pub fn is_possible(target: Vec<i32>) -> bool {
        if target.len() == 1 {
            return target[0] == 1;
        }
        let mut q = std::collections::BinaryHeap::new();
        let mut sum = 0;
        for t in target {
            sum += t as i64;
            q.push(t as i64);
        }
        while let Some(t) = q.pop() {
            if t == 1 {
                return true;
            }
            let nt = t % (sum - t);
            if nt == 0 {
                if sum - t == 1 {
                    return true;
                }
                return false;
            }
            if nt == t {
                return false;
            }
            sum -= t - nt;
            q.push(nt);
        }
        unreachable!()
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_is_possible() {
        assert_eq!(Solution::is_possible(vec![2]), false);
        assert_eq!(Solution::is_possible(vec![1, 1000000000]), true);
        assert_eq!(Solution::is_possible(vec![9, 3, 5]), true);
        assert_eq!(Solution::is_possible(vec![1, 1, 1, 2]), false);
        assert_eq!(Solution::is_possible(vec![8, 5]), true);
    }
}
