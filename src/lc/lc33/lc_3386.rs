// https://leetcode.com/problems/button-with-longest-push-time/
// 3386. Button With Longest Push Time
pub struct Solution;
impl Solution {
    pub fn button_with_longest_time(events: Vec<Vec<i32>>) -> i32 {
        let mut idx = 0;
        let mut max = 0;
        let mut last = 0;
        for event in events {
            let (i, t) = (event[0], event[1]);
            if t - last > max {
                max = t - last;
                idx = i;
            } else if t - last == max && i < idx {
                idx = i;
            }
            last = t;
        }
        idx
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn button_with_longest_time() {
        assert_eq!(
            Solution::button_with_longest_time(vec_vec![[1, 2], [2, 5], [3, 9], [1, 15]]),
            1
        );
        assert_eq!(Solution::button_with_longest_time(vec_vec![[10, 5], [1, 7]]), 10);
    }
}
