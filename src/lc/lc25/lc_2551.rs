// https://leetcode.com/problems/put-marbles-in-bags/
// 2551. Put Marbles in Bags
pub struct Solution;
impl Solution {
    pub fn put_marbles(weights: Vec<i32>, k: i32) -> i64 {
        let mut k = k as usize - 1;
        let l = weights.len() - 1;
        k = k.min(l - k);
        if k == 0 {
            return 0;
        }
        let mut maxh: std::collections::BinaryHeap<std::cmp::Reverse<i32>> =
            std::collections::BinaryHeap::new();
        let mut minh = std::collections::BinaryHeap::new();
        for i in 0..l {
            let s = weights[i] + weights[i + 1];
            if minh.len() >= k && *minh.peek().unwrap() > s {
                minh.pop();
            }
            if minh.len() < k {
                minh.push(s);
            }
            if maxh.len() >= k && maxh.peek().unwrap().0 < s {
                maxh.pop();
            }
            if maxh.len() < k {
                maxh.push(std::cmp::Reverse(s));
            }
        }
        let mut diff = 0;
        while let Some(std::cmp::Reverse(i)) = maxh.pop() {
            diff += i as i64;
        }
        while let Some(i) = minh.pop() {
            diff -= i as i64;
        }
        diff
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn put_marbles() {
        assert_eq!(Solution::put_marbles(vec![1, 3, 5, 1], 2), 4);
        assert_eq!(Solution::put_marbles(vec![1, 3], 2), 0);
    }
}
