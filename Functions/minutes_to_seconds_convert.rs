fn main(){
    let result = convert_seconds_to_minutes(300);
    println!("The result is: {result}");
}
fn convert_seconds_to_minutes(x: i32) -> i32{
    let converted_seconds = x/60;
    converted_seconds
}