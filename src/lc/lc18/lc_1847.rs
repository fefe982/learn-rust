// https://leetcode.com/problems/closest-room/
// 1847. Closest Room
pub struct Solution;
impl Solution {
    pub fn closest_room(rooms: Vec<Vec<i32>>, queries: Vec<Vec<i32>>) -> Vec<i32> {
        let mut rooms = rooms;
        rooms.sort_unstable_by_key(|r| std::cmp::Reverse(r[1]));
        let mut queries = queries
            .into_iter()
            .enumerate()
            .map(|x| (x.1[0], x.1[1], x.0))
            .collect::<Vec<_>>();
        queries.sort_unstable_by_key(|q| std::cmp::Reverse(q.1));
        let mut ans = vec![-1; queries.len()];
        let mut roomset = std::collections::BTreeSet::new();
        let mut roomindex = 0;
        for q in queries {
            while roomindex < rooms.len() && rooms[roomindex][1] >= q.1 {
                roomset.insert(rooms[roomindex][0]);
                roomindex += 1;
            }
            let right = roomset.range(q.0..).next();
            let left = roomset.range(..q.0).next_back();
            match (left, right) {
                (None, None) => {}
                (None, Some(&r)) => ans[q.2] = r,
                (Some(&l), None) => ans[q.2] = l,
                (Some(&l), Some(&r)) => ans[q.2] = if q.0 - l <= r - q.0 { l } else { r },
            }
        }
        ans
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn test_closest_room() {
        assert_eq!(
            Solution::closest_room(vec_vec![[2, 2], [1, 2], [3, 2]], vec_vec![[3, 1], [3, 3], [5, 2]]),
            [3, -1, 3]
        );
        assert_eq!(
            Solution::closest_room(
                vec_vec![[1, 4], [2, 3], [3, 5], [4, 1], [5, 2]],
                vec_vec![[2, 3], [2, 4], [2, 5]]
            ),
            [2, 1, 3]
        );
    }
}
