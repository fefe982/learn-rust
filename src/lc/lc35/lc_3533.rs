// https://leetcode.com/problems/concatenated-divisibility/
// 3533. Concatenated Divisibility
pub struct Solution;
impl Solution {
    pub fn concatenated_divisibility(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let mut nums = nums;
        nums.sort_unstable();
        let mut p10 = 1;
        let mut l = 0;
        let len = nums.len();
        let mut lnums = Vec::with_capacity(len);
        let mut lsum = 0;
        for i in 0..len {
            while p10 <= nums[i] {
                p10 *= 10;
                l += 1;
            }
            lnums.push(l);
            lsum += l;
        }
        let mut p10k = Vec::with_capacity(lsum);
        p10k.push(1 % k);
        for i in 1..lsum {
            p10k.push((p10k[i - 1] * 10) % k);
        }
        let ks = k as usize;
        let mut dp = vec![vec![usize::MAX; ks]; 1 << len];
        let mut masklen = vec![0; 1 << len];
        for i in 1..(1 << len) {
            masklen[i] = masklen[i & (i - 1)] + lnums[i.trailing_zeros() as usize];
        }
        dp[0][0] = 0;
        for i in 1..(1 << len) {
            for kk in 0..k {
                for j in 0..len {
                    if i & (1 << j) == 0 {
                        continue;
                    }
                    let ii = i ^ (1 << j);
                    let iik = ((k + kk - nums[j] * p10k[masklen[ii]] % k) % k) as usize;
                    if dp[ii][iik] != usize::MAX {
                        dp[i][kk as usize] = j;
                        break;
                    }
                }
            }
        }
        let mut ans = vec![];
        if dp[(1 << len) - 1][0] != usize::MAX {
            let mut i = (1 << len) - 1;
            let mut kk = 0;
            while i != 0 {
                let j = dp[i][kk as usize];
                ans.push(nums[j]);
                i ^= 1 << j;
                kk = (k + kk - nums[j] * p10k[masklen[i]] % k) % k;
            }
        }
        ans
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn concatenated_divisibility() {
        assert_eq!(Solution::concatenated_divisibility(vec![2, 7], 4), [7, 2]);
        assert_eq!(Solution::concatenated_divisibility(vec![3, 12, 45], 5), [3, 12, 45]);
        assert_eq!(Solution::concatenated_divisibility(vec![10, 5], 10), [5, 10]);
        assert_eq!(Solution::concatenated_divisibility(vec![1, 2, 3], 5), []);
    }
}
