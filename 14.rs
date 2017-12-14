#![feature(range_contains)]
extern crate itertools;
extern crate bit_vec;
use itertools::Itertools;
use bit_vec::BitVec;
extern crate petgraph;
use petgraph::graph::Graph;
use petgraph::Undirected;

fn hash(s:&str) -> Vec<u8>{
    //copy of advent of code 2017 number 10
    //we add the bytes to ret instead of returning a string

    let cl = 256;

    let solve = |mut x:i32, v: &Vec<i32>|->i32{
        let mut pos : i32 = ((v.len() * (v.len()-1) / 2) as i32 + v.iter().sum::<i32>())%cl;
        for (i,&len) in v.iter().enumerate().rev() {
            pos = (pos - (i as i32+len) + 128*cl)%cl;
            if x < pos { x += cl }
            if x >= pos && x < pos+len { x = (pos+len)-(x-pos+1);}
            x = x % cl;
        }
        x
    };   
    
    let s2 = "17, 31, 73, 47, 23";
    let v2 = s2.split(", ")
        .map(|x|x.parse().unwrap())
        .collect::<Vec<i32>>();
    let v3 = (0..64).flat_map(|_|s.chars().map(|c|c as i32)
        .chain(v2.iter().cloned()))
        .collect::<Vec<i32>>();
    (0..256).map(|i|solve(i,&v3))
        .chunks(16).into_iter()
        .map(|c|c.fold(0,|sum,x|sum^x))
        .map(|x|x as u8).collect()
}

fn main(){
    let v : Vec<Vec<u8>> = (0..128).map(|i|hash(&format!("hwlqcszp-{}",i))).collect();
    
    
    let is_one = |i:i32,j:i32|{ i>=0&&j>=0&&i<128&&j<128&& ((v[i as usize][(j/8) as usize]>>(7-j%8))&1) != 0};
    
    let mut g = Graph::<(), (),Undirected>::from_edges(
        (0i32..128).cartesian_product(0i32..128).filter(|&(x,y)|is_one(x,y))
        .flat_map(|(x,y)|{
            [(0i32,1i32),(1,0),(-1,0),(0,-1)].iter()
            .map(move |&(i,j)|(x+i,y+j)).filter(|&(a,b)|is_one(a,b))
            .map(move |(a,b)|((128*x+y) as u32,(128*a+b) as u32))
        }));
        
    g.retain_nodes(|_,idx|{let i = idx.index();is_one((i/128) as i32,(i%128) as i32)});
    
    let r1 = v.iter().map(|x|x.iter().map(|x|x.count_ones()).sum::<u32>()).sum::<u32>();
    let r2 = petgraph::algo::connected_components(&g);
    println!("{} {}",r1,r2);
    
}
