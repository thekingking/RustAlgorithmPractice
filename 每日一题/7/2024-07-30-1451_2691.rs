struct Solution;

impl Solution {
    pub fn get_good_indices(variables: Vec<Vec<i32>>, target: i32) -> Vec<i32> {
        let mut res = Vec::new();
        for (i, row) in variables.iter().enumerate() {
            let a = row[0];
            let mut b = row[1];
            let mut c = row[2];
            let m = row[3];
            let mut cnt = 1;
            while b > 0 {
                cnt = cnt * a % 10;
                b -= 1;
            }   
            let d = cnt;
            cnt = 1;
            while c > 0 {
                cnt = cnt * d % m;
                c -= 1;
            }
            if cnt == target {
                res.push(i as i32);
            }
        }
        res
    }
}
