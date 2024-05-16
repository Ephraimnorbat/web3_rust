//takes in array -reference
//loops through items
//returns sum

fn main() {
    println!("Hello, world!");
    let var_1:[u32: 4] = [4, 2, 3, 5];
    let sum = calc_sum(param_1: &var_1);
    println!("Sum is {sum}");
}


fn calc_sum(param_1: &[u32]) -> u32{
    let mut param_2: i32 = 0;
    for elements: &u32 in param_1 {
        param_2 = elements + param_2;

    }
    param_2;
}