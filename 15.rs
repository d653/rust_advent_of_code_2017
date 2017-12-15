#![feature(conservative_impl_trait)]

fn gen_iter<'a>(state:&'a mut u64, mult : u64) -> impl Iterator<Item=u64> + 'a {
    (0..).map(move |_|{
        *state = (*state*mult)%2147483647;
        *state&0x0ffff}
    )
}

fn main(){
    //part 1
    let mut statea = 783u64;
    let mut stateb = 325u64;
    
    let a1 = gen_iter(&mut statea, 16807);
    let b1 = gen_iter(&mut stateb, 48271);

    let r1 = a1.zip(b1).take(40_000_000-1).filter(|&(x,y)|{x==y}).count();
    println!("{}",r1);
    
    //part 2
    let mut statea = 783u64;
    let mut stateb = 325u64;
    
    let a2 = gen_iter(&mut statea, 16807).filter(|&x|x%4==0);
    let b2 = gen_iter(&mut stateb, 48271).filter(|&x|x%8==0);
    
    let r2 = a2.zip(b2).take(5_000_000).filter(|&(x,y)|{x==y}).count();
    println!("{}",r2);
}
