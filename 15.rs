#![feature(conservative_impl_trait)]

fn gen_iter<'a>(init:u64, mult : u64) -> impl Iterator<Item=u64> + 'a {
    (0..).scan(init,move |state,_|{
        *state = (*state*mult)%2147483647;
        Some(*state&0x0ffff)
    })
}

fn main(){
    let (s1,s2,m1,m2) = (783u64,325u64,16807,48271);
    let (it1,it2) = (40_000_000,5_000_000);
    
    //part 1
    let a1 = gen_iter(s1, m1);
    let b1 = gen_iter(s2, m2);

    let r1 = a1.zip(b1).take(it1-1).filter(|&(x,y)|{x==y}).count();
    println!("{}",r1);
    
    //part 2
    let a2 = gen_iter(s1, m1).filter(|&x|x%4==0);
    let b2 = gen_iter(s2, m2).filter(|&x|x%8==0);
    
    let r2 = a2.zip(b2).take(it2).filter(|&(x,y)|{x==y}).count();
    println!("{}",r2);
}
