use std::collections::{HashMap, HashSet};

fn main() {
    let x = "48/5
25/10
35/49
34/41
35/35
47/35
34/46
47/23
28/8
27/21
40/11
22/50
48/42
38/17
50/33
13/13
22/33
17/29
50/0
20/47
28/0
42/4
46/22
19/35
17/22
33/37
47/7
35/20
8/36
24/34
6/7
7/43
45/37
21/31
37/26
16/5
11/14
7/23
2/23
3/25
20/20
18/20
19/34
25/46
41/24
0/33
3/7
49/38
47/22
44/15
24/21
10/35
6/21
14/50
";

    let mut adj = HashMap::new();

    x.lines().for_each(|l| {
        let mut it = l.split('/').map(|s| s.parse::<usize>().unwrap());
        let a = it.next().unwrap();
        let b = it.next().unwrap();
        adj.entry(a).or_insert(HashSet::new()).insert(b);
        adj.entry(b).or_insert(HashSet::new()).insert(a);
    });

    let (r1, r2) = solve(0, &mut adj);
    println!("{} {} {}", r1, r2.0, r2.1);
}

fn solve(cur: usize, adj: &mut HashMap<usize, HashSet<usize>>) -> (usize, (usize, usize)) {
    let neighbors = adj[&cur].clone();
    let mut ret = (0, (0, 0));
    for new in neighbors {
        adj.get_mut(&cur).unwrap().remove(&new);
        adj.get_mut(&new).unwrap().remove(&cur);
        let s = solve(new, adj);
        ret.0 = std::cmp::max(ret.0, cur + new + s.0);
        ret.1 = std::cmp::max(ret.1, ((s.1).0 + 1, cur + new + (s.1).1));
        adj.get_mut(&cur).unwrap().insert(new);
        adj.get_mut(&new).unwrap().insert(cur);
    }
    ret
}

