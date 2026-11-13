// https://leetcode.com/problems/design-exam-scores-tracker/
// 3709. Design Exam Scores Tracker
pub struct ExamTracker {
    stk: Vec<(i32, i64)>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl ExamTracker {
    pub fn new() -> Self {
        Self { stk: vec![(0, 0)] }
    }

    pub fn record(&mut self, time: i32, score: i32) {
        let s = self.stk.last().unwrap().1 + score as i64;
        self.stk.push((time, s));
    }

    pub fn total_score(&self, start_time: i32, end_time: i32) -> i64 {
        let is = self.stk.partition_point(|&(t, _)| t < start_time) - 1;
        let ie = self.stk.partition_point(|&(t, _)| t <= end_time) - 1;
        self.stk[ie].1 - self.stk[is].1
    }
}

/**
 * Your ExamTracker object will be instantiated and called as such:
 * let obj = ExamTracker::new();
 * obj.record(time, score);
 * let ret_2: i64 = obj.total_score(startTime, endTime);
 */
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_3709() {
        let mut exam_tracker = ExamTracker::new();
        exam_tracker.record(1, 98);
        assert_eq!(exam_tracker.total_score(1, 1), 98);
        exam_tracker.record(5, 99);
        assert_eq!(exam_tracker.total_score(1, 3), 98);
        assert_eq!(exam_tracker.total_score(1, 5), 197);
        assert_eq!(exam_tracker.total_score(3, 4), 0);
        assert_eq!(exam_tracker.total_score(2, 5), 99);
    }
}
