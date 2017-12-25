use std::str::Lines;

fn main() {
    let x = "Begin in state A.
Perform a diagnostic checksum after 12523873 steps.

In state A:
  If the current value is 0:
    - Write the value 1.
    - Move one slot to the right.
    - Continue with state B.
  If the current value is 1:
    - Write the value 1.
    - Move one slot to the left.
    - Continue with state E.

In state B:
  If the current value is 0:
    - Write the value 1.
    - Move one slot to the right.
    - Continue with state C.
  If the current value is 1:
    - Write the value 1.
    - Move one slot to the right.
    - Continue with state F.

In state C:
  If the current value is 0:
    - Write the value 1.
    - Move one slot to the left.
    - Continue with state D.
  If the current value is 1:
    - Write the value 0.
    - Move one slot to the right.
    - Continue with state B.

In state D:
  If the current value is 0:
    - Write the value 1.
    - Move one slot to the right.
    - Continue with state E.
  If the current value is 1:
    - Write the value 0.
    - Move one slot to the left.
    - Continue with state C.

In state E:
  If the current value is 0:
    - Write the value 1.
    - Move one slot to the left.
    - Continue with state A.
  If the current value is 1:
    - Write the value 0.
    - Move one slot to the right.
    - Continue with state D.

In state F:
  If the current value is 0:
    - Write the value 1.
    - Move one slot to the right.
    - Continue with state A.
  If the current value is 1:
    - Write the value 1.
    - Move one slot to the right.
    - Continue with state C.
";

    let mut ls = x.split("\n\n");
    let mut h = ls.next().unwrap().lines();
    let bs = h.next().unwrap().chars().nth(15).unwrap() as u8 - b'A';
    let steps: usize = h.next()
        .unwrap()
        .split_whitespace()
        .nth(5)
        .unwrap()
        .parse()
        .unwrap();

    let f1 = |ls: &mut Lines, n| ls.next().unwrap().chars().nth(n).unwrap();
    
    let f2 = |ls: &mut Lines| {
        let w = f1(ls, 22) == '1';
        let m = if f1(ls, 27) == 'r' { 1i32 } else { -1 };
        let ns = f1(ls, 26) as u8 - b'A';
        (w, m, ns)
    };

    let v: Vec<_> = ls.map(|p| {
        let mut ls = p.lines();
        ls.next();
        ls.next();
        let g1 = f2(&mut ls);
        ls.next();
        let g2 = f2(&mut ls);
        [g1, g2]
    }).collect();

    let mut tape = vec![false; 2 * steps + 1];
    let mut p = steps as i32;
    let mut state = bs;

    for _ in 0..steps {
        let t = v[state as usize][tape[p as usize] as usize];
        tape[p as usize] = t.0;
        p += t.1;
        state = t.2;
    }
    let r1 = tape.iter().filter(|&&x| x).count();
    println!("{}", r1);
}

