// https://leetcode.cn/problems/minimum-jumps-to-reach-end-via-prime-teleportation/
// 3629. Minimum Jumps to Reach End via Prime Teleportation
pub struct Solution;
use std::collections::{HashMap, HashSet, VecDeque};
impl Solution {
    pub fn min_jumps(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        if n == 1 {
            return 0;
        }

        let max_val = nums.iter().copied().max().unwrap_or(1) as usize;
        let spf = Self::smallest_prime_factors(max_val);

        let mut prime_to_indices: HashMap<usize, Vec<usize>> = HashMap::new();
        for (i, &v) in nums.iter().enumerate() {
            let factors = Self::unique_prime_factors(v as usize, &spf);
            for p in factors {
                prime_to_indices.entry(p).or_default().push(i);
            }
        }

        let mut dist = vec![-1; n];
        let mut queue = VecDeque::new();
        let mut used_prime = HashSet::new();

        dist[0] = 0;
        queue.push_back(0usize);

        while let Some(i) = queue.pop_front() {
            let step = dist[i] + 1;

            if i + 1 < n && dist[i + 1] == -1 {
                dist[i + 1] = step;
                if i + 1 == n - 1 {
                    return step;
                }
                queue.push_back(i + 1);
            }
            if i > 0 && dist[i - 1] == -1 {
                dist[i - 1] = step;
                queue.push_back(i - 1);
            }

            let val = nums[i] as usize;
            if val >= 2 && spf[val] == val && used_prime.insert(val) {
                if let Some(indices) = prime_to_indices.get(&val) {
                    for &j in indices {
                        if dist[j] == -1 {
                            dist[j] = step;
                            if j == n - 1 {
                                return step;
                            }
                            queue.push_back(j);
                        }
                    }
                }
            }
        }

        dist[n - 1]
    }

    fn smallest_prime_factors(limit: usize) -> Vec<usize> {
        let mut spf: Vec<usize> = (0..=limit).collect();
        if limit >= 1 {
            spf[1] = 1;
        }

        let mut i = 2usize;
        while i * i <= limit {
            if spf[i] == i {
                let mut j = i * i;
                while j <= limit {
                    if spf[j] == j {
                        spf[j] = i;
                    }
                    j += i;
                }
            }
            i += 1;
        }
        spf
    }

    fn unique_prime_factors(mut x: usize, spf: &[usize]) -> Vec<usize> {
        let mut factors = Vec::new();
        while x > 1 {
            let p = spf[x];
            factors.push(p);
            while x % p == 0 {
                x /= p;
            }
        }
        factors
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn min_jumps() {
        assert_eq!(Solution::min_jumps(vec![1, 2, 4, 6]), 2);
        assert_eq!(Solution::min_jumps(vec![2, 3, 4, 7, 9]), 2);
        assert_eq!(Solution::min_jumps(vec![4, 6, 5, 8]), 3);
    }
}
