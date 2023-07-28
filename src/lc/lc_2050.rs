// https://leetcode.com/problems/parallel-courses-iii/
// 2050. Parallel Courses III
pub struct Solution;
impl Solution {
    pub fn minimum_time(n: i32, relations: Vec<Vec<i32>>, time: Vec<i32>) -> i32 {
        let mut max = 0;
        let mut prev = std::collections::HashMap::<i32, std::collections::HashSet<i32>>::new();
        let mut next = prev.clone();
        let mut prev_time = vec![0; n as usize];
        for r in relations {
            prev.entry(r[1] - 1).or_default().insert(r[0] - 1);
            next.entry(r[0] - 1).or_default().insert(r[1] - 1);
        }
        let mut q = vec![];
        for i in 0..n {
            if !prev.contains_key(&i) {
                q.push(i);
            }
        }
        while let Some(i) = q.pop() {
            let t = time[i as usize] + prev_time[i as usize];
            max = max.max(t);
            for &n in next.remove(&i).iter().flatten() {
                prev_time[n as usize] = prev_time[n as usize].max(t);
                let pset = prev.get_mut(&n).unwrap();
                pset.remove(&i);
                if pset.is_empty() {
                    prev.remove(&n);
                    q.push(n);
                }
            }
        }
        max
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn minimum_time() {
        assert_eq!(
            Solution::minimum_time(3, vec_vec![[1, 3], [2, 3]], vec![3, 2, 5]),
            8
        );
        assert_eq!(
            Solution::minimum_time(
                5,
                vec_vec![[1, 5], [2, 5], [3, 5], [3, 4], [4, 5]],
                vec![1, 2, 3, 4, 5]
            ),
            12
        );
    }
}
