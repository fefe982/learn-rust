// https://leetcode.com/problems/encode-and-decode-tinyurl/
// 535. Encode and Decode TinyURL
pub struct Codec {
    url_map: std::collections::HashMap<String, String>,
    seq: u64,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Codec {
    pub fn new() -> Self {
        Self {
            url_map: std::collections::HashMap::new(),
            seq: 0,
        }
    }

    // Encodes a URL to a shortened URL.
    pub fn encode(&mut self, long_url: String) -> String {
        if self.url_map.contains_key(&long_url) {
            return self.url_map.get(&long_url).unwrap().to_string();
        }
        let short_url = "http://tinyurl.com/".to_string() + &self.seq.to_string();
        self.seq += 1;
        self.url_map.insert(short_url.clone(), long_url);
        short_url
    }

    // Decodes a shortened URL to its original URL.
    pub fn decode(&self, short_url: String) -> String {
        if let Some(long_url) = self.url_map.get(&short_url) {
            return long_url.to_string();
        }
        String::new()
    }
}

/**
 * Your Codec object will be instantiated and called as such:
 * let obj = Codec::new();
 * let s: String = obj.encode(strs);
 * let ans: VecVec<String> = obj.decode(s);
 */
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_codec() {
        let mut codec = Codec::new();
        let long_url = "https://leetcode.com/problems/design-tinyurl".to_string();
        let short_url = codec.encode(long_url.clone());
        assert_eq!(codec.decode(short_url), long_url);
    }
}
