// https://leetcode.com/problems/continuous-median-lcci/
// 面试题 17.20. Continuous Median LCCI
pub struct MedianFinder {
    mf: super::super::lc02::lc_295::MedianFinder,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MedianFinder {
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
    #[test]
    fn median() {
        let mut m = MedianFinder::new();
        m.add_num(1);
        m.add_num(2);
        assert_eq!(m.find_median(), 1.5);
        m.add_num(3);
        assert_eq!(m.find_median(), 2.0);
    }
}
