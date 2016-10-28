fn main() {
    let a: [u64; 19] = [1,2,3,4,5,6,7,8,9,10,20,30,40,50,60,70,80,90,100];

    for number in a.iter() {
        println!("Fibonacci af {:?} er {:?}",number, fib(number.to_owned()));
    }
}

fn fib(n:u64) -> u64 {
    match n {
    0 | 1   =>  n,
    _       =>  fib(n-1)+fib(n-2)
    }
}
