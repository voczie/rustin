// We declare Named Functions with "fn" keyword
// You must declare the arguments data types
// Functions return an empty tuple "()" as default. To specify return type, use "->"


//Just prints "Hello, World!" and return an empty tuple
fn hello_world(){
    println!("Hello, World!");
}

//Arguments "a" and "b" needs data type assigned
fn print_sum(a: i8, b: i8){
    println!("sum is: {}", a + b);
}

//Defining the return type with "->"
fn plus_one(a: i32) -> i32{
    a + 1 //This line is equal to "return a + 1;" and it doesn't need ";"
}

fn plus_two(a: i32) -> i32{
    return a + 2; //We only use "return" keyword in conditional/early returns
                    //Using "return" as last expression in a function is considered bad practice
}

//Function pointers
fn main(){
    //Without type declarations
    let pointer_one = plus_one;
    let x = pointer_one(5);
    println!("{}", x);
    
    //With type declarations
    let pointer_one: fn(i32) -> i32 = plus_one;
    let x = pointer_one(5);
    println!("{}", x);
}