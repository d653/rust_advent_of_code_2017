extern crate itertools;
use itertools::Itertools;

fn main(){
    let s = "157,222,1,2,177,254,0,228,159,140,249,187,255,51,76,30";
    let v = s.split(',')
        .map(|x|x.parse().unwrap())
        .collect::<Vec<i32>>();
    let cl = 256;

    //part 1, in linear time     
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
    println!("{}",solve(0,&v)*solve(1,&v));
    
    //part 2 (reusing solution of part 1, not optimal...)
    let s2 = "17, 31, 73, 47, 23";
    let v2 = s2.split(", ")
        .map(|x|x.parse().unwrap())
        .collect::<Vec<i32>>();
    let v3 = (0..64).flat_map(|_|s.chars().map(|c|c as i32)
        .chain(v2.iter().cloned()))
        .collect::<Vec<i32>>();
    let r : String = (0..256).map(|i|solve(i,&v3))
        .chunks(16).into_iter()
        .map(|c|c.fold(0,|sum,x|sum^x))
        .map(|x|format!("{:02x}",x as u8)).collect();
    println!("{}",r);
    
}

