// https://leetcode.com/problems/meeting-rooms-iii/
// 2402. Meeting Rooms III
pub struct Solution;
impl Solution {
    pub fn most_booked(n: i32, meetings: Vec<Vec<i32>>) -> i32 {
        let mut meetings = meetings;
        meetings.sort_unstable();
        let mut rooms = vec![0; n as usize];
        let mut empty = (0..n as usize)
            .map(|i| std::cmp::Reverse(i))
            .collect::<std::collections::BinaryHeap<_>>();
        let mut booked = std::collections::BinaryHeap::<std::cmp::Reverse<(i64, usize)>>::new();
        let mut shift = 0i64;
        let mut empty_time = 0i64;
        for meeting in meetings {
            let mut start = meeting[0] as i64;
            let mut end = meeting[1] as i64;
            if start < empty_time {
                shift = empty_time - start;
                start += shift;
                end += shift;
            }
            while let Some(&book) = booked.peek() {
                if book.0 .0 <= start {
                    booked.pop();
                    empty.push(std::cmp::Reverse(book.0 .1));
                    empty_time = start;
                } else {
                    break;
                }
            }
            if empty.is_empty() {
                let b = booked.pop().unwrap();
                shift += b.0 .0 - start;
                end = b.0 .0 - start + end;
                start = b.0 .0;
                empty.push(std::cmp::Reverse(b.0 .1));
                empty_time = start;
            }
            while let Some(&book) = booked.peek() {
                if book.0 .0 <= start {
                    booked.pop();
                    empty.push(std::cmp::Reverse(book.0 .1))
                } else {
                    break;
                }
            }
            let room = empty.pop().unwrap().0;
            rooms[room] += 1;
            booked.push(std::cmp::Reverse((end, room)));
        }
        rooms
            .into_iter()
            .enumerate()
            .fold((0, 0), |(maxi, max_count), (i, count)| {
                if count > max_count {
                    (i, count)
                } else {
                    (maxi, max_count)
                }
            })
            .0 as i32
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn test_most_booked() {
        assert_eq!(Solution::most_booked(2, vec_vec![[0, 10], [1, 5], [2, 7], [3, 4]]), 0);
        assert_eq!(
            Solution::most_booked(3, vec_vec![[1, 20], [2, 10], [3, 5], [4, 9], [6, 8]]),
            1
        );
    }
}
