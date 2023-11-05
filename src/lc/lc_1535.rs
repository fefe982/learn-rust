// https://leetcode.com/problems/find-the-winner-of-an-array-game/
// 1535. Find the Winner of an Array Game
pub struct Solution;
impl Solution {
    pub fn get_winner(arr: Vec<i32>, k: i32) -> i32 {
        let mut max = arr[0];
        let mut cnt = 0;
        for &n in arr.iter().skip(1) {
            if n > max {
                max = n;
                cnt = 1;
            } else {
                cnt += 1;
            }
            if cnt == k {
                break;
            }
        }
        max
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_get_winner() {
        assert_eq!(Solution::get_winner(vec![1, 25, 35, 42, 68, 70], 1), 25);
        assert_eq!(Solution::get_winner(vec![2, 1, 3, 5, 4, 6, 7], 2), 5);
        assert_eq!(Solution::get_winner(vec![3, 2, 1], 10), 3);
    }
}
