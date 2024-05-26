struct Solution;

impl Solution {
    pub fn compressed_string(word: String) -> String {
        let mut pre = word.chars().nth(0).unwrap();
        let mut num = 1;
        let mut answer = Vec::new();
        for c in word[1..].chars() {
            if c != pre || num == 9 {
                let mut temp = num.to_string();
                temp.push(pre);
                answer.push(temp);
                pre = c;
                num = 0;
            }
            num += 1;
        }
        let mut temp = num.to_string();
        temp.push(pre);
        answer.push(temp);
        answer.join("")
    }
}