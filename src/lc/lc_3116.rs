// https://leetcode.com/problems/kth-smallest-amount-with-single-denomination-combination/
// 3116. Kth Smallest Amount With Single Denomination Combination
pub struct Solution;
impl Solution {
    pub fn find_kth_smallest(coins: Vec<i32>, k: i32) -> i64 {
        let len = coins.len();
        let mut mul = vec![1; 1 << len];
        let gcd = |mut a: i64, mut b: i64| -> i64 {
            loop {
                if b == 0 {
                    return a;
                }
                a = a % b;
                if a == 0 {
                    return b;
                }
                b = b % a;
            }
        };
        for i in 1..(1 << len) as usize {
            let j = i & (i - 1);
            let ic = i.trailing_zeros() as usize;
            let c = coins[ic] as i64;
            mul[i] = mul[j] / gcd(mul[j], c) * c;
        }
        let cnt = |x: i64| -> i64 {
            let mut cnt = 0;
            for i in 1..(1 << len) as usize {
                let ni = i.count_ones();
                if ni % 2 == 1 {
                    cnt += x / mul[i];
                } else {
                    cnt -= x / mul[i];
                }
            }
            cnt
        };
        let mut l = 1;
        let k = k as i64;
        let mut r = 25 * k;
        if cnt(l) >= k {
            return l;
        }
        while l + 1 < r {
            let mid = (l + r) / 2;
            if cnt(mid) < k {
                l = mid;
            } else {
                r = mid;
            }
        }
        r
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_find_kth_smallest() {
        assert_eq!(Solution::find_kth_smallest(vec![3, 6, 9], 3), 9);
        assert_eq!(Solution::find_kth_smallest(vec![5, 2], 7), 12);
    }
}
