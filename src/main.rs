fn main() {
    println!("Solving using Binet's approximation:");
    let b: [i32; 19] = [1,2,3,4,5,6,7,8,9,10,20,30,40,50,60,70,80,90,100];
    for number in b.iter() {
        println!("Fibonacci af {:?} er {:?}",number, binet(number.to_owned()).round());
    }
    println!("Solving correctly:");
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

fn binet(n:i32) -> f64 {
    ((5.0_f64.sqrt() + 1.0_f64) / 2.0_f64).powi(n) / 5.0_f64.sqrt()
}
