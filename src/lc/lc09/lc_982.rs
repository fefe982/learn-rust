// https://leetcode.com/problems/triples-with-bitwise-and-equal-to-zero/
// 982. Triples with Bitwise AND Equal To Zero
pub struct Solution;
impl Solution {
    pub fn count_triplets(nums: Vec<i32>) -> i32 {
        let mut cnt = 0;
        let len = nums.len();
        for i in 0..len {
            if nums[i] == 0 {
                cnt += 1 + 3 * (len - i - 1) + 3 * (len - i - 1) * (len - i - 1);
                continue;
            }
            for j in i + 1..len {
                let n = nums[i] & nums[j];
                if n == 0 {
                    cnt += 6 + 6 * (len - j - 1);
                    continue;
                }
                for k in j + 1..len {
                    if n & nums[k] == 0 {
                        cnt += 6;
                    }
                }
            }
        }
        cnt as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn count_triplets() {
        assert_eq!(Solution::count_triplets(vec![2, 1, 3]), 12);
        assert_eq!(Solution::count_triplets(vec![0, 0, 0]), 27);
    }
}
