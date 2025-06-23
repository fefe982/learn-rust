// https://leetcode.com/problems/design-a-text-editor/
// 2296. Design a Text Editor
#[derive(Debug)]
pub struct TextEditor {
    vstr: Vec<String>,
    vlinkl: Vec<usize>,
    vlinkr: Vec<usize>,
    vfree: Vec<usize>,
    cursor: usize,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl TextEditor {
    pub fn new() -> Self {
        Self {
            vstr: vec!["".to_string()],
            vlinkl: vec![0],
            vlinkr: vec![0],
            vfree: vec![],
            cursor: 0,
        }
    }

    pub fn add_text(&mut self, text: String) {
        let ncursor = if let Some(free) = self.vfree.pop() {
            self.vstr[free] = text;
            free
        } else {
            self.vstr.push(text);
            self.vlinkr.push(0);
            self.vlinkl.push(0);
            self.vstr.len() - 1
        };
        self.vlinkr[ncursor] = self.vlinkr[self.cursor];
        self.vlinkr[self.cursor] = ncursor;
        self.vlinkl[ncursor] = self.cursor;
        self.vlinkl[self.vlinkr[ncursor]] = ncursor;
        self.cursor = ncursor;
    }

    fn remove_text(&mut self) -> usize {
        if self.cursor == 0 {
            return 0;
        }
        let cursor = self.cursor;
        let ncursor = self.vlinkl[self.cursor];
        let next = self.vlinkr[self.cursor];
        self.vlinkl[next] = ncursor;
        self.vlinkr[ncursor] = next;
        self.cursor = ncursor;
        self.vfree.push(cursor);
        cursor
    }

    pub fn delete_text(&mut self, k: i32) -> i32 {
        let mut len = 0;
        let mut k = k as usize;
        while self.cursor != 0 && k > 0 {
            let removed = self.remove_text();
            let removed_len = self.vstr[removed].len();
            if k >= removed_len {
                k -= removed_len;
                len += removed_len;
            } else {
                let nstr = self.vstr[removed][0..removed_len - k].to_string();
                self.add_text(nstr);
                len += k;
                k = 0;
            }
        }
        len as i32
    }

    fn get_str(&self) -> String {
        let mut ret = String::new();
        let mut cursor = self.cursor;
        while cursor != 0 {
            ret = self.vstr[cursor].clone() + &ret;
            cursor = self.vlinkl[cursor];
            if ret.len() >= 10 {
                return ret[ret.len() - 10..].to_string();
            }
        }
        ret
    }

    pub fn cursor_left(&mut self, k: i32) -> String {
        let mut k = k as usize;
        while self.cursor != 0 && k > 0 {
            let seg_len = self.vstr[self.cursor].len();
            if k >= seg_len {
                self.cursor = self.vlinkl[self.cursor];
                k -= seg_len
            } else {
                let removed = self.remove_text();
                let lstr = self.vstr[removed][0..seg_len - k].to_string();
                let rstr = self.vstr[removed][seg_len - k..].to_string();
                self.add_text(lstr);
                self.add_text(rstr);
                self.cursor = self.vlinkl[self.cursor];
                break;
            }
        }
        self.get_str()
    }

    pub fn cursor_right(&mut self, k: i32) -> String {
        let mut k = k as usize;
        while self.vlinkr[self.cursor] != 0 && k > 0 {
            let seg_len = self.vstr[self.vlinkr[self.cursor]].len();
            if k >= seg_len {
                self.cursor = self.vlinkr[self.cursor];
                k -= seg_len
            } else {
                self.cursor = self.vlinkr[self.cursor];
                let removed = self.remove_text();
                let lstr = self.vstr[removed][0..k].to_string();
                let rstr = self.vstr[removed][k..].to_string();
                self.add_text(lstr);
                self.add_text(rstr);
                self.cursor = self.vlinkl[self.cursor];
                break;
            }
        }
        self.get_str()
    }
}

/**
 * Your TextEditor object will be instantiated and called as such:
 * let obj = TextEditor::new();
 * obj.add_text(text);
 * let ret_2: i32 = obj.delete_text(k);
 * let ret_3: String = obj.cursor_left(k);
 * let ret_4: String = obj.cursor_right(k);
 */
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn test() {
        let null = ();
        for case in [(
            [
                "TextEditor",
                "addText",
                "deleteText",
                "addText",
                "cursorRight",
                "cursorLeft",
                "deleteText",
                "cursorLeft",
                "cursorRight",
            ],
            vec_any![[], ["leetcode"], [4], ["practice"], [3], [8], [10], [2], [6]],
            vec_any![null, null, 4, null, "etpractice", "leet", 4, "", "practi"],
        )] {
            let mut obj = TextEditor::new();
            for ((&method, arg), res) in case.0.iter().zip(case.1.iter()).zip(case.2).skip(1) {
                match method {
                    "addText" => obj.add_text(arg[0].as_string()),
                    "deleteText" => assert_eq!(obj.delete_text(arg[0].as_i32()), res.as_i32()),
                    "cursorLeft" => assert_eq!(obj.cursor_left(arg[0].as_i32()), res.as_string()),
                    "cursorRight" => assert_eq!(obj.cursor_right(arg[0].as_i32()), res.as_string()),
                    _ => panic!(),
                }
            }
        }
    }
}
