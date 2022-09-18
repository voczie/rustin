// It's possible to redeclare the same variable with a different data type and/or a different mutability setting
// This process is called Shadowing!

fn main(){
    let x: f64 = -20.48; //Declaring x as float with value "-20.48"
    let x: i64 = x.floor() as i64; //Shadowing a variable to make it "-21" and an interger
    println!("{}", x); //-21

    let s: &str = "hello"; //Declared as &str
    let s: String = s.to_uppercase(); //Shadowing a variable to make it "HELLO" and a string
    println!("{}", s); //HELLO
}