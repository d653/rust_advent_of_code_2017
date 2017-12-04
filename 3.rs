fn main() {
    let n = 361527;
    let s = ((n as f64).sqrt() as i32 -1)/2;
    let r = ((n-(2*s+1)*(2*s+1))%(2*s+2) -s -1).abs();
    println!("{}",s+1+r);
}
