struct Solution;

impl Solution {
    pub fn report_spam(message: Vec<String>, banned_words: Vec<String>) -> bool {
        use std::collections::HashSet;

        let cnt: HashSet<String> = HashSet::from_iter(banned_words);
        let mut res = 0;
        for s in message {
            if cnt.contains(&s) {
                res += 1;
            }
            if res > 1 {
                return true;
            }
        }
        false
    }
}