struct Solution;

impl Solution {
    pub fn max_height_of_triangle(red: i32, blue: i32) -> i32 {
        fn find(mut l: i32, mut s: i32) -> i32 {
            let mut index = 0;
            let mut num = 1;
            while l >= 0 && s >= 0 {
                if index % 2 == 0 {
                    l -= num;
                } else {
                    s -= num;
                }
                index += 1;
                num += 1;
            }
            index - 1
        }
        std::cmp::max(find(red, blue), find(blue, red))
    }
}