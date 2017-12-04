fn main() {
    let n = 361527;
    let s = ((n as f64).sqrt() as i32 -1)/2;
    let r = ((n-(2*s+1)*(2*s+1))%(2*s+2) -s -1).abs();
    println!("{}",s+1+r);
    
    // too lazy to do the second part in Rust
    // it can be solved with this one line bash script
    // curl -s http://oeis.org/A141481/b141481.txt | awk '{ if( int($2) > 361527 ){print $2; exit(0);} }'
    
}
