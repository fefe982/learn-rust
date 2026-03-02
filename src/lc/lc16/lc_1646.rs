// https://leetcode.com/problems/get-maximum-in-generated-array/
// 1646. Get Maximum in Generated Array
pub struct Solution;
impl Solution {
    pub fn get_maximum_generated(n: i32) -> i32 {
        if n == 0 {
            return 0;
        }
        if n == 1 {
            return 1;
        }
        let n = n as usize;
        let mut arr = vec![0; n + 1];
        arr[1] = 1;
        let mut max = 1;
        for i in 2..=n {
            if i % 2 == 0 {
                arr[i] = arr[i / 2];
            } else {
                arr[i] = arr[i / 2] + arr[i / 2 + 1];
                max = max.max(arr[i]);
            }
        }
        max
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn get_maximum_generated() {
        assert_eq!(Solution::get_maximum_generated(7), 3);
        assert_eq!(Solution::get_maximum_generated(2), 1);
    }
}
