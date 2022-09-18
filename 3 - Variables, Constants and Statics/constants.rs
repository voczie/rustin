// Constants need type annotation
// You can't change constants values after they're assigned
// They don't have fixed address in the memory
// We place constants at the top of the code, outside functions and after module imports/use declarations

const REAL_CONSTANT_DECLARATION: i32 = 12;

fn main(){
    const N: i32 = 5; //"const" is the keyword for constants
    println!("{}, {}", N, REAL_CONSTANT_DECLARATION);
}