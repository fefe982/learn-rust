// https://leetcode.com/problems/minimum-time-to-complete-all-tasks/
// 2589. Minimum Time to Complete All Tasks
pub struct Solution;
impl Solution {
    pub fn find_minimum_time(tasks: Vec<Vec<i32>>) -> i32 {
        let mut stk = vec![(-1, -1, 0)];
        let mut tasks = tasks;
        tasks.sort_unstable_by_key(|x| x[1]);
        for task in tasks {
            let s = task[0];
            let e = task[1];
            let i = stk.partition_point(|x| x.0 < s);
            let mut dur = task[2];
            dur -= stk.last().unwrap().2 - stk[i - 1].2;
            if stk[i - 1].1 >= s {
                dur -= stk[i - 1].1 - s + 1;
            }
            if dur <= 0 {
                continue;
            }
            let mut last = 0;
            while let Some(&(ls, le, ld)) = stk.last() {
                if e - dur + 1 <= le {
                    stk.pop();
                    dur += le - ls + 1;
                } else {
                    last = ld;
                    break;
                }
            }
            stk.push((e - dur + 1, e, last + dur));
        }
        stk.last().unwrap().2
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn test_find_minimum_time() {
        assert_eq!(
            Solution::find_minimum_time(vec_vec![[1, 10, 7], [4, 11, 1], [3, 19, 7], [10, 15, 2]]),
            8
        );
        assert_eq!(
            Solution::find_minimum_time(vec_vec![[2, 3, 1], [4, 5, 1], [1, 5, 2]]),
            2
        );
        assert_eq!(
            Solution::find_minimum_time(vec_vec![[1, 3, 2], [2, 5, 3], [5, 6, 2]]),
            4
        );
    }
}
