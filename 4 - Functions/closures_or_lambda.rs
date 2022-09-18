// Closure are also known as Lambda/Anonymous Functions
// Assign data types to arguments and/or returns are optional here

fn main(){
    //Example 1: With optional type declarations of input and return types
    let w = 2;
    let square_one = |i: i32| -> i32{ //Use "| |" to define arguments and "{ }" to create function body
        i * i
    };

    println!("{}", square_one(w));

    //Example 2: Without optional type declarations of input and return types
    let x = 4;
    let square_two = |i| i * i; //"{ }" are optional if the closure is single-lined

    println!("{}", square_two(x));

    //Example 3: With optional type declarations | Creating and calling together
    let y = 6;
    let y_square = |i: i32| -> i32 {i * i}(y); //"{ }" are not optional if you're creating and calling together

    println!("{}", y_square);

    //Example 4: Without optional type declarations | Creating and calling together
    let z = 8;
    let z_square = |i| -> i32 {i * i}(z); //Here we will need to put return type

    println!("{}", z_square);
}

