fn main() {
    let a = 30;
    println!("Fibonacci af {} er {}",a,fib(a));
}

fn fib(n:i64) -> i64 {
    match n {
    0 | 1   =>  n,
    _       =>  fib(n-1)+fib(n-2)
    }
}
