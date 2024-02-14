// https://leetcode.com/problems/minimum-operations-to-halve-array-sum/
// 2208. Minimum Operations to Halve Array Sum
pub struct Solution;
#[derive(Debug, Copy, Default, Hash)]
struct PartialOrdUnwrap<T>(pub T);
impl<T: PartialOrd> PartialEq for PartialOrdUnwrap<T> {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl<T: PartialOrd> Eq for PartialOrdUnwrap<T> {}
impl<T: PartialOrd> PartialOrd for PartialOrdUnwrap<T> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.0.partial_cmp(&other.0)
    }
}
impl<T: PartialOrd> Ord for PartialOrdUnwrap<T> {
    fn cmp(&self, other: &PartialOrdUnwrap<T>) -> std::cmp::Ordering {
        self.0.partial_cmp(&other.0).unwrap()
    }
}
impl<T: Clone> Clone for PartialOrdUnwrap<T> {
    fn clone(&self) -> PartialOrdUnwrap<T> {
        PartialOrdUnwrap(self.0.clone())
    }

    fn clone_from(&mut self, other: &Self) {
        self.0.clone_from(&other.0)
    }
}
impl Solution {
    pub fn halve_array(nums: Vec<i32>) -> i32 {
        let mut cnt = 0;
        let mut sum = 0.0;
        let mut hp = std::collections::BinaryHeap::new();
        for n in nums {
            sum += n as f64;
            hp.push(PartialOrdUnwrap(n as f64));
        }
        let mut reduce = 0.0;
        while reduce * 2.0 < sum {
            let n = hp.pop().unwrap().0 / 2.0;
            reduce += n;
            hp.push(PartialOrdUnwrap(n));
            cnt += 1;
        }
        cnt
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn halve_array() {
        assert_eq!(Solution::halve_array(vec![5, 19, 8, 1]), 3);
        assert_eq!(Solution::halve_array(vec![3, 8, 20]), 3);
    }
}
