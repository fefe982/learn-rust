// https://leetcode.com/problems/find-the-minimum-cost-array-permutation/
// 3149. Find the Minimum Cost Array Permutation
pub struct Solution;
impl Solution {
    pub fn find_permutation(nums: Vec<i32>) -> Vec<i32> {
        let len = nums.len();
        if len == 2 {
            return vec![0, 1];
        }
        let nstate = 1 << (len - 1);
        let mut save = vec![vec![(0, 0); len - 1]; nstate];
        let last: usize = nstate - 1;
        for i in 1..=last {
            let mut i_masked = i;
            while i_masked > 0 {
                let nbit = i_masked.trailing_zeros() as usize;
                let bit_mask = 1 << nbit;
                let last_mask = i ^ bit_mask;
                if last_mask == 0 {
                    save[i][nbit] = ((nums[0] - nbit as i32 - 1).abs(), 0);
                    break;
                }
                let mut min = i32::MAX;
                let mut min_bit = 0;
                let mut last_masked = last_mask;
                while last_masked > 0 {
                    let last_bit = last_masked.trailing_zeros() as usize;
                    let this_cost = save[last_mask][last_bit].0 + (nums[last_bit + 1] - nbit as i32 - 1).abs();
                    if this_cost < min {
                        min = this_cost;
                        min_bit = last_bit;
                    }
                    last_masked = last_masked ^ (1 << last_bit);
                }
                save[i][nbit] = (min, min_bit);
                i_masked = i_masked ^ bit_mask;
            }
        }
        let mut min = i32::MAX;
        let mut res = vec![0; len];
        let mut last_state = 0;
        let mut last_bit = 0;
        for i in 0..len - 1 {
            let this_cost = save[last][i].0 + nums[i + 1];
            if this_cost < min {
                res[1] = i as i32 + 1;
                last_state = last ^ (1 << i);
                last_bit = save[last][i].1;
                min = this_cost;
            }
        }
        for i in 2..len {
            res[i] = last_bit as i32 + 1;
            let n_last_bit = save[last_state][last_bit].1;
            last_state = last_state ^ (1 << last_bit);
            last_bit = n_last_bit;
        }
        res
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_find_permutation() {
        assert_eq!(Solution::find_permutation(vec![0, 3, 2, 1]), [0, 2, 3, 1]);
        assert_eq!(Solution::find_permutation(vec![0, 2, 1, 3]), [0, 2, 3, 1]);
        assert_eq!(Solution::find_permutation(vec![1, 0, 2]), [0, 1, 2]);
        assert_eq!(Solution::find_permutation(vec![0, 2, 1]), [0, 2, 1]);
    }
}
