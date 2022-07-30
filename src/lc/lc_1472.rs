// https://leetcode.com/problems/design-browser-history/
// 1472. Design Browser History
pub struct BrowserHistory {
    hist: Vec<String>,
    pos: usize,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl BrowserHistory {
    pub fn new(homepage: String) -> Self {
        BrowserHistory {
            hist: vec![homepage],
            pos: 0,
        }
    }

    pub fn visit(&mut self, url: String) {
        self.pos += 1;
        self.hist.truncate(self.pos);
        self.hist.push(url);
    }

    pub fn back(&mut self, steps: i32) -> String {
        self.pos = self.pos.saturating_sub(steps as usize);
        self.hist[self.pos].clone()
    }

    pub fn forward(&mut self, steps: i32) -> String {
        let steps = steps as usize;
        if self.pos + steps >= self.hist.len() {
            self.pos = self.hist.len() - 1;
        } else {
            self.pos += steps;
        }
        self.hist[self.pos].clone()
    }
}

/**
 * Your BrowserHistory object will be instantiated and called as such:
 * let obj = BrowserHistory::new(homepage);
 * obj.visit(url);
 * let ret_2: String = obj.back(steps);
 * let ret_3: String = obj.forward(steps);
 */

#[cfg(test)]
mod tests {
    use super::*;
    enum Op {
        HomePage(String),
        Visit(String),
        Back((i32, String)),
        Forward((i32, String)),
    }
    fn run_test(ops: Vec<Op>) {
        let mut hist = BrowserHistory::new("".to_string());
        for op in ops {
            match op {
                Op::HomePage(homepage) => hist = BrowserHistory::new(homepage),
                Op::Visit(url) => hist.visit(url),
                Op::Back((steps, expect)) => assert_eq!(hist.back(steps), expect),
                Op::Forward((steps, expect)) => assert_eq!(hist.forward(steps), expect),
            }
        }
    }
    #[test]
    fn browser() {
        run_test(vec![
            Op::HomePage("leetcode.com".to_string()),
            Op::Visit("google.com".to_string()),
            Op::Visit("facebook.com".to_string()),
            Op::Visit("youtube.com".to_string()),
            Op::Back((1, "facebook.com".to_string())),
            Op::Back((1, "google.com".to_string())),
            Op::Forward((1, "facebook.com".to_string())),
            Op::Visit("linkedin.com".to_string()),
            Op::Forward((2, "linkedin.com".to_string())),
            Op::Back((2, "google.com".to_string())),
            Op::Back((2, "leetcode.com".to_string())),
        ])
    }
}
