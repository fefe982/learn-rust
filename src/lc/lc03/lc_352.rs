// https://leetcode.com/problems/data-stream-as-disjoint-intervals/
// 352. Data Stream as Disjoint Intervals
pub struct SummaryRanges {
    map_se: std::collections::BTreeMap<i32, i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl SummaryRanges {
    pub fn new() -> Self {
        Self {
            map_se: std::collections::BTreeMap::new(),
        }
    }

    pub fn add_num(&mut self, value: i32) {
        let (s1, e1, s0, e0) = {
            let mut es_iter = self.map_se.range(..=value + 1).rev();
            if let Some((&s1, &e1)) = es_iter.next() {
                if let Some((&s0, &e0)) = es_iter.next() {
                    (s1, e1, s0, e0)
                } else {
                    (s1, e1, -2, -2)
                }
            } else {
                (-2, -2, -2, -2)
            }
        };
        if s1 == value + 1 {
            if e0 == value - 1 {
                self.map_se.insert(s0, e1);
                self.map_se.remove(&s1);
            } else {
                self.map_se.insert(value, e1);
                self.map_se.remove(&s1);
            }
        } else if s1 <= value && value <= e1 {
            ()
        } else {
            if e1 == value - 1 {
                self.map_se.insert(s1, value);
            } else {
                self.map_se.insert(value, value);
            }
        }
    }

    pub fn get_intervals(&self) -> Vec<Vec<i32>> {
        self.map_se.iter().map(|(&s, &e)| vec![s, e]).collect()
    }
}

/**
 * Your SummaryRanges object will be instantiated and called as such:
 * let obj = SummaryRanges::new();
 * obj.add_num(value);
 * let ret_2: Vec<Vec<i32>> = obj.get_intervals();
 */
#[cfg(test)]
mod test {
    use super::*;
    use crate::*;
    #[test]
    fn summary_ranges() {
        let mut obj = SummaryRanges::new();
        obj.add_num(1);
        assert_eq!(obj.get_intervals(), vec_vec![[1, 1]]);
        obj.add_num(3);
        assert_eq!(obj.get_intervals(), vec_vec![[1, 1], [3, 3]]);
        obj.add_num(7);
        assert_eq!(obj.get_intervals(), vec_vec![[1, 1], [3, 3], [7, 7]]);
        obj.add_num(2);
        assert_eq!(obj.get_intervals(), vec_vec![[1, 3], [7, 7]]);
        obj.add_num(6);
        assert_eq!(obj.get_intervals(), vec_vec![[1, 3], [6, 7]]);
        let mut obj = SummaryRanges::new();
        obj.add_num(1);
        assert_eq!(obj.get_intervals(), vec_vec![[1, 1]]);
        obj.add_num(9);
        assert_eq!(obj.get_intervals(), vec_vec![[1, 1], [9, 9]]);
        obj.add_num(2);
        assert_eq!(obj.get_intervals(), vec_vec![[1, 2], [9, 9]]);
        let mut obj = SummaryRanges::new();
        obj.add_num(6);
        assert_eq!(obj.get_intervals(), vec_vec![[6, 6]]);
        obj.add_num(6);
        assert_eq!(obj.get_intervals(), vec_vec![[6, 6]]);
        obj.add_num(0);
        assert_eq!(obj.get_intervals(), vec_vec![[0, 0], [6, 6]]);
        obj.add_num(4);
        assert_eq!(obj.get_intervals(), vec_vec![[0, 0], [4, 4], [6, 6]]);
        obj.add_num(8);
        assert_eq!(obj.get_intervals(), vec_vec![[0, 0], [4, 4], [6, 6], [8, 8]]);
        obj.add_num(7);
        assert_eq!(obj.get_intervals(), vec_vec![[0, 0], [4, 4], [6, 8]]);
        obj.add_num(6);
        assert_eq!(obj.get_intervals(), vec_vec![[0, 0], [4, 4], [6, 8]]);
        obj.add_num(4);
        assert_eq!(obj.get_intervals(), vec_vec![[0, 0], [4, 4], [6, 8]]);
        obj.add_num(7);
        assert_eq!(obj.get_intervals(), vec_vec![[0, 0], [4, 4], [6, 8]]);
        obj.add_num(5);
        assert_eq!(obj.get_intervals(), vec_vec![[0, 0], [4, 8]]);
    }
}
