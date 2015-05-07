use std::env;

extern crate threatbutt;

fn main() {
    let args: Vec<_> = env::args().collect();
    let res = threatbutt::identify_sample(&args[1]);
    println!("{:?}", res);
}
