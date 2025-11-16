// https://leetcode.cn/problems/shu-ju-liu-zhong-de-zhong-wei-shu-lcof/
// LCR 160. 数据流中的中位数
pub struct MedianFinder {
    mf: super::super::lc02::lc_295::MedianFinder,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MedianFinder {
    /** initialize your data structure here. */
    pub fn new() -> Self {
        Self {
            mf: super::super::lc02::lc_295::MedianFinder::new(),
        }
    }

    pub fn add_num(&mut self, num: i32) {
        self.mf.add_num(num);
    }

    pub fn find_median(&self) -> f64 {
        self.mf.find_median()
    }
}

/**
 * Your MedianFinder object will be instantiated and called as such:
 * let obj = MedianFinder::new();
 * obj.add_num(num);
 * let ret_2: f64 = obj.find_median();
 */
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    use assert_approx_eq::assert_approx_eq;
    #[test]
    fn median() {
        let null = 0.0;
        for case in [
            (
                vec!["MedianFinder", "addNum", "addNum", "findMedian", "addNum", "findMedian"],
                vec_vec![[], [1], [2], [], [3], []],
                vec![null, null, null, 1.50000, null, 2.00000],
            ),
            (
                vec!["MedianFinder", "addNum", "findMedian", "addNum", "findMedian"],
                vec_vec![[], [2], [], [3], []],
                vec![null, null, 2.00000, null, 2.50000],
            ),
        ] {
            let mut mf = MedianFinder::new();
            for ((op, args), expect) in case
                .0
                .into_iter()
                .zip(case.1.into_iter())
                .zip(case.2.into_iter())
                .skip(1)
            {
                match op {
                    "addNum" => mf.add_num(args[0]),
                    "findMedian" => assert_approx_eq!(mf.find_median(), expect),
                    _ => panic!("unknown op"),
                }
            }
        }
    }
}
