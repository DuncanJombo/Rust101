fn main(){
    let fibo = fibonacci(3) ;
    println!("{fibo}");
}
fn fibonacci(x: i32) -> i32{
    if x == 0{
        return x;
    }else if x == 1{
        return x;
    }else{
        return fibonacci(x-2) + fibonacci(x-1) ;
    }
}