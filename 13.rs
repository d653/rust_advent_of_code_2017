fn main(){
    let x = r"0: 3
1: 2
2: 5
4: 4
6: 6
8: 4
10: 8
12: 8
14: 6
16: 8
18: 6
20: 6
22: 8
24: 12
26: 12
28: 8
30: 12
32: 12
34: 8
36: 10
38: 9
40: 12
42: 10
44: 12
46: 14
48: 14
50: 12
52: 14
56: 12
58: 12
60: 14
62: 14
64: 12
66: 14
68: 14
70: 14
74: 24
76: 14
80: 18
82: 14
84: 14
90: 14
94: 17
";

    let v : Vec<Vec<usize>> = x.lines().map(|l|l.split(": ").map(|s|s.parse().unwrap()).collect()).collect();
    let f = |d:usize|{v.iter().filter(move |x|(x[0]+d)%(2*x[1]-2)==0)};
    let r1 : usize = f(0).map(|x|x[0]*x[1]).sum();
    let r2 = (0..).skip_while(|&i|f(i).next().is_some()).next().unwrap();
    println!("{} {}",r1,r2);
    
    
}
