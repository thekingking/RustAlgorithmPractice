fn main() {
    println!("hello, world");
}

struct Solution;

impl Solution {
    pub fn minimum_cost(target: String, words: Vec<String>, costs: Vec<i32>) -> i32 {
        let n = target.len();
        let mut dp = vec![-1; n + 1];
        dp[0] = 0;
        let mut trie = Trie::new();
        for (i, w) in words.into_iter().enumerate() {
            trie.insert(w, costs[i]);
        }
        for i in 0..n {
            if dp[i] == -1 {
                continue;
            }
            for j in (i + 1)..=n {
                if let Some(node) = trie.get_node(&target[i..j]) {
                    if node.is_word {
                        if dp[j] == -1 {
                            dp[j] = dp[i] + node.cost;
                        } else {
                            dp[j] = std::cmp::min(dp[i] + node.cost, dp[j]);
                        }
                    }
                } else {
                    break;
                }
            }
        }
        dp[n]
    }
}