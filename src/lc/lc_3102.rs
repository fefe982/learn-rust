// https://leetcode.com/problems/minimize-manhattan-distances/
// 3102. Minimize Manhattan Distances
pub struct Solution;
impl Solution {
    fn add<F>(v: &mut Vec<(i32, Vec<usize>)>, i: usize, x: i32, compare: F)
    where
        F: Fn(i32, i32) -> std::cmp::Ordering,
    {
        match compare(x, v[0].0) {
            std::cmp::Ordering::Equal => v[0].1.push(i),
            std::cmp::Ordering::Less => {
                v.pop();
                v.insert(0, (x, vec![i]));
            }
            std::cmp::Ordering::Greater => match compare(x, v[1].0) {
                std::cmp::Ordering::Equal => v[1].1.push(i),
                std::cmp::Ordering::Less => v[1] = (x, vec![i]),
                std::cmp::Ordering::Greater => (),
            },
        }
    }
    fn remove_one(vmax: &Vec<(i32, Vec<usize>)>, vmin: &Vec<(i32, Vec<usize>)>, i: usize) -> i32 {
        let mut max = vmax[0].0;
        let mut min = vmin[0].0;
        if vmax[0].1.len() == 1 && vmax[0].1[0] == i {
            max = vmax[1].0;
        }
        if vmin[0].1.len() == 1 && vmin[0].1[0] == i {
            min = vmin[1].0;
        }
        max - min
    }
    fn remove(
        from_max: &Vec<(i32, Vec<usize>)>,
        from_min: &Vec<(i32, Vec<usize>)>,
        other_max: &Vec<(i32, Vec<usize>)>,
        other_min: &Vec<(i32, Vec<usize>)>,
    ) -> i32 {
        let mut res = from_max[0].0 - from_min[0].0;
        if from_max[0].1.len() == 1 {
            res =
                res.min((from_max[1].0 - from_min[0].0).max(Self::remove_one(other_max, other_min, from_max[0].1[0])));
        }
        if from_min[0].1.len() == 1 {
            res =
                res.min((from_max[0].0 - from_min[1].0).max(Self::remove_one(other_max, other_min, from_min[0].1[0])));
        }
        res
    }
    pub fn minimum_distance(points: Vec<Vec<i32>>) -> i32 {
        let mut submin = vec![(i32::MAX, vec![]); 2];
        let mut submax = vec![(i32::MIN, vec![]); 2];
        let mut addmin = submin.clone();
        let mut addmax = submax.clone();
        for (i, p) in points.into_iter().enumerate() {
            let sub = p[0] - p[1];
            let add = p[0] + p[1];
            Self::add(&mut submin, i, sub, |a, b| a.cmp(&b));
            Self::add(&mut submax, i, sub, |a, b| b.cmp(&a));
            Self::add(&mut addmin, i, add, |a, b| a.cmp(&b));
            Self::add(&mut addmax, i, add, |a, b| b.cmp(&a));
        }
        let sub_dist = submax[0].0 - submin[0].0;
        let add_dist = addmax[0].0 - addmin[0].0;
        let mut res = i32::MAX;
        if add_dist >= sub_dist {
            res = res.min(Self::remove(&addmax, &addmin, &submax, &submin));
        }
        if add_dist <= sub_dist {
            res = res.min(Self::remove(&submax, &submin, &addmax, &addmin));
        }
        res
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn test_minimum_distance() {
        assert_eq!(
            Solution::minimum_distance(vec_vec![[3, 10], [5, 15], [10, 2], [4, 4]]),
            12
        );
        assert_eq!(Solution::minimum_distance(vec_vec![[1, 1], [1, 1], [1, 1]]), 0);
    }
}
