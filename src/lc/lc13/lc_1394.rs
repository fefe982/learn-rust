// https://leetcode.com/problems/find-lucky-integer-in-an-array/
// 1394. Find Lucky Integer in an Array
pub struct Solution;
impl Solution {
    pub fn find_lucky(arr: Vec<i32>) -> i32 {
        let mut map = std::collections::HashMap::new();
        for i in arr {
            *map.entry(i).or_insert(0) += 1;
        }
        let mut ans = -1;
        for (k, v) in map {
            if k == v && k > ans {
                ans = k;
            }
        }
        ans
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn find_lucky() {
        assert_eq!(Solution::find_lucky(vec![2, 2, 3, 4]), 2);
        assert_eq!(Solution::find_lucky(vec![1, 2, 2, 3, 3, 3]), 3);
    }
}
