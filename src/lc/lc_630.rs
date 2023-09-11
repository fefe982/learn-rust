// https://leetcode.com/problems/course-schedule-iii/
// 630. Course Schedule III
pub struct Solution;
impl Solution {
    pub fn schedule_course(courses: Vec<Vec<i32>>) -> i32 {
        let mut courses = courses;
        courses.sort_unstable_by_key(|x| x[1]);
        let mut heap = std::collections::BinaryHeap::new();
        let mut total = 0;
        for course in courses {
            let duration = course[0];
            if total < course[0] && total + duration <= course[1] {
                total += duration;
                heap.push(duration);
            } else if let Some(&dtop) = heap.peek() {
                if dtop > duration {
                    heap.pop();
                    heap.push(duration);
                    total -= dtop - duration;
                }
            }
        }
        heap.len() as i32
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn schedule_course() {
        assert_eq!(
            Solution::schedule_course(vec_vec![
                [100, 200],
                [200, 1300],
                [1000, 1250],
                [2000, 3200]
            ]),
            3
        );
        assert_eq!(Solution::schedule_course(vec_vec![[1, 2]]), 1);
        assert_eq!(Solution::schedule_course(vec_vec![[3, 2], [4, 3]]), 0);
    }
}
