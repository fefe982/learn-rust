// https://leetcode.com/problems/split-the-array-to-make-coprime-products/
// 2584. Split the Array to Make Coprime Products
pub struct Solution;
impl Solution {
    pub fn find_valid_split(nums: Vec<i32>) -> i32 {
        let primes = [
            2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67, 71, 73, 79, 83, 89, 97, 101, 103,
            107, 109, 113, 127, 131, 137, 139, 149, 151, 157, 163, 167, 173, 179, 181, 191, 193, 197, 199, 211, 223,
            227, 229, 233, 239, 241, 251, 257, 263, 269, 271, 277, 281, 283, 293, 307, 311, 313, 317, 331, 337, 347,
            349, 353, 359, 367, 373, 379, 383, 389, 397, 401, 409, 419, 421, 431, 433, 439, 443, 449, 457, 461, 463,
            467, 479, 487, 491, 499, 503, 509, 521, 523, 541, 547, 557, 563, 569, 571, 577, 587, 593, 599, 601, 607,
            613, 617, 619, 631, 641, 643, 647, 653, 659, 661, 673, 677, 683, 691, 701, 709, 719, 727, 733, 739, 743,
            751, 757, 761, 769, 773, 787, 797, 809, 811, 821, 823, 827, 829, 839, 853, 857, 859, 863, 877, 881, 883,
            887, 907, 911, 919, 929, 937, 941, 947, 953, 967, 971, 977, 983, 991, 997,
        ];
        let get_primes = |mut n: i32| -> Vec<i32> {
            let mut pv = vec![];
            let mut i = 0;
            while n > 1 && i < primes.len() && primes[i] * primes[i] <= n {
                if n % primes[i] == 0 {
                    pv.push(primes[i]);
                    while n % primes[i] == 0 {
                        n /= primes[i];
                    }
                }
                i += 1;
            }
            if n > 1 {
                pv.push(n);
            }
            pv
        };
        let mut r = std::collections::HashMap::<i32, i32>::new();
        let mut nums = nums
            .into_iter()
            .map(|n| {
                let v = get_primes(n);
                for &pv in &v {
                    *r.entry(pv).or_default() += 1;
                }
                v
            })
            .collect::<Vec<_>>();
        let mut l = std::collections::HashMap::<i32, i32>::new();
        let mut common = 0;
        nums.pop();
        for (i, nv) in nums.iter().enumerate() {
            for &n in nv {
                let lc = l.entry(n).or_default();
                *lc += 1;
                let rc = r.entry(n).or_default();
                *rc -= 1;
                if *lc == 1 && *rc > 0 {
                    common += 1;
                } else if *lc > 1 && *rc == 0 {
                    common -= 1;
                }
            }
            if common == 0 {
                return i as i32;
            }
        }
        -1
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_find_valid_split() {
        assert_eq!(Solution::find_valid_split(vec![1]), -1);
        assert_eq!(Solution::find_valid_split(vec![1, 41, 99, 17, 9, 8, 36, 70]), 0);
        assert_eq!(Solution::find_valid_split(vec![4, 7, 8, 15, 3, 5]), 2);
        assert_eq!(Solution::find_valid_split(vec![4, 7, 15, 8, 3, 5]), -1);
    }
}
