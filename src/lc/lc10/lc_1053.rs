// https://leetcode.com/problems/previous-permutation-with-one-swap/
// 1053. Previous Permutation With One Swap
pub struct Solution;
impl Solution {
    pub fn prev_perm_opt1(mut arr: Vec<i32>) -> Vec<i32> {
        let mut last_rev = usize::MAX;
        let mut max_swap = 0;
        for idx in 0..arr.len() - 1 {
            if arr[idx] > arr[idx + 1] {
                last_rev = idx;
                max_swap = idx + 1;
            } else if last_rev != usize::MAX
                && (arr[idx + 1] < arr[last_rev] && arr[idx + 1] > arr[max_swap])
            {
                max_swap = idx + 1;
            }
        }
        if last_rev != usize::MAX {
            let tmp = arr[last_rev];
            arr[last_rev] = arr[max_swap];
            arr[max_swap] = tmp;
        }
        arr
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn prev_perm_opt1() {
        assert_eq!(Solution::prev_perm_opt1(vec![3, 2, 1]), vec![3, 1, 2]);
        assert_eq!(Solution::prev_perm_opt1(vec![1, 1, 5]), vec![1, 1, 5]);
        assert_eq!(
            Solution::prev_perm_opt1(vec![1, 9, 4, 6, 7]),
            vec![1, 7, 4, 6, 9]
        );
    }
}
