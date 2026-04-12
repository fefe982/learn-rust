// https://leetcode.com/problems/good-subsequence-queries/
// 3901. Good Subsequence Queries
pub struct Solution;

struct GcdSegTree {
    n: usize,
    tree: Vec<i32>,
}

impl GcdSegTree {
    fn new(vals: &[i32]) -> Self {
        let mut n = 1usize;
        while n < vals.len() {
            n <<= 1;
        }
        let mut tree = vec![0; n << 1];
        for (i, &v) in vals.iter().enumerate() {
            tree[n + i] = v;
        }
        for i in (1..n).rev() {
            tree[i] = Solution::gcd(tree[i << 1], tree[i << 1 | 1]);
        }
        Self { n, tree }
    }

    fn update(&mut self, idx: usize, val: i32) {
        let mut p = self.n + idx;
        self.tree[p] = val;
        while p > 1 {
            p >>= 1;
            self.tree[p] = Solution::gcd(self.tree[p << 1], self.tree[p << 1 | 1]);
        }
    }

    fn all_gcd(&self) -> i32 {
        self.tree[1]
    }
}

impl Solution {
    fn gcd(mut a: i32, mut b: i32) -> i32 {
        while b != 0 {
            (a, b) = (b, a % b);
        }
        a.abs()
    }

    fn build_spf(max_v: usize) -> Vec<usize> {
        let mut spf = vec![0usize; max_v + 1];
        if max_v >= 1 {
            spf[1] = 1;
        }
        for i in 2..=max_v {
            if spf[i] == 0 {
                spf[i] = i;
                if i * i <= max_v {
                    let mut j = i * i;
                    while j <= max_v {
                        if spf[j] == 0 {
                            spf[j] = i;
                        }
                        j += i;
                    }
                }
            }
        }
        spf
    }

    fn unique_prime_factors(mut x: i32, spf: &[usize]) -> Vec<usize> {
        let mut factors = Vec::new();
        if x <= 1 {
            return factors;
        }
        while x > 1 {
            let p = spf[x as usize] as i32;
            factors.push(p as usize);
            while x % p == 0 {
                x /= p;
            }
        }
        factors
    }

    fn xor_upto(x: usize) -> usize {
        match x & 3 {
            0 => x,
            1 => 1,
            2 => x + 1,
            _ => 0,
        }
    }

    fn update_prime_status(
        prime: usize,
        delta: i32,
        idx: usize,
        n: usize,
        all_xor: usize,
        prime_cnt: &mut [i32],
        prime_xor: &mut [usize],
        cover_cnt: &mut [i32],
        zero_cover: &mut i32,
    ) {
        let old_cnt = prime_cnt[prime];
        let old_xor = prime_xor[prime];
        if old_cnt == n as i32 - 1 {
            let miss = all_xor ^ old_xor;
            if cover_cnt[miss] == 1 {
                *zero_cover += 1;
            }
            cover_cnt[miss] -= 1;
        }

        prime_cnt[prime] += delta;
        prime_xor[prime] ^= idx;

        let new_cnt = prime_cnt[prime];
        let new_xor = prime_xor[prime];
        if new_cnt == n as i32 - 1 {
            let miss = all_xor ^ new_xor;
            if cover_cnt[miss] == 0 {
                *zero_cover -= 1;
            }
            cover_cnt[miss] += 1;
        }
    }

    pub fn count_good_subseq(nums: Vec<i32>, p: i32, queries: Vec<Vec<i32>>) -> i32 {
        const MAX_V: usize = 50_000;
        let n = nums.len();
        let spf = Self::build_spf(MAX_V);
        let mut nums = nums;
        let mut scaled = vec![0i32; n];
        let mut divisible_cnt = 0i32;

        for i in 0..n {
            if nums[i] % p == 0 {
                scaled[i] = nums[i] / p;
                divisible_cnt += 1;
            }
        }

        let mut seg = GcdSegTree::new(&scaled);

        let mut prime_cnt = vec![0i32; MAX_V + 1];
        let mut prime_xor = vec![0usize; MAX_V + 1];
        for (idx, &v) in scaled.iter().enumerate() {
            if v > 1 {
                for pr in Self::unique_prime_factors(v, &spf) {
                    prime_cnt[pr] += 1;
                    prime_xor[pr] ^= idx;
                }
            }
        }

        let all_xor = Self::xor_upto(n - 1);
        let mut cover_cnt = vec![0i32; n];
        let mut zero_cover = n as i32;
        for pr in 2..=MAX_V {
            if prime_cnt[pr] == n as i32 - 1 {
                let miss = all_xor ^ prime_xor[pr];
                if cover_cnt[miss] == 0 {
                    zero_cover -= 1;
                }
                cover_cnt[miss] += 1;
            }
        }

        let mut ans = 0;
        for q in queries {
            let idx = q[0] as usize;
            let val = q[1];

            let old_scaled = scaled[idx];
            let new_scaled = if val % p == 0 { val / p } else { 0 };

            if old_scaled != new_scaled {
                let old_factors = Self::unique_prime_factors(old_scaled, &spf);
                let new_factors = Self::unique_prime_factors(new_scaled, &spf);

                for &pr in &old_factors {
                    if !new_factors.contains(&pr) {
                        Self::update_prime_status(
                            pr,
                            -1,
                            idx,
                            n,
                            all_xor,
                            &mut prime_cnt,
                            &mut prime_xor,
                            &mut cover_cnt,
                            &mut zero_cover,
                        );
                    }
                }
                for &pr in &new_factors {
                    if !old_factors.contains(&pr) {
                        Self::update_prime_status(
                            pr,
                            1,
                            idx,
                            n,
                            all_xor,
                            &mut prime_cnt,
                            &mut prime_xor,
                            &mut cover_cnt,
                            &mut zero_cover,
                        );
                    }
                }

                if old_scaled == 0 {
                    divisible_cnt += 1;
                }
                if new_scaled == 0 {
                    divisible_cnt -= 1;
                }

                scaled[idx] = new_scaled;
                seg.update(idx, new_scaled);
            }
            nums[idx] = val;

            let good = if divisible_cnt == 0 {
                false
            } else if divisible_cnt < n as i32 {
                seg.all_gcd() == 1
            } else {
                seg.all_gcd() == 1 && zero_cover > 0
            };

            if good {
                ans += 1;
            }
        }

        ans
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn test_count_good_subseq() {
        assert_eq!(
            Solution::count_good_subseq(vec![4, 8, 12, 16], 2, vec_vec![[0, 3], [2, 6]]),
            1
        );
        assert_eq!(
            Solution::count_good_subseq(vec![4, 5, 7, 8], 3, vec_vec![[0, 6], [1, 9], [2, 3]]),
            2
        );
        assert_eq!(
            Solution::count_good_subseq(vec![5, 7, 9], 2, vec_vec![[1, 4], [2, 8]]),
            0
        );
    }
}
