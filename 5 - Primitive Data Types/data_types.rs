fn plus_one(a: i32) -> i32 {
    a + 1
}

fn main(){
    //Boolean - True or False
    let x = true;
    let y: bool = false; //always lower case
    println!("{}, {}", x, y);

    //Character - Single Unicode scalar value (not single byte, but four bytes (unicode support))
    let x = 'x'; //only single quotes
    let y: char = '😎';
    println!("{}, {}", x, y);

    //Integers (i8, i16, i32, i64, i128)
    //Min and Max values of each: -(2^(n - 1)) to (2^(n - 1) - 1)
    //Or you can just simply type i8::min_value() or i8::max_value()
    let x = 10; //Default interger is i32
    let y: i8 = -128;
    println!("{}, {}", x, y);

    //Unsigned Integers (u8, u16, u32, u64, u128)
    //Min and Max values of each: 0 to (2^(n) - 1)
    //Or you can just simply type u8::min_value() or u8::max_value()
    let x: u8 = 255;
    println!("{}", x);

    //Pointer sized signed and unsigned integer types (isize, usize)
    //Bit size depends of your platform bit size (32-bits or 64-bits)

    //Floats (f32, f64)
    //f32 is the same as single precision float
    //f64 is the same as double precision float
    let x = 1.5; //Default float is i64
    let y: f64 = 2.0;
    println!("{}, {}", x, y);

    //Arrays - Are immutable by default, element count cannot be changed even with "mut"
    let a = [1, 2, 3];
    println!("a = {:?}", a);
    let a: [i32; 3] = [1, 2, 3]; //[Type, number of elements]
    println!("a = {:?}", a);

    let b: [i32; 0] = []; //Empty array
    println!("b = {:?}", b);

    let mut c: [i32; 3] = [1, 2, 3];
    println!("c = {:?}", c);
    c[0] = 2;
    c[1] = 4;
    c[2] = 6;
    println!("c = {:?}", c);

    let d = [0; 5];
    println!("d = {:?}", d);

    let e = ["x"; 5];
    println!("e = {:?}", e);

    //Tuples - Are immutable by default, element count cannot be changed even with "mut"
    //If changing some value, the new value must be same data type as previous one
    let a = (1, 1.5, true, 'a');
    println!("a = {:?}", a);
    let a: (i32, f64, bool, char) = (1, 1.5, true, 'a');
    println!("a = {:?}", a);

    let mut b = (1, 1.5);
    println!("b = {:?}", b);
    b.0 = 2;
    b.1 = 3.0;
    println!("b = {:?}", b);

    let (c, d) = b;
    println!("c = {}, d = {}", c, d);

    let (e, _, _, f) = a;
    println!("e = {}, f = {}", e, f);

    let g = (0,); //Single-element tuple
    println!("g = {:?}", g);

    let h = (b, (2, 4), 5); //Tuple inside tuples
    println!("h = {:?}", h);

    //Slice
    let a: [i32; 4] = [1, 2, 3, 4]; //Array
    println!("a = {:?}", a);

    let b: &[i32] = &a; //Slicing whole array
    println!("b = {:?}", b);

    let c = &a[0..4]; //From 0th position to 4th (excluding)
    println!("c = {:?}", c);

    let d = &a[..]; //Slicing whole array
    println!("d = {:?}", d);

    let e = &a[1..3];
    println!("e = {:?}", e);

    let f = &a[1..];
    println!("f = {:?}", f);

    let g = &a[..3];
    println!("g = {:?}", g);

    //Strings (str) - Unsized UTF-8 sequence of unicode string slices
    let a = "Hello, World!"; // a: &'static str, use String when you need ownership
    let b: &str = "こんにちは, 世界!"; //&str is used to borrow and assign the whole array in variable binding
    println!("{} = {}", a, b);              //use &str when you need to borrow a string
    
    //Generating strings in other ways
    let s: &str = "Hello"; //&str
    println!("{}", s);
    let s = s.to_string(); //String
    println!("{}", s);
    let s = String::from(s); //String
    println!("{}", s);
    let s = s.as_str(); //&str
    println!("{}", s);

    //Function
    let pointer_one: fn(i32) -> i32 = plus_one;
    let x = pointer_one(5);
    println!("{}", x);


}