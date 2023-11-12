// https://leetcode.com/problems/bus-routes/
// 815. Bus Routes
pub struct Solution;
impl Solution {
    pub fn num_buses_to_destination(routes: Vec<Vec<i32>>, source: i32, target: i32) -> i32 {
        let mut stop_to_route = std::collections::HashMap::<i32, Vec<i32>>::new();
        for (i, route) in routes.iter().enumerate() {
            for stop in route {
                stop_to_route.entry(*stop).or_default().push(i as i32);
            }
        }
        let mut visited_stop = std::collections::HashSet::new();
        let mut visited_route = std::collections::HashSet::new();
        let mut queue = std::collections::VecDeque::new();
        queue.push_back((source, 0));
        visited_stop.insert(source);
        while let Some((stop, depth)) = queue.pop_front() {
            if stop == target {
                return depth;
            }
            for &route in stop_to_route.get(&stop).unwrap() {
                if !visited_route.contains(&route) {
                    visited_route.insert(route);
                    for &next_stop in routes[route as usize].iter() {
                        if !visited_stop.contains(&next_stop) {
                            queue.push_back((next_stop, depth + 1));
                            visited_stop.insert(next_stop);
                        }
                    }
                }
            }
        }
        -1
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn test_num_buses_to_destination() {
        assert_eq!(
            Solution::num_buses_to_destination(vec_vec![[1, 2, 7], [3, 6, 7]], 1, 6),
            2
        );
        assert_eq!(
            Solution::num_buses_to_destination(vec_vec![[7, 12], [4, 5, 15], [6], [15, 19], [9, 12, 13]], 15, 12),
            -1
        );
    }
}
