fn main(){
    let result = square_of_a_number(4);
    println!("The square of the number is :{result}");
}
fn square_of_a_number(x: i32) -> i32{
    let square = x*x ;
    square
}