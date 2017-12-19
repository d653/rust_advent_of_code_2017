#[macro_use]
extern crate lazy_static;
extern crate regex;
use regex::Regex;
use std::str::FromStr;
use std::collections::VecDeque;

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
    Snd(Param),
    Set(Param, Param),
    Add(Param, Param),
    Mul(Param, Param),
    Mod(Param, Param),
    Rcv(Param),
    Jgz(Param, Param),
}

impl FromStr for Op {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        lazy_static!{
            static ref SND: Regex = Regex::new(r"^snd (.*?)$").unwrap();
            static ref SET: Regex  = Regex::new(r"^set (.*?) (.*?)$").unwrap();
            static ref ADD: Regex = Regex::new(r"^add (.*?) (.*?)$").unwrap();
            static ref MUL: Regex  = Regex::new(r"^mul (.*?) (.*?)$").unwrap();
            static ref MOD: Regex   = Regex::new(r"^mod (.*?) (.*?)$").unwrap();
            static ref RCV: Regex = Regex::new(r"^rcv (.*?)$").unwrap();
            static ref JGZ: Regex  = Regex::new(r"^jgz (.*?) (.*?)$").unwrap();
        }

        if let Some(c) = SND.captures(s) {
            return Ok(Op::Snd(c[1].parse()?));
        }
        if let Some(c) = SET.captures(s) {
            return Ok(Op::Set(c[1].parse()?, c[2].parse()?));
        }
        if let Some(c) = ADD.captures(s) {
            return Ok(Op::Add(c[1].parse()?, c[2].parse()?));
        }
        if let Some(c) = MUL.captures(s) {
            return Ok(Op::Mul(c[1].parse()?, c[2].parse()?));
        }
        if let Some(c) = MOD.captures(s) {
            return Ok(Op::Mod(c[1].parse()?, c[2].parse()?));
        }
        if let Some(c) = RCV.captures(s) {
            return Ok(Op::Rcv(c[1].parse()?));
        }
        if let Some(c) = JGZ.captures(s) {
            return Ok(Op::Jgz(c[1].parse()?, c[2].parse()?));
        }
        Err(())
    }
}

struct Run<'a> {
    i: i64,
    regs: [i64; 26],
    v: &'a Vec<Op>,
    sndcount: u64,
}

impl<'a> Run<'a> {
    fn new(v: &'a Vec<Op>, second: bool) -> Self {
        let mut regs = [0i64; 26];
        regs[(b'p' - b'a') as usize] = second as i64;
        Run {
            i: 0,
            regs,
            v,
            sndcount: 0,
        }
    }

    fn run(&mut self, qs: &mut VecDeque<i64>, qr: &mut VecDeque<i64>) {
        loop {
            match self.v[self.i as usize] {
                Op::Set(ref p1, ref p2) => {
                    if let &Param::Reg(r) = p1 {
                        self.regs[r] = p2.getval(&self.regs);
                    }
                }

                Op::Add(ref p1, ref p2) => {
                    if let &Param::Reg(r) = p1 {
                        self.regs[r] += p2.getval(&self.regs);
                    }
                }

                Op::Mul(ref p1, ref p2) => {
                    if let &Param::Reg(r) = p1 {
                        self.regs[r] *= p2.getval(&self.regs);
                    }
                }

                Op::Mod(ref p1, ref p2) => {
                    if let &Param::Reg(r) = p1 {
                        self.regs[r] %= p2.getval(&self.regs);
                    }
                }

                Op::Jgz(ref p1, ref p2) => {
                    if p1.getval(&self.regs) > 0 {
                        self.i += p2.getval(&self.regs) - 1;
                    }
                }

                Op::Rcv(ref p1) => {
                    if qr.is_empty() {
                        return;
                    }
                    let p2 = qr.pop_front().unwrap();
                    if let &Param::Reg(r) = p1 {
                        self.regs[r] = p2;
                    }
                }

                Op::Snd(ref p) => {
                    qs.push_back(p.getval(&self.regs));
                    self.sndcount += 1;
                }
            }
            self.i += 1;
        }
    }
}

fn main() {
    let x = r"set i 31
set a 1
mul p 17
jgz p p
mul a 2
add i -1
jgz i -2
add a -1
set i 127
set p 622
mul p 8505
mod p a
mul p 129749
add p 12345
mod p a
set b p
mod b 10000
snd b
add i -1
jgz i -9
jgz a 3
rcv b
jgz b -1
set f 0
set i 126
rcv a
rcv b
set p a
mul p -1
add p b
jgz p 4
snd a
set a b
jgz 1 3
snd b
set f 1
add i -1
jgz i -11
snd a
jgz f -16
jgz a -19
";

    let v: Vec<Op> = x.lines().map(|l| l.parse().unwrap()).collect();
    let mut p1 = Run::new(&v, false);
    let mut p2 = Run::new(&v, true);
    let mut q1 = VecDeque::new();
    let mut q2 = VecDeque::new();
    
    loop{
        p1.run(&mut q1,&mut q2);
        p2.run(&mut q2,&mut q1);
        if q1.is_empty() && q2.is_empty() { break; }
    }
    
    println!("{}",p2.sndcount);
    
}

