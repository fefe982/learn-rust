// https://leetcode.com/problems/element-appearing-more-than-25-in-sorted-array/
// 1287. Element Appearing More Than 25% In Sorted Array
pub struct Solution;
impl Solution {
    pub fn find_special_integer(arr: Vec<i32>) -> i32 {
        let n = arr.len() as i32;
        let mut last = -1;
        let mut cnt = 0;
        for a in arr {
            if a == last {
                cnt += 1;
                if cnt * 4 > n {
                    return last;
                }
            } else {
                last = a;
                cnt = 1;
                if cnt * 4 > n {
                    return last;
                }
            }
        }
        -1
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_special_integer() {
        assert_eq!(Solution::find_special_integer(vec![1, 2, 2, 6, 6, 6, 6, 7, 10]), 6);
        assert_eq!(Solution::find_special_integer(vec![1, 1]), 1);
        assert_eq!(Solution::find_special_integer(vec![1]), 1);
    }
}
