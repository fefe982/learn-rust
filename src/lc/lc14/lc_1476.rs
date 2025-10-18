// https://leetcode.com/problems/subrectangle-queries/
// 1476. Subrectangle Queries
pub struct SubrectangleQueries {
    r: Vec<Vec<i32>>,
    q: Vec<(i32, i32, i32, i32, i32)>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl SubrectangleQueries {
    pub fn new(rectangle: Vec<Vec<i32>>) -> Self {
        Self {
            r: rectangle,
            q: vec![],
        }
    }

    pub fn update_subrectangle(&mut self, row1: i32, col1: i32, row2: i32, col2: i32, new_value: i32) {
        self.q.push((row1, col1, row2, col2, new_value));
    }

    pub fn get_value(&self, row: i32, col: i32) -> i32 {
        for i in (0..self.q.len()).rev() {
            let (r1, c1, r2, c2, v) = self.q[i];
            if row >= r1 && row <= r2 && col >= c1 && col <= c2 {
                return v;
            }
        }
        self.r[row as usize][col as usize]
    }
}

/**
 * Your SubrectangleQueries object will be instantiated and called as such:
 * let obj = SubrectangleQueries::new(rectangle);
 * obj.update_subrectangle(row1, col1, row2, col2, newValue);
 * let ret_2: i32 = obj.get_value(row, col);
 */
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn subrectangle_queries() {
        let mut obj = SubrectangleQueries::new(vec_vec![[1, 2, 1], [4, 3, 4], [3, 2, 1], [1, 1, 1]]);
        assert_eq!(obj.get_value(0, 2), 1);
        obj.update_subrectangle(0, 0, 3, 2, 5);
        assert_eq!(obj.get_value(0, 2), 5);
        assert_eq!(obj.get_value(3, 1), 5);
        obj.update_subrectangle(3, 0, 3, 2, 10);
        assert_eq!(obj.get_value(3, 1), 10);
        assert_eq!(obj.get_value(0, 2), 5);
    }
}
