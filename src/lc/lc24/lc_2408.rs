// https://leetcode.com/problems/design-sql/
// 2408. Design SQL
pub struct SQL {
    name_map: std::collections::HashMap<String, usize>,
    ncol: Vec<usize>,
    tables: Vec<Vec<Vec<String>>>,
    valid: Vec<Vec<bool>>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl SQL {
    pub fn new(names: Vec<String>, columns: Vec<i32>) -> Self {
        let columns: Vec<usize> = columns.iter().map(|&x| x as usize).collect();
        let mut name_map = std::collections::HashMap::new();
        for (i, name) in names.into_iter().enumerate() {
            name_map.insert(name, i);
        }
        let n = columns.len();
        Self {
            name_map,
            ncol: columns,
            tables: vec![vec![]; n],
            valid: vec![vec![]; n],
        }
    }

    pub fn ins(&mut self, name: String, row: Vec<String>) -> bool {
        if let Some(&itable) = self.name_map.get(&name) {
            if row.len() != self.ncol[itable] {
                return false;
            }
            let table = &mut self.tables[itable];
            let valid = &mut self.valid[itable];
            table.push(row);
            valid.push(true);
            true
        } else {
            false
        }
    }

    pub fn rmv(&mut self, name: String, row_id: i32) {
        if let Some(&itable) = self.name_map.get(&name) {
            if row_id < 1 || row_id as usize > self.tables[itable].len() {
                return;
            }
            let valid = &mut self.valid[itable];
            valid[row_id as usize - 1] = false;
        }
    }

    pub fn sel(&self, name: String, row_id: i32, column_id: i32) -> String {
        if let Some(&itable) = self.name_map.get(&name) {
            if row_id < 1
                || row_id as usize > self.tables[itable].len()
                || column_id < 1
                || column_id as usize > self.ncol[itable]
            {
                return "<null>".to_string();
            }
            let irow = row_id as usize - 1;
            if self.valid[itable][irow] {
                return self.tables[itable][irow][column_id as usize - 1].clone();
            } else {
                return "<null>".to_string();
            }
        } else {
            "<null>".to_string()
        }
    }

    pub fn exp(&self, name: String) -> Vec<String> {
        if let Some(&itable) = self.name_map.get(&name) {
            let mut r = vec![];
            for i in 0..self.tables[itable].len() {
                if self.valid[itable][i] {
                    r.push(format!("{},{}", i + 1, self.tables[itable][i].join(",")));
                }
            }
            r
        } else {
            vec![]
        }
    }
}

/**
 * Your SQL object will be instantiated and called as such:
 * let obj = SQL::new(names, columns);
 * let ret_1: bool = obj.ins(name, row);
 * obj.rmv(name, rowId);
 * let ret_3: String = obj.sel(name, rowId, columnId);
 * let ret_4: Vec<String> = obj.exp(name);
 */
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn test_sql() {
        let null = 0;
        for test in [
            (
                vec!["SQL", "ins", "sel", "ins", "exp", "rmv", "sel", "exp"],
                vec_any![
                    [["one", "two", "three"], [2, 3, 1]],
                    ["two", ["first", "second", "third"]],
                    ["two", 1, 3],
                    ["two", ["fourth", "fifth", "sixth"]],
                    ["two"],
                    ["two", 1],
                    ["two", 2, 2],
                    ["two"],
                ],
                vec_any![
                    null,
                    true,
                    "third",
                    true,
                    ["1,first,second,third", "2,fourth,fifth,sixth"],
                    null,
                    "fifth",
                    ["2,fourth,fifth,sixth"],
                ],
            ),
            (
                vec!["SQL", "ins", "sel", "rmv", "sel", "ins", "ins"],
                vec_any![
                    [["one", "two", "three"], [2, 3, 1]],
                    ["two", ["first", "second", "third"]],
                    ["two", 1, 3],
                    ["two", 1],
                    ["two", 1, 2],
                    ["two", ["fourth", "fifth"]],
                    ["two", ["fourth", "fifth", "sixth"]],
                ],
                vec_any![null, true, "third", null, "<null>", false, true],
            ),
        ] {
            let mut sql = SQL::new(test.1[0][0].as_vec_string(), test.1[0][1].as_vec_i32());
            for ((&cmd, args), expect) in test.0.iter().zip(test.1.iter()).zip(test.2.iter()).skip(1) {
                match cmd {
                    "ins" => assert_eq!(sql.ins(args[0].as_string(), args[1].as_vec_string()), expect.as_bool()),
                    "rmv" => sql.rmv(args[0].as_string(), args[1].as_i32()),
                    "sel" => assert_eq!(
                        sql.sel(args[0].as_string(), args[1].as_i32(), args[2].as_i32()),
                        expect.as_string()
                    ),
                    "exp" => assert_eq!(sql.exp(args[0].as_string()), expect.as_vec_string()),
                    _ => unreachable!(),
                };
            }
        }
    }
}
