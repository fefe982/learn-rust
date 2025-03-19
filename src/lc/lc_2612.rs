// https://leetcode.com/problems/minimum-reverse-operations/
// 2612. Minimum Reverse Operations
pub struct Solution;
impl Solution {
    fn get(v: &mut Vec<usize>, i: usize) -> usize {
        if v[i] == i {
            i
        } else {
            v[i] = Self::get(v, v[i]);
            v[i]
        }
    }
    fn merge(v: &mut Vec<usize>, i: usize, j: usize) {
        let pi = Self::get(v, i);
        let pj = Self::get(v, j);
        v[pi] = pj;
    }
    pub fn min_reverse_operations(n: i32, p: i32, banned: Vec<i32>, k: i32) -> Vec<i32> {
        let n = n as usize;
        let p = p as usize;
        let k = k as usize;
        let mut ans = vec![-1; n];
        ans[p] = 0;
        let mut remain = vec![0; n + 2];
        for i in 0..n + 2 {
            remain[i] = i;
        }
        for b in banned {
            let b = b as usize;
            Self::merge(&mut remain, b, b + 2);
        }
        Self::merge(&mut remain, p, p + 2);
        let mut q = std::collections::VecDeque::new();
        q.push_back(p);
        while let Some(c) = q.pop_front() {
            let l = if c + 1 >= k { c + 1 - k } else { k - c - 1 };
            let r = (k + c - 1).min(n * 2 - k - 1 - c);
            let mut nx = l;
            while nx <= r {
                let pn = Self::get(&mut remain, nx);
                if pn > r {
                    break;
                }
                q.push_back(pn);
                ans[pn] = ans[c] + 1;
                nx = pn + 2;
                Self::merge(&mut remain, pn, pn + 2);
            }
        }
        ans
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_min_reverse_operations() {
        assert_eq!(
            Solution::min_reverse_operations(100000, 52511, vec![], 28762),
            [0, 1, 2, 3, 4]
        );
        assert_eq!(Solution::min_reverse_operations(5, 0, vec![], 2), [0, 1, 2, 3, 4]);
        assert_eq!(Solution::min_reverse_operations(4, 0, vec![1, 2], 4), [0, -1, -1, 1]);
        assert_eq!(
            Solution::min_reverse_operations(5, 0, vec![2, 4], 3),
            [0, -1, -1, -1, -1]
        );
    }
}
