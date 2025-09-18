// https://leetcode.com/problems/design-spreadsheet/
// 3484. Design Spreadsheet
pub struct Spreadsheet {
    cells: std::collections::HashMap<String, i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Spreadsheet {
    pub fn new(_rows: i32) -> Self {
        Self {
            cells: std::collections::HashMap::new(),
        }
    }

    pub fn set_cell(&mut self, cell: String, value: i32) {
        if value == 0 {
            self.cells.remove(&cell);
        } else {
            self.cells.insert(cell, value);
        }
    }

    pub fn reset_cell(&mut self, cell: String) {
        self.cells.remove(&cell);
    }

    pub fn get_value(&self, formula: String) -> i32 {
        formula.split('=').skip(1).next().unwrap().split('+').fold(0, |acc, x| {
            if let Ok(i) = x.parse::<i32>() {
                acc + i
            } else {
                acc + self.cells.get(x).unwrap_or(&0)
            }
        })
    }
}

/**
 * Your Spreadsheet object will be instantiated and called as such:
 * let obj = Spreadsheet::new(rows);
 * obj.set_cell(cell, value);
 * obj.reset_cell(cell);
 * let ret_3: i32 = obj.get_value(formula);
 */
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_3484() {
        let mut obj = Spreadsheet::new(3);
        assert_eq!(obj.get_value("=5+7".to_string()), 12);
        obj.set_cell("A1".to_string(), 10);
        assert_eq!(obj.get_value("=A1+6".to_string()), 16);
        obj.set_cell("B2".to_string(), 15);
        assert_eq!(obj.get_value("=A1+B2".to_string()), 25);
        obj.reset_cell("A1".to_string());
        assert_eq!(obj.get_value("=A1+B2".to_string()), 15);
    }
}
