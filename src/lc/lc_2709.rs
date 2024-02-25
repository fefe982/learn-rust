// https://leetcode.com/problems/greatest-common-divisor-traversal/
// 2709. Greatest Common Divisor Traversal
pub struct Solution;
impl Solution {
    fn p(v: &mut Vec<usize>, i: usize) -> usize {
        let mut p = i;
        while v[p] != p {
            p = v[p];
        }
        v[i] = p;
        p
    }
    fn union(v: &mut Vec<usize>, a: usize, b: usize) {
        let pa = Self::p(v, a);
        let pb = Self::p(v, b);
        v[pa.max(pb)] = pa.min(pb);
    }
    pub fn can_traverse_all_pairs(nums: Vec<i32>) -> bool {
        if nums.len() == 1 {
            return true;
        }
        let mut p = (0..=100000).collect::<Vec<_>>();
        let primes = vec![
            2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67, 71, 73, 79, 83, 89, 97, 101, 103,
            107, 109, 113, 127, 131, 137, 139, 149, 151, 157, 163, 167, 173, 179, 181, 191, 193, 197, 199, 211, 223,
            227, 229, 233, 239, 241, 251, 257, 263, 269, 271, 277, 281, 283, 293, 307, 311, 313,
        ];
        let mut primes_set = std::collections::HashSet::<i32>::new();
        let mut least = i32::MAX;
        for n in nums {
            if n == 1 {
                return false;
            }
            if primes_set.contains(&(n as i32)) || Self::p(&mut p, n as usize) != n as usize {
                continue;
            }
            let mut m = n;
            let mut last_pr = 1;
            for &pr in &primes {
                if m % pr == 0 {
                    primes_set.insert(pr);
                    if last_pr == 1 {
                        least = least.min(pr);
                        Self::union(&mut p, n as usize, pr as usize);
                    } else {
                        Self::union(&mut p, last_pr as usize, pr as usize);
                    }
                    while m % pr == 0 {
                        m /= pr;
                    }
                    last_pr = pr;
                }
                if m == 1 {
                    break;
                }
            }
            if m != 1 {
                primes_set.insert(m);
                if last_pr == 1 {
                    Self::union(&mut p, n as usize, m as usize);
                    least = least.min(m);
                } else {
                    Self::union(&mut p, last_pr as usize, m as usize);
                }
            }
        }
        for pr in primes_set {
            if Self::p(&mut p, pr as usize) != least as usize {
                return false;
            }
        }
        true
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_can_traverse_all_pairs() {
        assert_eq!(Solution::can_traverse_all_pairs(vec![2, 3, 6]), true);
        assert_eq!(Solution::can_traverse_all_pairs(vec![3, 9, 5]), false);
        assert_eq!(Solution::can_traverse_all_pairs(vec![4, 3, 12, 8]), true);
    }
}
