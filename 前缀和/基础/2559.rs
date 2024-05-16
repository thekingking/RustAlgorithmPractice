struct Solution;

// 经典前缀和，读题不仔细耽误不少时间，要求字符串首尾皆包含元音字母
impl Solution {
    pub fn vowel_strings(words: Vec<String>, queries: Vec<Vec<i32>>) -> Vec<i32> {
        let arr = [b'a', b'e', b'i', b'o', b'u'];
        let mut res = vec![0; words.len() + 1];
        for (i, x) in words.iter().enumerate() {
            let word = x.as_bytes();
            res[i + 1] = res[i] + (arr.contains(word.first().unwrap()) && arr.contains(word.last().unwrap())) as i32;
        }
        queries.iter().map(|x| res[x[1] as usize + 1] - res[x[0] as usize]).collect()
    }
}