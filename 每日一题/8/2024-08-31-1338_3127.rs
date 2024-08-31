struct Solution;

impl Solution {
    pub fn can_make_square(grid: Vec<Vec<char>>) -> bool {
        for i in 0..2 {
            for j in 0..2 {
                let mut num = 0;
                for k in 0..2 {
                    for l in 0..2 {
                        if grid[k + i][l + j] == 'B' {
                            num += 1;
                        } else {
                            num -= 1;
                        }
                    }
                }
                if num == 4 || num == 2 || num == -2 || num == -4 {
                    return true;
                }
            }
        }
        false
    }
}
