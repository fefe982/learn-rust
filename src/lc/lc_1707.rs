// https://leetcode.com/problems/maximum-xor-with-an-element-from-array/
// 1707. Maximum XOR With an Element From Array
pub struct Solution;
impl Solution {
    fn query(nums: &Vec<i32>, x: i32, m: i32) -> i32 {
        let pos = nums.partition_point(|&n| n <= m);
        if pos == 0 {
            return -1;
        }
        let mut mask = 1 << 30;
        let mut slice = &nums[..pos];
        while slice.len() > 1 {
            let pos = slice.partition_point(|x| x & mask == 0);
            if pos != 0 && pos != slice.len() {
                if x & mask == 0 {
                    slice = &slice[pos..];
                } else {
                    slice = &slice[..pos];
                }
            }
            mask >>= 1;
        }
        slice[0] ^ x
    }
    pub fn maximize_xor(nums: Vec<i32>, queries: Vec<Vec<i32>>) -> Vec<i32> {
        let mut nums = nums;
        nums.sort_unstable();
        nums = nums
            .into_iter()
            .scan(-1, |s, x| {
                let ss = *s;
                *s = x;
                Some((ss, x))
            })
            .filter_map(|(ss, x)| if ss != x { Some(x) } else { None })
            .collect();
        queries.into_iter().map(|q| Self::query(&nums, q[0], q[1])).collect()
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn test_maximize_xor() {
        assert_eq!(
            Solution::maximize_xor(
                vec![
                    0, 3090096, 4194304, 845573555, 0, 778971744, 267447, 0, 4194304, 4194304, 446819176, 45001354,
                    4194304, 3777171, 873518539, 0, 1165390, 4194304, 3372544, 0
                ],
                vec_vec![
                    [4194304, 1000000000],
                    [280475304, 1000000000],
                    [536870912, 1000000000],
                    [1000000000, 1000000000],
                    [668787504, 391894850],
                    [536870912, 3143980],
                    [1000000000, 619304916],
                    [453771895, 1000000000],
                    [1000000000, 1000000000],
                    [4194304, 1000000000],
                    [536870912, 66331],
                    [308138274, 1000000000],
                    [536870912, 240619799],
                    [4194304, 1000000000],
                    [536870912, 4194304],
                    [315860359, 1000000000],
                    [4194304, 4194304],
                    [1000000000, 1000000000],
                    [4194304, 1000000000],
                    [1000000000, 994164423]
                ]
            ),
            [
                877712843, 1054448840, 983690088, 1004194304, 670287232, 539961008, 1004194304, 895625239, 1004194304,
                877712843, 536870912, 1010039618, 581872266, 877712843, 541065216, 1019056103, 7971475, 1004194304,
                877712843, 1004194304
            ]
        );
        assert_eq!(
            Solution::maximize_xor(vec![0, 1, 2, 3, 4], vec_vec![[3, 1], [1, 3], [5, 6]]),
            [3, 3, 7]
        );
        assert_eq!(
            Solution::maximize_xor(vec![5, 2, 4, 6, 6, 3], vec_vec![[12, 4], [8, 1], [6, 3]]),
            [15, -1, 5]
        );
    }
}
