// Statics need type annotation and are also known as "global variables"
// It has a fixed address in the memory

fn main(){
    static N: i32 = 5; //"static" is the keyword for statics
    println!("{}", N);
}