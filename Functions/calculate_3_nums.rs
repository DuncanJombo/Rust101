fn main(){
    let sum = calculate(12,91,10);
    println!("The sum is : {sum}");
}
fn calculate(num1:i32,num2:i32,num3:i32) -> i32{
    let sum = num1 + num2 + num3 ;
    sum
}