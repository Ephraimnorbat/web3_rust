fn main() {
    println!("Hello, world!");
    let mut value:String = String::from("What is this");
    let mut called_function:String = string_function(& mut value);
    println!("{}",called_function);

}


fn immut_string(value:& mut String) ->& mut String{
    value.push str(string: "this is a string");
    value
    
}