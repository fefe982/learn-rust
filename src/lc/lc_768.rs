// https://leetcode.com/problems/max-chunks-to-make-sorted-ii/
// 768. Max Chunks To Make Sorted II
pub struct Solution;
impl Solution {
    pub fn max_chunks_to_sorted(arr: Vec<i32>) -> i32 {
        let mut arr = arr.into_iter().zip(0usize..).collect::<Vec<_>>();
        arr.sort();
        let mut visited = vec![false; arr.len()];
        let mut max_chunks = 1;
        let mut last = 0;
        for (i, &(_, j)) in arr.iter().enumerate() {
            if visited[i] {
                continue;
            }
            if i > last {
                max_chunks += 1;
                last = i;
            }
            visited[i] = true;
            let mut next = j;
            while next != i {
                last = last.max(next);
                visited[next] = true;
                next = arr[next].1;
            }
        }
        max_chunks
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_max_chunks_to_sorted() {
        assert_eq!(Solution::max_chunks_to_sorted(vec![5, 4, 3, 2, 1]), 1);
        assert_eq!(Solution::max_chunks_to_sorted(vec![2, 1, 3, 4, 4]), 4);
    }
}
