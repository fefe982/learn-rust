// https://leetcode.com/problems/the-number-of-good-subsets/
// 1994. The Number of Good Subsets
pub struct Solution;
impl Solution {
    fn fact(n: i32) -> usize {
        let mut res = 0;
        for (i, &f) in [2, 3, 5, 7, 11, 13, 17, 19, 23, 29].iter().enumerate() {
            if n % f == 0 {
                res |= 1 << i;
            }
        }
        res
    }
    pub fn number_of_good_subsets(nums: Vec<i32>) -> i32 {
        let mut cnt = vec![0; 31];
        let mut mask = vec![0; 31];
        let mut full_mask = 0;
        let md = 1_0000_0000_7i64;
        for num in nums {
            if num % 4 == 0 || num % 9 == 0 || num % 25 == 0 {
                continue;
            }
            cnt[num as usize] += 1;
            if num > 1 && mask[num as usize] == 0 {
                let m = Self::fact(num);
                full_mask |= m;
                mask[num as usize] = m;
            }
        }
        let mut all_cnt = vec![0; 1 << 10];
        all_cnt[0] = 1;
        for i in 2..=30 {
            if cnt[i] == 0 {
                continue;
            }
            let mut m = full_mask;
            while m > 0 {
                m = (m - 1) & full_mask;
                if m & mask[i] == 0 {
                    all_cnt[m | mask[i]] = (all_cnt[m | mask[i]] + all_cnt[m] * cnt[i]) % md;
                }
            }
        }
        let mut sum = 0;
        let mut m = full_mask;
        while m > 0 {
            sum = (sum + all_cnt[m]) % md;
            m = (m - 1) & full_mask;
        }
        let mut pow = 2;
        while cnt[1] > 0 {
            if cnt[1] & 1 == 1 {
                sum = sum * pow % md;
            }
            pow = pow * pow % md;
            cnt[1] >>= 1;
        }
        sum as i32
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_number_of_good_subsets() {
        assert_eq!(
            Solution::number_of_good_subsets(vec![18, 28, 2, 17, 29, 30, 15, 9, 12]),
            19
        );
        assert_eq!(Solution::number_of_good_subsets(vec![1, 2, 3, 4]), 6);
        assert_eq!(Solution::number_of_good_subsets(vec![4, 2, 3, 15]), 5);
    }
}
