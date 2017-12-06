use std::collections::HashMap;

fn main(){
    let mut v : Vec<u32> = "4	10	4	1	8	4	9	14	5	1	14	15	0	15	3	5"
        .split_whitespace()
        .map(|s|s.parse().unwrap())
        .collect();
    
    let mut hs : HashMap<Vec<u32>,u32> = HashMap::new();
    let len = v.len();
    let mut j = 0;
    
    while ! hs.contains_key(&v) {
        hs.insert(v.clone(),j);
        let pmax = v.iter().enumerate().rev().max_by_key(|&(_, e)| e).unwrap().0;
        let mut i = pmax+1;
        let mut x = v[pmax];
        v[pmax] = 0;
        while x > 0 {
            v[i%len] +=1;
            x -=1;
            i+=1;
        }
        j+=1;
    }
    println!("{} {}",j, j-hs[&v]);
}
