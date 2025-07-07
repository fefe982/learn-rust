// https://leetcode.cn/problems/keys-and-rooms/
// 841. Keys and Rooms
pub struct Solution;
impl Solution {
    pub fn can_visit_all_rooms(rooms: Vec<Vec<i32>>) -> bool {
        let mut visited = vec![false; rooms.len()];
        let mut stack = vec![0];
        visited[0] = true;
        while let Some(i) = stack.pop() {
            for &j in rooms[i].iter() {
                if !visited[j as usize] {
                    stack.push(j as usize);
                    visited[j as usize] = true;
                }
            }
        }
        visited.iter().all(|&x| x)
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn can_visit_all_rooms() {
        assert_eq!(Solution::can_visit_all_rooms(vec_vec![[1], [2], [3], []]), true);
        assert_eq!(
            Solution::can_visit_all_rooms(vec_vec![[1, 3], [3, 0, 1], [2], [0]]),
            false
        );
    }
}
