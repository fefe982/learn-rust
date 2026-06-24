// https://leetcode.com/problems/longest-uploaded-prefix/
// 2424. Longest Uploaded Prefix
pub struct LUPrefix {
    uploaded: Vec<bool>,
    max_prefix: i32,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl LUPrefix {
    pub fn new(n: i32) -> Self {
        Self {
            uploaded: vec![false; n as usize + 1],
            max_prefix: 0,
        }
    }

    pub fn upload(&mut self, video: i32) {
        self.uploaded[video as usize] = true;
        while self.max_prefix + 1 < self.uploaded.len() as i32 && self.uploaded[(self.max_prefix + 1) as usize] {
            self.max_prefix += 1;
        }
    }

    pub fn longest(&self) -> i32 {
        self.max_prefix
    }
}

/**
 * Your LUPrefix object will be instantiated and called as such:
 * let obj = LUPrefix::new(n);
 * obj.upload(video);
 * let ret_2: i32 = obj.longest();
 */
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn longest_uploaded_prefix() {
        let mut obj = LUPrefix::new(4);
        obj.upload(3);
        assert_eq!(obj.longest(), 0);
        obj.upload(1);
        assert_eq!(obj.longest(), 1);
        obj.upload(2);
        assert_eq!(obj.longest(), 3);
        obj.upload(4);
        assert_eq!(obj.longest(), 4);
    }
}
