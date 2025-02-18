// https://leetcode.cn/problems/range-frequency-queries
// 2080. Range Frequency Queries
pub struct RangeFreqQuery {
    idx: std::collections::HashMap<i32, Vec<i32>>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl RangeFreqQuery {
    pub fn new(arr: Vec<i32>) -> Self {
        Self {
            idx: arr
                .iter()
                .enumerate()
                .fold(std::collections::HashMap::new(), |mut idx, (i, &v)| {
                    idx.entry(v).or_insert(vec![]).push(i as i32);
                    idx
                }),
        }
    }

    pub fn query(&self, left: i32, right: i32, value: i32) -> i32 {
        if let Some(v) = self.idx.get(&value) {
            let l = v.partition_point(|&x| x < left);
            let r = v.partition_point(|&x| x <= right);
            (r - l) as i32
        } else {
            0
        }
    }
}

/**
 * Your RangeFreqQuery object will be instantiated and called as such:
 * let obj = RangeFreqQuery::new(arr);
 * let ret_1: i32 = obj.query(left, right, value);
 */
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn test_range_freq_query() {
        for test in [(
            vec_vec![[12, 33, 4, 56, 22, 2, 34, 33, 22, 12, 34, 56]],
            vec![[1, 2, 4], [0, 11, 33]],
            vec![1, 2],
        )] {
            let obj = RangeFreqQuery::new(test.0[0].clone());
            for (args, expected) in test.1.into_iter().zip(test.2).skip(1) {
                let ret = obj.query(args[0], args[1], args[2]);
                assert_eq!(expected, ret);
            }
        }
    }
}
