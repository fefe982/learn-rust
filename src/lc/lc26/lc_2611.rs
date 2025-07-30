// https://leetcode.com/problems/mice-and-cheese/
// 2611. Mice and Cheese
pub struct Solution;
impl Solution {
    fn search(reward1: Vec<i32>, reward2: Vec<i32>, k: usize) -> i32 {
        let mut q = std::collections::BinaryHeap::<i32>::new();
        let mut sum = 0;
        for (&r1, &r2) in reward1.iter().zip(reward2.iter()) {
            let diff = r1 - r2;
            if q.len() == k && k > 0 && -*q.peek().unwrap() < diff {
                q.pop();
            }
            if q.len() < k {
                q.push(-diff);
            }
            sum += r2;
        }
        while let Some(r) = q.pop() {
            sum += -r;
        }
        sum
    }
    pub fn mice_and_cheese(reward1: Vec<i32>, reward2: Vec<i32>, k: i32) -> i32 {
        let l = reward1.len();
        let k = k as usize;
        if k <= l - k {
            Self::search(reward1, reward2, k)
        } else {
            Self::search(reward2, reward1, l - k)
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn mice_and_cheese() {
        assert_eq!(
            Solution::mice_and_cheese(vec![1, 1, 3, 4], vec![4, 4, 1, 1], 2),
            15
        );
        assert_eq!(Solution::mice_and_cheese(vec![1, 1], vec![1, 1], 2), 2);
    }
}
