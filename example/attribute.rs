use std::env;

extern crate threatbutt;

fn main() {
    let args: Vec<_> = env::args().collect();
    let res = threatbutt::attribute_ip(&args[1]);
    println!("{:?}", res);
}
