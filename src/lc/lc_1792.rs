// https://leetcode.com/problems/maximum-average-pass-ratio/
// 1792. Maximum Average Pass Ratio
pub struct Solution;
struct Cls(i32, i32, f64);
impl Cls {
    fn new(a: i32, b: i32) -> Self {
        Self(a, b, (b - a) as f64 / (b as f64 * (b + 1) as f64))
    }
    fn score(&self) -> f64 {
        self.0 as f64 / self.1 as f64
    }
}
impl std::cmp::PartialEq for Cls {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0 && self.1 == other.1
    }
}
impl std::cmp::Eq for Cls {}
impl std::cmp::PartialOrd for Cls {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.2.partial_cmp(&other.2)
    }
}
impl std::cmp::Ord for Cls {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.2.partial_cmp(&other.2).unwrap()
    }
}
impl Solution {
    pub fn max_average_ratio(classes: Vec<Vec<i32>>, extra_students: i32) -> f64 {
        let n = classes.len() as f64;
        let mut h = std::collections::BinaryHeap::new();
        for c in classes {
            h.push(Cls::new(c[0], c[1]));
        }
        for _ in 0..extra_students {
            let c = h.pop().unwrap();
            h.push(Cls::new(c.0 + 1, c.1 + 1));
        }
        h.into_iter().map(|c| c.score()).sum::<f64>() / n
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    use assert_approx_eq::assert_approx_eq;
    #[test]
    fn max_average_ratio() {
        assert_approx_eq!(
            Solution::max_average_ratio(vec_vec![[1, 2], [3, 5], [2, 2]], 2),
            0.78333,
            1.0e-5
        );
        assert_approx_eq!(
            Solution::max_average_ratio(vec_vec![[2, 4], [3, 9], [4, 5], [2, 10]], 4),
            0.53485,
            1.0e-5
        );
    }
}
