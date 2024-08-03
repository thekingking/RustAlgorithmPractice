struct Solution;

impl Solution {
    pub fn validate_stack_sequences(pushed: Vec<i32>, popped: Vec<i32>) -> bool {
        let mut stack = Vec::new();
        let mut i = 0;
        let mut j = 0;
        while i < pushed.len() {
            stack.push(pushed[i]);
            while let Some(&x) = stack.last() {
                if x != popped[j] {
                    break;
                }
                j += 1;
                stack.pop();
            }
            i += 1;
        }
        while let Some(x) = stack.pop() {
            if x != popped[j] {
                return false;
            }
            j += 1;
        }
        true
    }
}