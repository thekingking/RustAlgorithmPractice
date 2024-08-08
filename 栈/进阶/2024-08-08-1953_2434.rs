struct Solution;

impl Solution {
    pub fn robot_with_string(s: String) -> String {
        let mut cnt = vec![0; 26];
        let bs = s.into_bytes();
        for &c in bs.iter(){
            cnt[(c - b'a') as usize] += 1;
        }
        let mut stack = Vec::new();
        let mut res = Vec::new();
        let mut min = 0;
        for c in bs {
            cnt[(c - b'a') as usize] -= 1;
            while min < 25 && cnt[min] == 0 {
                min += 1;
            }
            stack.push(c);
            while stack.len() != 0 && (stack.last().unwrap() - b'a') as usize <= min {
                res.push(stack.pop().unwrap());
            }
        }
        unsafe {String::from_utf8_unchecked(res)}
    }
}
