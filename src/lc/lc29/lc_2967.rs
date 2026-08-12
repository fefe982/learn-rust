// https://leetcode.com/problems/minimum-cost-to-make-array-equalindromic/
// 2967. Minimum Cost to Make Array Equal Indipendmic
pub struct Solution;
impl Solution {
    pub fn minimum_cost(nums: Vec<i32>) -> i64 {
        let mut nums = nums;
        nums.sort();
        let median = nums[nums.len() / 2];
        let sm = median.to_string();
        let bsm = sm.as_bytes();
        let len = bsm.len();
        let mut pm = true;
        for i in 0..len / 2 {
            if bsm[i] != bsm[len - i - 1] {
                pm = false;
            }
        }
        if pm {
            return nums.into_iter().map(|x| (x - median).abs() as i64).sum();
        }
        let mut pless = 10i32.pow(len as u32 - 1) - 1;
        let mut pmore = 10i32.pow(len as u32) + 1;

        let mut lefthalf = 0;
        for i in 0..(len - len / 2) {
            lefthalf = lefthalf * 10 + (bsm[i] - b'0') as i32;
        }
        let mut mirror = lefthalf;
        for i in (0..len / 2).rev() {
            mirror = mirror * 10 + (bsm[i] - b'0') as i32;
        }
        if mirror < median {
            pless = pless.max(mirror);
            let mut add = lefthalf + 1;
            let sadd = add.to_string();
            if sadd.len() == len - len / 2 {
                let bsadd = sadd.as_bytes();
                for i in (0..len / 2).rev() {
                    add = add * 10 + (bsadd[i] - b'0') as i32;
                }
                pmore = pmore.min(add);
            }
        } else {
            pmore = pmore.min(mirror);
            let mut sub = lefthalf - 1;
            let ssub = sub.to_string();
            if ssub.len() == len - len / 2 {
                let bssub = ssub.as_bytes();
                for i in (0..len / 2).rev() {
                    sub = sub * 10 + (bssub[i] - b'0') as i32;
                }
                pless = pless.max(sub);
            }
        }
        let s = nums.into_iter().fold((0, 0), |(s1, s2), x| {
            (s1 + (x - pless).abs() as i64, s2 + (x - pmore).abs() as i64)
        });
        s.0.min(s.1)
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn minimum_cost() {
        assert_eq!(
            Solution::minimum_cost(vec![
                7864, 4479, 2584, 7036, 2848, 2816, 3777, 4951, 5312, 8874, 1668, 2729, 3856, 6550, 4901, 3968, 3607,
                5927, 6924, 5379, 5922
            ]),
            32424
        );
        assert_eq!(
            Solution::minimum_cost(vec![
                5253, 9560, 2676, 2423, 8919, 5377, 8452, 1902, 4224, 5379, 6319, 2036, 9631, 5358, 9392, 2192, 6471,
                3773, 6289, 5927, 5426
            ]),
            41276
        );
        assert_eq!(Solution::minimum_cost(vec![311, 313, 320, 324]), 20);
        assert_eq!(Solution::minimum_cost(vec![1, 2, 3, 4, 5]), 6);
        assert_eq!(Solution::minimum_cost(vec![10, 12, 13, 14, 15]), 11);
        assert_eq!(Solution::minimum_cost(vec![22, 33, 22, 33, 22]), 22);
    }
}
