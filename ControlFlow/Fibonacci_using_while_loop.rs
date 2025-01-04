fn main() {
    let fibo = fibonacci(3);
    println!("{}", fibo);
}

fn fibonacci(x: i32) -> i32 {
    let mut a = 0;
    let mut b = 1;
    let mut count = 0;

    while count < x {
        let temp = a + b;
        a = b;
        b = temp;
        count += 1;
    }

    a
}
