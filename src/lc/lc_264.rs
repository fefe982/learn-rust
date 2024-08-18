// https://leetcode.com/problems/ugly-number-ii/
// 264. Ugly Number II
pub struct Solution;
impl Solution {
    pub fn nth_ugly_number(n: i32) -> i32 {
        let mut q = std::collections::BinaryHeap::new();
        q.push(std::cmp::Reverse(1i64));
        let mut max = 1;
        for i in 1..n {
            let t = q.pop().unwrap().0;
            if t == 1 || (t % 3 != 0 && t % 5 != 0) {
                if q.len() as i32 + i < n || t * 2 < max {
                    q.push(std::cmp::Reverse(t * 2));
                    max = max.max(t * 2)
                }
                if q.len() as i32 + i < n || t * 3 < max {
                    q.push(std::cmp::Reverse(t * 3));
                    max = max.max(t * 3)
                }
                if q.len() as i32 + i < n || t * 5 < max {
                    q.push(std::cmp::Reverse(t * 5));
                    max = max.max(t * 5)
                }
            } else if t % 5 != 0 {
                if q.len() as i32 + i < n || t * 3 < max {
                    q.push(std::cmp::Reverse(t * 3));
                    max = max.max(t * 3)
                }
                if q.len() as i32 + i < n || t * 5 < max {
                    q.push(std::cmp::Reverse(t * 5));
                    max = max.max(t * 5)
                }
            } else {
                if q.len() as i32 + i < n || t * 5 < max {
                    q.push(std::cmp::Reverse(t * 5));
                    max = max.max(t * 5)
                }
            }
        }
        q.pop().unwrap().0 as i32
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_nth_ugly_number() {
        assert_eq!(Solution::nth_ugly_number(1407), 536870912);
        assert_eq!(Solution::nth_ugly_number(10), 12);
        assert_eq!(Solution::nth_ugly_number(1), 1);
    }
}
