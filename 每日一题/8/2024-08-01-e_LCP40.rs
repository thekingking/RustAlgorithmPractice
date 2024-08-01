struct Solution;

impl Solution {
    pub fn maxmium_score(cards: Vec<i32>, cnt: i32) -> i32 {
        let mut even = Vec::new();
        let mut odd = Vec::new();
        for &x in &cards {
            if x % 2 == 0 {
                even.push(x);
            } else {
                odd.push(x);
            }
        }
        if even.len() == 0 && cnt % 2 == 1 || cnt as usize == cards.len() && odd.len() % 2 == 1 {
            return 0;
        }
        even.sort_unstable_by_key(|&x| -x);
        odd.sort_unstable_by_key(|&x| -x);
        let mut res = 0;
        let mut num = 0;
        let mut i = 0;
        while i + 1 < odd.len() && num + 2 <= cnt {
            res += odd[i] + odd[i + 1];
            num += 2;
            i += 2;
        }
        let mut j = 0;
        while j < even.len() && num < cnt {
            res += even[j];
            j += 1;
            num += 1;
        }
        if num != cnt {
            return 0;
        }
        if odd.len() == 0 || odd.len() == 1 || even.len() == 0 {
            return res;
        }
        while i > 1 && j + 1 < even.len() {
            if odd[i - 1] + odd[i - 2] >= even[j] + even[j + 1] {
                break;
            }
            res += even[j] + even[j + 1] - odd[i - 1] - odd[i - 2];
            i -= 2;
            j += 2;
        }
        res
    }
}