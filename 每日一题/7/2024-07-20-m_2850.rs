fn is_less(a: &[i32], b: &[i32]) -> bool {
    a[0] < b[0] || (a[0] == b[0] && a[1] < b[1])
}

fn swap(arr: &mut Vec<Vec<i32>>, i: usize, j: usize) {
    let temp = arr[i].clone();
    arr[i] = arr[j].clone();
    arr[j] = temp;
}

struct Solution;

impl Solution {
    pub fn minimum_moves(grid: Vec<Vec<i32>>) -> i32 {
        let mut more = vec![];
        let mut less = vec![];
        for i in 0..3 {
            for j in 0..3 {
                if grid[i][j] > 1 {
                    for _ in 0..grid[i][j] - 1 {
                        more.push(vec![i as i32, j as i32]);
                    }
                } else if grid[i][j] == 0 {
                    less.push(vec![i as i32, j as i32]);
                }
            }
        }

        let mut ans = i32::MAX;
        loop {
            let mut steps = 0;
            for i in 0..more.len() {
                steps += (less[i][0] - more[i][0]).abs() + (less[i][1] - more[i][1]).abs();
            }
            if steps < ans {
                ans = steps;
            }
            if !Self::next_permutation(&mut more) {
                break;
            }
        }
        ans
    }

    pub fn next_permutation(arr: &mut Vec<Vec<i32>>) -> bool {
        let mut p = -1;
        for i in 0..arr.len() - 1 {
            if is_less(&arr[i], &arr[i + 1]) {
                p = i as i32;
            }
        }
        if p == -1 {
            return false;
        }
        let mut q = -1;
        for j in (p + 1) as usize..arr.len() {
            if is_less(&arr[p as usize], &arr[j]) {
                q = j as i32;
            }
        }
        swap(arr, p as usize, q as usize);
        let mut i = p + 1;
        let mut j = arr.len() as i32 - 1;
        while i < j {
            swap(arr, i as usize, j as usize);
            i += 1;
            j -= 1;
        }
        true
    }
}