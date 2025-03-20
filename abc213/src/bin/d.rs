use proconio::marker::*;
use proconio::*;

fn dfs(cur: usize, pre: Option<usize>, g: &Vec<Vec<usize>>, ans: &mut Vec<usize>) {
    // 行きがけ
    ans.push(cur);
    for &nxt in &g[cur] {
        if Some(nxt) != pre {
            dfs(nxt, Some(cur), g, ans);
            // 帰りがけ
            ans.push(cur);
        }
    }
}

fn main() {
    input! {
        n: usize,
        ab: [(Usize1, Usize1); n-1]
    }

    let mut g: Vec<Vec<usize>> = vec![vec![]; n];
    for (a, b) in ab {
        g[a].push(b);
        g[b].push(a);
    }

    for l in &mut g {
        l.sort()
    }

    let mut ans: Vec<usize> = vec![];

    dfs(0, None, &g, &mut ans);

    for res in ans {
        println!("{}", res + 1);
    }
}
