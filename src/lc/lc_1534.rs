// https://leetcode.com/problems/count-good-triplets/
// 1534. Count Good Triplets
pub struct Solution;
impl Solution {
    pub fn count_good_triplets(arr: Vec<i32>, a: i32, b: i32, c: i32) -> i32 {
        let mut cnt = 0;
        for i in 0..arr.len() - 2 {
            for j in i + 1..arr.len() - 1 {
                for k in j + 1..arr.len() {
                    if (arr[i] - arr[j]).abs() <= a && (arr[j] - arr[k]).abs() <= b && (arr[i] - arr[k]).abs() <= c {
                        cnt += 1;
                    }
                }
            }
        }
        cnt
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_count_good_triplets() {
        assert_eq!(Solution::count_good_triplets(vec![3, 0, 1, 1, 9, 7], 7, 2, 3), 4);
        assert_eq!(Solution::count_good_triplets(vec![1, 1, 2, 2, 3], 0, 0, 1), 0);
    }
}
