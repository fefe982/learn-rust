// https://leetcode.com/problems/valid-mountain-array/
// 941. Valid Mountain Array
pub struct Solution;
impl Solution {
    pub fn valid_mountain_array(arr: Vec<i32>) -> bool {
        if arr.len() < 3 {
            return false;
        }
        let mut inc = 1;
        if arr[0] >= arr[1] {
            return false;
        }
        for i in 2..arr.len() {
            match arr[i].cmp(&arr[i - 1]) {
                std::cmp::Ordering::Less => {
                    inc = 0;
                }
                std::cmp::Ordering::Equal => {
                    return false;
                }
                std::cmp::Ordering::Greater => {
                    if inc == 0 {
                        return false;
                    }
                }
            }
        }
        inc == 0
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn valid_mountain_array() {
        assert_eq!(Solution::valid_mountain_array(vec![9, 8, 7]), false);
        assert_eq!(Solution::valid_mountain_array(vec![2, 1]), false);
        assert_eq!(Solution::valid_mountain_array(vec![3, 5, 5]), false);
        assert_eq!(Solution::valid_mountain_array(vec![0, 3, 2, 1]), true);
    }
}
