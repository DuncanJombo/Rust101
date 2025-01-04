fn  main(){
    {
    //Declaring a tuple 
    let my_tuple:(u32,u64,u128) = (120,120120,120120120);
    let (num1,num2,num3) = my_tuple;
    print!("num1: {},num2: {},num3: {} \n",num1,num2,num3);
    }

    {
    //Declaring an array
    let my_array = [1,2,3,4,5,6,7,8,9,10];
    print!("{:?}",my_array)
    }
}

/*
Prefixing variables with an underscore tells the compiler 
that those variables should be ignored by the compiler during execution
*/