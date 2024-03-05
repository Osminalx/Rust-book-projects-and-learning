fn fibonacci(num:u32)->u32{
    if num <=1{
        return 1;
    }
    let mut fib_ant = 0;
    let mut fib_act = 1;
    for _ in 2..num{
        let fib_sig = fib_ant + fib_act;
        fib_ant = fib_act;
        fib_act = fib_sig;
    }
    return fib_act;
}

fn main() {
    let n:u32 = 4;
    let result = fibonacci(n);
    println!("resultado {result}")
}
