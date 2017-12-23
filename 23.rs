#![feature(iterator_step_by)] 
#[macro_use]
extern crate lazy_static;
extern crate regex;
use regex::Regex;
use std::str::FromStr;

enum Param {
    Reg(usize),
    Val(i64),
}

impl FromStr for Param {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Ok(n) = s.parse() {
            return Ok(Param::Val(n));
        }
        return Ok(Param::Reg(
            (s.chars().next().unwrap() as u8 - b'a') as usize,
        ));
    }
}

impl Param {
    fn getval(&self, regs: &[i64]) -> i64 {
        match self {
            &Param::Reg(x) => regs[x],
            &Param::Val(x) => x,
        }
    }
}

enum Op {
    Set(Param, Param),
    Sub(Param, Param),
    Mul(Param, Param),
    Jnz(Param, Param),
}

impl FromStr for Op {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        lazy_static!{
            static ref SET: Regex  = Regex::new(r"^set (.*?) (.*?)$").unwrap();
            static ref SUB: Regex = Regex::new(r"^sub (.*?) (.*?)$").unwrap();
            static ref MUL: Regex  = Regex::new(r"^mul (.*?) (.*?)$").unwrap();
            static ref JNZ: Regex  = Regex::new(r"^jnz (.*?) (.*?)$").unwrap();
        }

        if let Some(c) = SET.captures(s) {
            return Ok(Op::Set(c[1].parse()?, c[2].parse()?));
        }
        if let Some(c) = SUB.captures(s) {
            return Ok(Op::Sub(c[1].parse()?, c[2].parse()?));
        }
        if let Some(c) = MUL.captures(s) {
            return Ok(Op::Mul(c[1].parse()?, c[2].parse()?));
        }
        if let Some(c) = JNZ.captures(s) {
            return Ok(Op::Jnz(c[1].parse()?, c[2].parse()?));
        }
        Err(())
    }
}

fn main() {
    let x = "set b 79
set c b
jnz a 2
jnz 1 5
mul b 100
sub b -100000
set c b
sub c -17000
set f 1
set d 2
set e 2
set g d
mul g e
sub g b
jnz g 2
set f 0
sub e -1
set g e
sub g b
jnz g -8
sub d -1
set g d
sub g b
jnz g -13
jnz f 2
sub h -1
set g b
sub g c
jnz g 2
jnz 1 3
sub b -17
jnz 1 -23
";


    let v: Vec<Op> = x.lines().map(|l| l.parse().unwrap()).collect();

    let mut regs = [0i64; 26];
    let mut i = 0i64;
    let mut mulcount = 0i64;
    
    while i >= 0 && i < v.len() as i64 {
        
        match v[i as usize] {
            
            Op::Set(ref p1,ref p2) => {
                if let &Param::Reg(r) = p1 { regs[r] = p2.getval(&regs); }
            }
            
            Op::Sub(ref p1,ref p2) => {
                if let &Param::Reg(r) = p1 { regs[r] -= p2.getval(&regs); }
            }
            
            Op::Mul(ref p1,ref p2) => {
                if let &Param::Reg(r) = p1 { regs[r] *= p2.getval(&regs); }
                mulcount += 1;
            }
            
            Op::Jnz(ref p1,ref p2) => {
                if p1.getval(&regs) != 0 {
                    i += p2.getval(&regs)-1;
                }
            }
        }
        i+=1;
    }
    
    println!("{}",mulcount);

    //not optimal
    let iscomposite = |x:i64|{(2..x).any(|t|x%t==0)};
    let r2 = (107900..124900+1).step_by(17).filter(|&x|iscomposite(x)).count();
    println!("{}",r2);
    
    
}
