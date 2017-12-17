
fn main() {

    //not optimal with a vec
    let mut v = vec![0];
    let x = 328usize;
    let mut p = 0;
    
    for i in 1..2018 {
        p = (p + x) % v.len() + 1;
        v.insert(p,i);
    }
    
    let r1 = v[(p+1)%v.len()];
    
    let mut p = 0;
    let r2 = (1..50000000).filter(|i|{
            p = (p + x) % i + 1;
            p == 1
        }).last().unwrap();
    
    println!("{} {}",r1,r2);
    
    
}
