struct Solution;

impl Solution {
    pub fn min_add_to_make_valid(s: String) -> i32 {
        let mut cnt = [0; 3];
        let mut res = 0;
        for c in s.chars() {
            match c {
                '(' => cnt[0] += 1,
                ')' => if cnt[0] > 0 { cnt[0] -= 1 } else { res += 1 },
                '[' => cnt[1] += 1,
                ']' => if cnt[1] > 0 { cnt[1] -= 1 } else { res += 1 },
                '{' => cnt[2] += 1,
                '}' => if cnt[2] > 0 { cnt[2] -= 1 } else { res += 1 },
                _ => (),
            }
        }
        res + cnt.map(|x: i32| x.abs()).into_iter().sum::<i32>()
    }
}
