struct Solution;

impl Solution {
    pub fn dest_city(paths: Vec<Vec<String>>) -> String {
        use std::collections::HashMap;
        let mut map = HashMap::new();
        for path in paths.iter() {
            map.insert(path[0].as_str(), path[1].as_str());
        }
        let mut key = paths[0][0].as_str();
        while map.contains_key(key) {
            key = map.get(key).unwrap();
        }
        key.to_string()
    }
}