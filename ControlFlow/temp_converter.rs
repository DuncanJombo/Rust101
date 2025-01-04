fn main(){
    convert(33.2);
}
fn convert(x: f32){
    let celcius = (x - 32.0) * 5.0/9.0 ;
    println!("The converted temperature from farenheit to celcius is: {celcius}C");
}

//Convert temperatures from Fahrenheit to Celsius