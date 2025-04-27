// https://leetcode.com/problems/super-washing-machines/
// 517. Super Washing Machines
pub struct Solution;
impl Solution {
    pub fn find_min_moves(machines: Vec<i32>) -> i32 {
        let sum = machines.iter().sum::<i32>();
        if sum % machines.len() as i32 != 0 {
            return -1;
        }
        let avg = sum / machines.len() as i32;
        let mut left: i32 = 0;
        let mut s = 0;
        for m in machines {
            s = s.max(left.abs());
            if m > avg {
                s = s.max(m - avg);
            }
            left += m - avg;
        }
        s
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn find_min_moves() {
        assert_eq!(Solution::find_min_moves(vec![0, 0, 10, 0, 0, 0, 10, 0, 0, 0]), 8);
        assert_eq!(Solution::find_min_moves(vec![7, 6, 5, 4, 3, 2, 1]), 6);
        assert_eq!(Solution::find_min_moves(vec![9, 1, 8, 8, 9]), 4);
        assert_eq!(Solution::find_min_moves(vec![0, 0, 11, 5]), 8);
        assert_eq!(Solution::find_min_moves(vec![4, 0, 0, 4]), 2);
        assert_eq!(Solution::find_min_moves(vec![1, 0, 5]), 3);
        assert_eq!(Solution::find_min_moves(vec![0, 3, 0]), 2);
        assert_eq!(Solution::find_min_moves(vec![0, 2, 0]), -1);
    }
}
