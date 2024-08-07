use std::collections::{HashMap, HashSet};
impl Solution {
    pub fn accounts_merge(accounts: Vec<Vec<String>>) -> Vec<Vec<String>> {
        let mut email_to_idx = HashMap::new();

    for (i, account) in accounts.iter().enumerate() {
      for acc in account.iter().skip(1) {
        email_to_idx.entry(acc.clone()).or_insert(Vec::new()).push(i);
      }
    }

    fn dfs(idx: usize, accounts: &Vec<Vec<String>>, vis: &mut Vec<bool>, email_to_idx: &HashMap<String, Vec<usize>>, email_set: &mut HashSet<String>) {
      vis[idx] = true;
      for email in accounts[idx].iter().skip(1) {
        if email_set.contains(email) {
          continue;
        }
        email_set.insert(email.clone());
        for &j in email_to_idx.get(email).unwrap() {
          if !vis[j] {
            dfs(j, accounts, vis, email_to_idx, email_set);
          }
        }
      }
    }

    let mut vis = vec![false; accounts.len()];
    let mut res = Vec::new();
    for (i, account) in accounts.iter().enumerate() {
      if vis[i] {
        continue;
      }
      let mut email_set = HashSet::new();
      dfs(i, &accounts, &mut vis, &email_to_idx, &mut email_set);
      let mut ans = email_set.into_iter().collect::<Vec<String>>();
      ans.sort_unstable();
      ans.insert(0, account[0].clone());
      res.push(ans);
    }
    res
    }
}