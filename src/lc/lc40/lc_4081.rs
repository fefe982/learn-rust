// https://leetcode.com/problems/prime-subset-selection-ii/
// 4081. Prime Subset Selection II
pub struct Solution;
impl Solution {
    pub fn max_prime_subset(n: i32, s: i32) -> Vec<i32> {
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
        let s = s as usize;
        let n = n as usize;
        let m = primes.partition_point(|&x| x <= n);
        let mut f = vec![vec![i32::MAX; s + 1]; m + 1];
        f[m][0] = 0;
        for i in (0..m).rev() {
            let x = primes[i];
            for j in 0..=s {
                if j < x || f[i + 1][j - x] == i32::MAX {
                    f[i][j] = f[i + 1][j];
                } else {
                    f[i][j] = f[i + 1][j].min(f[i + 1][j - x] + 1);
                }
            }
        }
        let mut j = s;
        while j > 0 && f[0][j] == i32::MAX {
            j -= 1;
        }
        let mut ans = vec![];
        for i in 0..m {
            if j == 0 {
                break;
            }
            let x = primes[i];
            if f[i + 1][j - x] != i32::MAX && f[i + 1][j - x] + 1 <= f[i + 1][j] {
                ans.push(x as i32);
                j -= x;
            }
        }
        ans
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn max_prime_subset() {
        assert_eq!(Solution::max_prime_subset(7, 20), vec![2, 3, 5, 7]);
        assert_eq!(Solution::max_prime_subset(15, 18), vec![5, 13]);
        assert_eq!(Solution::max_prime_subset(5, 1), vec![]);
    }
}
