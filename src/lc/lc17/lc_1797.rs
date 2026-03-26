// https://leetcode.com/problems/design-authentication-manager/
// 1797. Design Authentication Manager
use std::collections::HashMap;

pub struct AuthenticationManager {
    ttl: i32,
    expiry: HashMap<String, i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl AuthenticationManager {
    pub fn new(time_to_live: i32) -> Self {
        Self {
            ttl: time_to_live,
            expiry: HashMap::new(),
        }
    }

    pub fn generate(&mut self, token_id: String, current_time: i32) {
        self.expiry.insert(token_id, current_time + self.ttl);
    }

    pub fn renew(&mut self, token_id: String, current_time: i32) {
        if let Some(expire_at) = self.expiry.get_mut(&token_id) {
            if *expire_at > current_time {
                *expire_at = current_time + self.ttl;
            }
        }
    }

    pub fn count_unexpired_tokens(&mut self, current_time: i32) -> i32 {
        self.expiry.retain(|_, expire_at| *expire_at > current_time);
        self.expiry.len() as i32
    }
}

/**
 * Your AuthenticationManager object will be instantiated and called as such:
 * let obj = AuthenticationManager::new(timeToLive);
 * obj.generate(tokenId, currentTime);
 * obj.renew(tokenId, currentTime);
 * let ret_3: i32 = obj.count_unexpired_tokens(currentTime);
 */
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn example_1() {
        let mut authentication_manager = AuthenticationManager::new(5);
        authentication_manager.renew("aaa".to_string(), 1);
        authentication_manager.generate("aaa".to_string(), 2);
        assert_eq!(authentication_manager.count_unexpired_tokens(6), 1);
        authentication_manager.generate("bbb".to_string(), 7);
        authentication_manager.renew("aaa".to_string(), 8);
        authentication_manager.renew("bbb".to_string(), 10);
        assert_eq!(authentication_manager.count_unexpired_tokens(15), 0);
    }
}
