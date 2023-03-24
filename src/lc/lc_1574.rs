// https://leetcode.com/problems/shortest-subarray-to-be-removed-to-make-array-sorted/
// 1574. Shortest Subarray to be Removed to Make Array Sorted
pub struct Solution;
impl Solution {
    pub fn find_length_of_shortest_subarray(arr: Vec<i32>) -> i32 {
        let l = arr.len();
        if arr.len() <= 1 {
            return 0;
        }
        let mut front = 1;
        while front < l && arr[front] >= arr[front - 1] {
            front += 1;
        }
        if front == l {
            return 0;
        }
        let mut back = 1;
        while back < l && arr[l - back] >= arr[l - back - 1] {
            back += 1;
        }
        if arr[l - back] >= arr[front - 1] {
            return (l - back - front) as i32;
        }
        let mut short_len = std::cmp::min(l - back, l - front);
        let mut iback = back;
        while iback > 0 && arr[l - iback] < arr[front - 1] {
            iback -= 1;
        }
        short_len = std::cmp::min(short_len, l - front - iback);
        let mut ifront = front;
        while ifront > 1 {
            ifront -= 1;
            while iback < back && arr[l - iback - 1] >= arr[ifront - 1] {
                iback += 1;
            }
            short_len = std::cmp::min(short_len, l - ifront - iback);
        }
        short_len as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn find_length_of_shortest_subarray() {
        assert_eq!(
            Solution::find_length_of_shortest_subarray(vec![1, 2, 3, 10, 4, 2, 3, 5]),
            3
        );
        assert_eq!(
            Solution::find_length_of_shortest_subarray(vec![5, 4, 3, 2, 1]),
            4
        );
        assert_eq!(Solution::find_length_of_shortest_subarray(vec![1, 2, 3]), 0);
    }
}
