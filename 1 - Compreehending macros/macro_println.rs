fn main(){
    //Multiple ways to print "Hello, World!"
    println!("1 - {}, {}!", "Hello", "World");
    println!("2 - {0}, {1}!", "Hello", "World");
    println!("3 - {greeting}, {name}!", greeting = "Hello", name = "World");

    let (greeting, name) = ("Hello", "World"); //Declaring variables
    println!("4 - {greeting}, {name}!");

    //Formatting data
    println!("{:?}", [1, 2, 3]);
    println!("{:#?}", [1, 2, 3]); //With new line

    //Format! macro stores a formatted string
    let formatted_string = format!("{}, {}!", "Hello", "World");
    println!("5 - {}", formatted_string);

    //There's also print! macro, it doesn't put a new line (obviously)
    print!("6 - Hello, World!"); //Doesn't has new line ('\n') at the end
    println!("7 - Hello, World!"); //Does has new line ('\n') at the end

    //Macro is different from functions because macros are "code that write code" (metaprogramming)
}