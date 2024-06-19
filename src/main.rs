fn main() {
    println!("hello, world");
}

#[warn(dead_code)]
struct Solution;

impl Solution {
    pub fn max_increasing_cells(mat: Vec<Vec<i32>>) -> i32 {
        let mut cnt = std::collections::HashMap::new();
        let mut set = std::collections::HashSet::new();
        for (i, row) in mat.iter().enumerate() {
            for (j, &x) in row.iter().enumerate() {
                cnt.entry(x).and_modify(|l: &mut Vec<(usize, usize)>| l.push((i, j))).or_insert(vec![(i, j)]);
                set.insert(x);
            }
        }
        let mut arr = set.into_iter().collect::<Vec<i32>>();
        arr.sort();
        let mut row = vec![0; mat.len()];
        let mut col = vec![0; mat[0].len()];
        for x in arr {
            let mut mx = Vec::new();
            for &(i, j) in cnt.get(&x).unwrap() {
                mx.push(std::cmp::max(row[i], col[j]) + 1);
            }
            for (&k, &v) in mx.iter().zip(cnt.get(&x).unwrap().iter()).collect::<Vec<(&i32, &(usize, usize))>>() {
                row[v.0] = std::cmp::max(row[v.0], k);
                col[v.1] = std::cmp::max(col[v.1], k);
            }
        }
        std::cmp::max(row.into_iter().max().unwrap(), col.into_iter().max().unwrap())
    }
}