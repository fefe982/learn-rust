// https://leetcode.com/problems/digit-operations-to-make-two-integers-equal/
// 3377. Digit Operations to Make Two Integers Equal
pub struct Solution;
impl Solution {
    pub fn min_operations(n: i32, m: i32) -> i32 {
        let mut cost = vec![-1; 10001];
        cost[1] = 0;
        for i in 2..100 {
            if cost[i] != -1 {
                continue;
            }
            for j in (i * i..10001).step_by(i) {
                cost[j] = i32::MAX;
            }
        }
        let n = n as usize;
        let m = m as usize;
        if cost[n] == -1 || cost[m] == -1 {
            return -1;
        }
        if n == m {
            return n as i32;
        }
        cost[n] = n as i32;
        let mut q = std::collections::BinaryHeap::new();
        q.push(std::cmp::Reverse((n as i32, n)));
        while let Some(std::cmp::Reverse((c, x))) = q.pop() {
            if x == m {
                return c;
            }
            let mut d = 1;
            while d <= x {
                let xd = x / d % 10;
                if xd > 0 {
                    let t = x - d;
                    if (c + t as i32) < cost[t] {
                        cost[t] = c + t as i32;
                        q.push(std::cmp::Reverse((cost[t], t)));
                    }
                }
                if xd < 9 {
                    let t = x + d;
                    if (c + t as i32) < cost[t] {
                        cost[t] = c + t as i32;
                        q.push(std::cmp::Reverse((cost[t], t)));
                    }
                }
                d *= 10;
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
        assert_eq!(Solution::min_operations(7, 7), -1);
        assert_eq!(Solution::min_operations(10, 12), 85);
        assert_eq!(Solution::min_operations(4, 8), -1);
        assert_eq!(Solution::min_operations(6, 2), -1);
    }
}
