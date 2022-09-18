// Rust is a *statically typed* language, that means that it checks data types at compile-time!
// It automatically checks the usage and sets the better type for the variable
// ":" is used to define the type

fn main(){
    //We use the "let" keyword to bind a name to a value or function
    let a; //Declaration without data type
    a = 5; //Variable assignment
    println!("{}", a);

    let b: i8; //Declaration with data type
    b = 5; //Variable assignment
    println!("{}", b);

    let t = true; //Declaration with assignment, but without data type
    let f: bool = false; //Declaration with assignment, but with data type
    println!("{}, {}", t, f);

    let (x, y) = (1, 2); //Declare "x" and "y" and assign 1 to "x" and 2 to "y"
    println!("{}, {}", x, y);

    let mut z = 5; //Declare and assign 5 to "z", but with "mut" you can reassign values
    println!("{}", z);
    z = 6; //z = 6
    println!("{}", z);

    let z = {x + y}; //z = 3
    println!("{}", z);

    let z ={
        let x = 1;
        let y = 2;

        x + y
    }; //z = 3 | That's the most beautiful thing I saw in my life
    println!("{}", z);
}