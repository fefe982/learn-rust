// https://leetcode.com/problems/maximum-segment-sum-after-removals/
// 2382. Maximum Segment Sum After Removals
pub struct Solution;
impl Solution {
    pub fn maximum_segment_sum(nums: Vec<i32>, remove_queries: Vec<i32>) -> Vec<i64> {
        let mut sum = vec![0; nums.len() + 1];
        for i in 0..nums.len() {
            sum[i + 1] = sum[i] + nums[i] as i64;
        }
        let mut segs = std::collections::BTreeMap::<usize, usize>::new();
        let mut res = vec![];
        let mut seg_sum = std::collections::BinaryHeap::<(i64, usize, usize)>::new();
        seg_sum.push((sum[nums.len()], 0, sum.len() - 1));
        segs.insert(0, nums.len() - 1);
        for r in remove_queries {
            let r = r as usize;
            if let Some((&s, &e)) = segs.range(..=r).next_back() {
                segs.remove(&s);
                if r != s {
                    segs.insert(s, r - 1);
                    seg_sum.push((sum[r] - sum[s], s, r - 1));
                }
                if r != e {
                    segs.insert(r + 1, e);
                    seg_sum.push((sum[e + 1] - sum[r + 1], r + 1, e));
                }
            }
            loop {
                if let Some(&(s, l, r)) = seg_sum.peek() {
                    if let Some(e) = segs.get(&l) {
                        if *e == r {
                            res.push(s);
                            break;
                        }
                    }
                } else {
                    res.push(0);
                    break;
                }
                seg_sum.pop();
            }
        }
        res
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_maximum_segment_sum() {
        assert_eq!(
            Solution::maximum_segment_sum(vec![1, 2, 5, 6, 1], vec![0, 3, 2, 4, 1]),
            vec![14, 7, 2, 2, 0]
        );
        assert_eq!(
            Solution::maximum_segment_sum(vec![3, 2, 11, 1], vec![3, 2, 1, 0]),
            vec![16, 5, 3, 0]
        );
    }
}
