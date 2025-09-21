// https://leetcode.com/problems/sum-of-mutated-array-closest-to-target/
// 1300. Sum of Mutated Array Closest to Target
pub struct Solution;
impl Solution {
    pub fn find_best_value(arr: Vec<i32>, target: i32) -> i32 {
        let mut arr = arr;
        arr.sort_unstable();
        let mut target = target;
        for i in 0..arr.len() {
            let r = (arr.len() - i) as i32;
            let mut next = target / r;
            if (next + 1) * r - target < target - next * r {
                next += 1;
            }
            if next < arr[i] {
                return next;
            }
            target -= arr[i]
        }
        arr[arr.len() - 1]
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn find_best_value() {
        assert_eq!(Solution::find_best_value(vec![3, 4, 5, 6], 18), 6);
        assert_eq!(Solution::find_best_value(vec![4, 9, 3], 10), 3);
        assert_eq!(Solution::find_best_value(vec![2, 3, 5], 10), 5);
        assert_eq!(
            Solution::find_best_value(vec![60864, 25176, 27249, 21296, 20204], 56803),
            11361
        );
    }
}
