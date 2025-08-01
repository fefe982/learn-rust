// https://leetcode.com/problems/reward-top-k-students/
// 2512. Reward Top K Students
pub struct Solution;
impl Solution {
    pub fn top_students(
        positive_feedback: Vec<String>,
        negative_feedback: Vec<String>,
        report: Vec<String>,
        student_id: Vec<i32>,
        k: i32,
    ) -> Vec<i32> {
        let positive_feedback = positive_feedback
            .into_iter()
            .collect::<std::collections::HashSet<_>>();
        let negative_feedback = negative_feedback
            .into_iter()
            .collect::<std::collections::HashSet<_>>();
        let mut heap = std::collections::BinaryHeap::new();
        let k = k as usize;
        for (r, i) in report.into_iter().zip(student_id.into_iter()) {
            let score = r.split_ascii_whitespace().fold(0, |mut acc, s| {
                if positive_feedback.contains(s) {
                    acc += 3;
                } else if negative_feedback.contains(s) {
                    acc -= 1;
                }
                acc
            });
            if heap.len() >= k && *heap.peek().unwrap() > (std::cmp::Reverse(score), i) {
                heap.pop();
            }
            if heap.len() < k {
                heap.push((std::cmp::Reverse(score), i));
            }
        }
        heap.into_sorted_vec().into_iter().map(|(_, i)| i).collect()
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn test_top_students() {
        assert_eq!(
            Solution::top_students(
                vec_str!["fkeofjpc", "qq", "iio"],
                vec_str!["jdh", "khj", "eget", "rjstbhe", "yzyoatfyx", "wlinrrgcm"],
                vec_str![
                    "rjstbhe eget kctxcoub urrmkhlmi yniqafy fkeofjpc iio yzyoatfyx khj iio",
                    "gpnhgabl qq qq fkeofjpc dflidshdb qq iio khj qq yzyoatfyx",
                    "tizpzhlbyb eget z rjstbhe iio jdh jdh iptxh qq rjstbhe",
                    "jtlghe wlinrrgcm jnkdbd k iio et rjstbhe iio qq jdh",
                    "yp fkeofjpc lkhypcebox rjstbhe ewwykishv egzhne jdh y qq qq",
                    "fu ql iio fkeofjpc jdh luspuy yzyoatfyx li qq v",
                    "wlinrrgcm iio qq omnc sgkt tzgev iio iio qq qq",
                    "d vhg qlj khj wlinrrgcm qq f jp zsmhkjokmb rjstbhe"
                ],
                vec![
                    96537918, 589204657, 765963609, 613766496, 43871615, 189209587, 239084671,
                    908938263
                ],
                3
            ),
            vec![239084671, 589204657, 43871615]
        );
        assert_eq!(
            Solution::top_students(
                vec_str!["smart", "brilliant", "studious"],
                vec_str!["not"],
                vec_str!["this student is studious", "the student is smart"],
                vec![1, 2],
                2
            ),
            vec![1, 2]
        );
        assert_eq!(
            Solution::top_students(
                vec_str!["smart", "brilliant", "studious"],
                vec_str!["not"],
                vec_str!["this student is not studious", "the student is smart"],
                vec![1, 2],
                2
            ),
            vec![2, 1]
        );
    }
}
