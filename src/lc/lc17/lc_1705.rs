// https://leetcode.com/problems/maximum-number-of-eaten-apples/
// 1705. Maximum Number of Eaten Apples
pub struct Solution;
impl Solution {
    pub fn eaten_apples(apples: Vec<i32>, days: Vec<i32>) -> i32 {
        let mut h = std::collections::BinaryHeap::new();
        let mut n = 0;
        let mut len = apples.len();
        for (i, (apple, day)) in apples.into_iter().zip(days.into_iter()).enumerate() {
            if apple > 0 {
                h.push((std::cmp::Reverse(i + day as usize), apple));
            }
            while let Some(mut p) = h.peek_mut() {
                if p.0 .0 <= i {
                    std::collections::binary_heap::PeekMut::pop(p);
                    continue;
                }
                p.1 -= 1;
                n += 1;
                if p.1 == 0 {
                    std::collections::binary_heap::PeekMut::pop(p);
                }
                break;
            }
        }
        while let Some((std::cmp::Reverse(d), a)) = h.pop() {
            if d <= len {
                continue;
            }
            n += a.min((d - len) as i32);
            len += (d - len).min(a as usize);
        }
        n
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn max_eaten_apples() {
        assert_eq!(Solution::eaten_apples(vec![2, 1, 10], vec![2, 10, 1]), 4);
        assert_eq!(Solution::eaten_apples(vec![1, 2, 3, 5, 2], vec![3, 2, 1, 4, 2]), 7);
        assert_eq!(
            Solution::eaten_apples(vec![3, 0, 0, 0, 0, 2], vec![3, 0, 0, 0, 0, 2]),
            5
        );
    }
}
