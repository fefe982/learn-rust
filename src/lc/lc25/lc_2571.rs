// https://leetcode.com/problems/minimum-operations-to-reduce-an-integer-to-0/
// 2571. Minimum Operations to Reduce an Integer to 0
pub struct Solution;
impl Solution {
    pub fn min_operations(n: i32) -> i32 {
        let mut n = n;
        while n % 2 == 0 {
            n >>= 1;
        }
        let mut m = std::collections::HashSet::new();
        let mut q = std::collections::VecDeque::new();
        q.push_back((n, 0));
        m.insert(n);
        while let Some((n, c)) = q.pop_front() {
            let mut nn = n - 1;
            if nn == 0 {
                return c + 1;
            }
            while nn % 2 == 0 {
                nn >>= 1;
            }
            if !m.contains(&nn) {
                q.push_back((nn, c + 1));
                m.insert(nn);
            }
            let mut nn = n + 1;
            while nn % 2 == 0 {
                nn >>= 1;
            }
            if !m.contains(&nn) {
                q.push_back((nn, c + 1));
                m.insert(nn);
            }
        }
        -1
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn min_operations() {
        assert_eq!(Solution::min_operations(39), 3);
        assert_eq!(Solution::min_operations(54), 3);
    }
}
