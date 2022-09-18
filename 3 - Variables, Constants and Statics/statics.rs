// Statics need type annotation and are also known as "global variables"
// It has a fixed address in the memory
// We place statics at the top of the code, outside functions and after module imports/use declarations

static REAL_STATIC_DECLARATION: i32 = 102;

fn main(){
    static N: i32 = 5; //"static" is the keyword for statics
    println!("{}, {}", N, REAL_STATIC_DECLARATION);
}