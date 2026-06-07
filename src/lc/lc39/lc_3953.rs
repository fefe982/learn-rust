// https://leetcode.com/problems/maximum-score-with-co-prime-element/
// 3953. Maximum Score With Co-Prime Element
pub struct Solution;
impl Solution {
    pub fn max_score(nums: Vec<i32>, max_val: i32) -> i32 {
        let n = nums.len() as i32;
        let max_num = nums.iter().copied().max().unwrap_or(1) as usize;
        let max_val_u = max_val as usize;
        let m = max_num.max(max_val_u);

        let mut freq = vec![0i32; m + 1];
        for &x in &nums {
            freq[x as usize] += 1;
        }

        // Linear sieve for Mobius mu up to m.
        let mut mu = vec![0i32; m + 1];
        let mut primes = Vec::<usize>::new();
        let mut is_composite = vec![false; m + 1];
        mu[1] = 1;
        for i in 2..=m {
            if !is_composite[i] {
                primes.push(i);
                mu[i] = -1;
            }
            for &p in &primes {
                let ip = i * p;
                if ip > m {
                    break;
                }
                is_composite[ip] = true;
                if i % p == 0 {
                    mu[ip] = 0;
                    break;
                } else {
                    mu[ip] = -mu[i];
                }
            }
        }

        let mut cnt_multiples = vec![0i32; m + 1];
        for d in 1..=m {
            let mut s = 0i32;
            let mut k = d;
            while k <= m {
                s += freq[k];
                k += d;
            }
            cnt_multiples[d] = s;
        }

        // coprime_count[x] = sum_{d|x} mu[d] * cnt_multiples[d]
        let mut coprime_count = vec![0i32; m + 1];
        for d in 1..=m {
            let md = mu[d];
            if md == 0 {
                continue;
            }
            let add = md * cnt_multiples[d];
            let mut x = d;
            while x <= m {
                coprime_count[x] += add;
                x += d;
            }
        }

        let mut conflicts = vec![0i32; m + 1];
        for x in 1..=m {
            conflicts[x] = n - coprime_count[x];
        }

        let mut ans = i32::MIN;

        // Candidates that can be set by modification: 1..=max_val.
        for s in 1..=max_val_u {
            let conf = conflicts[s];
            let bonus = if s == 1 {
                if freq[1] > 0 {
                    0
                } else {
                    -1
                }
            } else if s <= max_num && freq[s] > 0 {
                1
            } else if conf > 0 {
                0
            } else {
                -1
            };

            let score = s as i32 - conf + bonus;
            ans = ans.max(score);
        }

        // Values greater than max_val can only be chosen if they already exist (unchanged selected index).
        if max_num > max_val_u {
            for s in (max_val_u + 1)..=max_num {
                if freq[s] > 0 {
                    // For s > 1, selecting an existing s contributes +1 term in the transformed formula.
                    let score = s as i32 - conflicts[s] + 1;
                    ans = ans.max(score);
                }
            }
        }

        ans
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_max_score() {
        assert_eq!(Solution::max_score(vec![3, 4, 6], 5), 4);
        assert_eq!(Solution::max_score(vec![1, 2, 3], 4), 3);
        assert_eq!(Solution::max_score(vec![2, 2], 1), 1);
    }
}
