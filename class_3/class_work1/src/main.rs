fn main() {
    println!("Hello, world!");
    let var2 = [1.2, 2.3, 4.2, 5.6];
    let output1 = array_input(var2);
    println!("The total is {}",output1);
}
//takes in an array
//returns a param
fn array_input(par_1:&[f32]) -> f32{
    let mut index = 0;
    let mut length = par_1.len();
    let mut totals = 0.0;
    while index < length{
        // index += 1
        index = index + 1;
        totals = totals + par_1[index-1];
    }
    returns totals
}
//1. Create an external module eg fruits
      //a . file module
      //b. directory module
      //file module funcion takes in a slice(variables inside curly braces) and returns the second item in the slice
      // directory module -> println functions
      // call the file modules and directory functions from main rs
