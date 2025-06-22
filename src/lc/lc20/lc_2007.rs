// https://leetcode.com/problems/find-original-array-from-doubled-array/
// 2007. Find Original Array From Doubled Array
pub struct Solution;
impl Solution {
    pub fn find_original_array(changed: Vec<i32>) -> Vec<i32> {
        if changed.len() % 2 == 1 {
            return vec![];
        }
        let mut changed = changed;
        changed.sort_unstable();
        let mut ans = Vec::with_capacity(changed.len() / 2);
        let mut i = 0;
        let mut j = changed.partition_point(|&x| x < changed[0] * 2);
        if j == 0 {
            j = 1;
        }
        if j == changed.len() || changed[j] != changed[0] * 2 {
            return vec![];
        }
        ans.push(changed[0]);
        changed[j] = i32::MAX;
        i += 1;
        while i < changed.len() && changed[i] == i32::MAX {
            i += 1;
        }
        j = (j + 1).max(i + 1);
        while j < changed.len() {
            while j < changed.len() && changed[j] < changed[i] * 2 {
                j += 1;
            }
            if j == changed.len() {
                break;
            }
            if changed[j] != changed[i] * 2 {
                return vec![];
            }
            ans.push(changed[i]);
            changed[j] = i32::MAX;
            i += 1;
            while i < changed.len() && changed[i] == i32::MAX {
                i += 1;
            }
            j = (j + 1).max(i + 1);
        }
        if ans.len() * 2 == changed.len() {
            ans
        } else {
            vec![]
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_find_original_array() {
        assert_eq!(Solution::find_original_array(vec![1, 3, 4, 2, 6, 8]), [1, 3, 4]);
        assert_eq!(Solution::find_original_array(vec![6, 3, 0, 1]), []);
        assert_eq!(Solution::find_original_array(vec![1]), []);
    }
}
