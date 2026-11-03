// https://leetcode.com/problems/count-bowl-subarrays/
// 3676. Count Bowl Subarrays
pub struct Solution;
impl Solution {
    pub fn bowl_subarrays(nums: Vec<i32>) -> i64 {
        let mut st = vec![];
        let mut left = Vec::with_capacity(nums.len());
        for i in 0..nums.len() {
            while let Some(&j) = st.last() {
                if nums[j] < nums[i] {
                    st.pop();
                } else {
                    break;
                }
            }
            left.push(if let Some(&j) = st.last() { j as i32 } else { -1 });
            st.push(i);
        }
        st.clear();
        let mut right = vec![0; nums.len()];
        for i in (0..nums.len()).rev() {
            while let Some(&j) = st.last() {
                if nums[j] < nums[i] {
                    st.pop();
                } else {
                    break;
                }
            }
            right[i] = if let Some(&j) = st.last() {
                j as i32
            } else {
                nums.len() as i32
            };
            st.push(i);
        }
        let mut ans = 0;
        for i in 0..nums.len() {
            if left[i] != -1 && right[i] != nums.len() as i32 {
                ans += 1;
            }
        }
        ans
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn bowl_subarrays() {
        assert_eq!(Solution::bowl_subarrays(vec![2, 5, 3, 1, 4]), 2);
        assert_eq!(Solution::bowl_subarrays(vec![5, 1, 2, 3, 4]), 3);
        assert_eq!(Solution::bowl_subarrays(vec![1000000000, 999999999, 999999998]), 0);
    }
}
