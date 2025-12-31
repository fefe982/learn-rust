// https://leetcode.com/problems/partition-array-for-maximum-xor-and-and/
// 3630. Partition Array for Maximum XOR AND AND
pub struct Solution;
impl Solution {
    pub fn maximize_xor_and_xor(nums: Vec<i32>) -> i64 {
        let n = nums.len();
        let n2 = 1 << n;
        let mask = n2 - 1;
        let mut sub_and = vec![0; n2];
        let mut sub_xor = vec![0; n2];
        let mut sub_or = vec![0; n2];
        sub_and[0] = i32::MAX;
        for i in 0..n {
            let j = 1 << i;
            for k in 0..j {
                sub_and[j + k] = sub_and[k] & nums[i];
                sub_xor[j + k] = sub_xor[k] ^ nums[i];
                sub_or[j + k] = sub_or[k] | nums[i];
            }
        }
        sub_and[0] = 0;
        let mut ans = 0i64;
        for i in 0..n2 {
            let v = sub_and[i] as i64;
            let xmask = mask ^ i;
            let x = sub_xor[xmask];
            if v + 2 * (sub_or[xmask] as i64) - x as i64 <= ans {
                continue;
            }
            let mut b = [0; 31];
            let mut xvmask = xmask;
            while xvmask > 0 {
                let bit = xvmask.trailing_zeros() as usize;
                let mut n = nums[bit] & !x;
                while n > 0 {
                    let bbit = (i32::BITS - 1 - n.leading_zeros()) as usize;
                    if b[bbit] == 0 {
                        b[bbit] = n;
                        break;
                    } else {
                        n ^= b[bbit];
                    }
                }
                xvmask = (xvmask - 1) & xmask;
            }
            let mut xx = 0;
            for j in (0..31).rev() {
                xx = xx.max(xx ^ b[j]);
            }
            ans = ans.max(v + xx as i64 * 2 + x as i64);
        }
        ans
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn maximize_xor_and_xor() {
        assert_eq!(Solution::maximize_xor_and_xor(vec![2, 3]), 5);
        assert_eq!(Solution::maximize_xor_and_xor(vec![1, 3, 2]), 6);
        assert_eq!(Solution::maximize_xor_and_xor(vec![2, 3, 6, 7]), 15);
    }
}
