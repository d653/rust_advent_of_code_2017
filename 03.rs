#![feature(inclusive_range_syntax)]
extern crate itertools;
use itertools::Itertools;

fn main(){
    // part 1
    let n = 361527;
    let s = ((n as f64).sqrt() as i32 -1)/2;
    let r = ((n-(2*s+1)*(2*s+1))%(2*s+2) -s -1).abs();
    println!("{}",s+1+r);
    
    // part 2
    let mut mat = [[0;1000];1000];
    let mov = [(1,0),(0,1),(-1,0),(0,-1)];    
    let moves = (0..).interleave(0..).enumerate()
        .flat_map(|(i,x)|(1..x)
        .map(move |_|mov[i%4]));
    
    let mut pos = (500,500);
    mat[pos.0 as usize][pos.1 as usize] = 1;
    
    for x in moves {
        pos = (pos.0+x.0,pos.1+x.1);
        let s = (-1..=1).cartesian_product(-1..=1)
            .map(|(a,b)|mat[(pos.0+a) as usize][(pos.1+b) as usize])
            .sum();
        mat[pos.0 as usize][pos.1 as usize] = s;
        
        if s > n {
            println!("{}",s);
            break;
        }
    }
    
}
