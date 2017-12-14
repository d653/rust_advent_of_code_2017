#![feature(range_contains)]
extern crate itertools;
extern crate bit_vec;
use itertools::Itertools;
use bit_vec::BitVec;

fn hash(s:&str, ret : &mut Vec<u8>) {
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
    ret.extend((0..256).map(|i|solve(i,&v3))
        .chunks(16).into_iter()
        .map(|c|c.fold(0,|sum,x|sum^x))
        .map(|x|x as u8));
}

fn main(){
    let mut v = Vec::<u8>::new();
    (0..128).for_each(|i|hash(&format!("hwlqcszp-{}",i),&mut v));
    
    let mut b = BitVec::from_bytes(&v);
    let r1 = b.iter().filter(|x| *x).count();
    println!("{}",r1);
    
    
    //copy of advent of code 2017 number 12
    //the list of neighbors is computed differently
    b.negate();
    let mut v = Vec::new();
    let mut p = 0;
    let mut count = 0;
    
    'outer: loop{
        while b[p] { p+=1; if p >= b.len() {break 'outer; }}
        v.push(p);
        while let Some(x) = v.pop() {
            if !b[x] {
                b.set(x,true);
                let x = x as i32;
                [(0i32,1i32),(1,0),(-1,0),(0,-1)].iter().map(|&(a,b)|{(x/128+a,x%128 + b)})
                .filter(|&(a,b)|(0..128).contains(a)&&(0..128).contains(b))
                .map(|(a,b)|(128*a+b) as usize).filter(|&n|!b[n])
                .for_each(|n|v.push(n));
            }
        }
        count += 1;
    }
    
    println!("{}",count);
    
}

