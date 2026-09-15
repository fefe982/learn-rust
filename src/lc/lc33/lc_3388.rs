// https://leetcode.com/problems/count-beautiful-splits-in-an-array/
// 3388. Count Beautiful Splits in an Array
pub struct Solution;
impl Solution {
    pub fn beautiful_splits(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        if n < 3 {
            return 0;
        }
        let mut lcps = vec![vec![0; n]; n];
        for i in 0..n {
            let mut max_i = i;
            let mut max_r = i;
            for j in i + 1..n {
                if max_r > j {
                    lcps[i][j] = (max_r - j + 1).min(lcps[i][j - max_i + i]);
                }
                while j + lcps[i][j] < n && nums[j + lcps[i][j]] == nums[lcps[i][j] + i] {
                    lcps[i][j] += 1;
                }
                if j + lcps[i][j] - 1 > max_r {
                    max_i = j;
                    max_r = j + lcps[i][j] - 1;
                }
            }
        }
        let mut ans = 0;
        for i in 0..n - 2 {
            for j in i + 1..n - 1 {
                if lcps[0][i + 1] >= i + 1 && j - i >= i + 1 {
                    ans += 1;
                    continue;
                }
                if lcps[i + 1][j + 1] >= j - i {
                    ans += 1;
                    continue;
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
    fn beautiful_splits() {
        assert_eq!(
            Solution::beautiful_splits(vec![2, 2, 0, 0, 0, 0, 0, 1, 2, 2, 0, 0, 0, 1, 0]),
            22
        );
        assert_eq!(Solution::beautiful_splits(vec![1, 1, 2, 1]), 2);
        assert_eq!(Solution::beautiful_splits(vec![1, 2, 3, 4]), 0);
    }
}
