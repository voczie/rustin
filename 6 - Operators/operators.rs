fn main(){
    // Arithmetic Operators

    let a = 5;
    let b = a + 1; //Add
    let c = a - 1; //Subtract
    let d = a * 2; //Multiply
    let e = a / 2; //Divide (returns Interger)
    let f = a % 2; //Rest of Division

    println!("a = {}, b = {}, c = {}, d = {}, e = {}, f = {}", a, b, c, d, e, f);

    // Comparison Operators

    let a = 1;
    let b = 2;

    let c = a == b; //It's equal to
    let d = a != b; //It's different than
    let e = a < b; //It's smaller than
    let f = a > b; //It's bigger than
    let g = a <= a; //It's smaller than or equal to
    let h = a >= a; //It's bigger than or equal to

    let i = true > false; //Supposedly, true = 1 and false = 0
    let j = 'a' > 'A'; //'a' = 97 and 'A' = 65 (ASCII table values)

    println!("a = {}, b = {}, c = {}, d = {}, e = {}, f = {}, g = {}, h = {}, i = {}, j = {}", a, b, c, d, e, f, g, h, i, j);

    // Logical Operators

    let a = true;
    let b = false;

    let c = !a; //Not
    let d = a && b; //And
    let e = a || b; //Or

    //Not operator (!) can be used in intergers and it returns the two's complement of the value

    let f = !-2;
    let g = !-1;
    let h = !0;
    let i = !1;

    println!("a = {}, b = {}, c = {}, d = {}, e = {}, f = {}, g = {}, h = {}, i = {}", a, b, c, d, e, f, g, h, i);

    // Bitwise Operators

    let a = 1;
    let b = 2;

    let c = a & b; //And, but 01 && 10 = 00
    let d = a | b; //Or, but 01 || 10 = 11
    let e = a ^ b; //Different than, 01 != 10 = 11
    let f = a << b; //Add (b = 2) number of zeros at the end of (a), 01 + 00 = 100
    let g = a >> b; //Remove (b = 2) number of bits from the end of (a), 01 = 0

    println!("a = {}, b = {}, c = {}, d = {}, e = {}, f = {}, g = {}", a, b, c, d, e, f, g);

    // Assignment and Compound Assignment Operators

    let mut a = 2;
    println!("a = {}", a);

    a += 5; //Add 5 and assign (Increment)
    println!("a = {}", a);
    a -= 2; //Subtract 2 and assign (Decrement)
    println!("a = {}", a);
    a *= 5; //Multiply 5 and assign
    println!("a = {}", a);
    a /= 2; //Divide by 2 and assign
    println!("a = {}", a);
    a %= 5; //Rest of division by 5 and assign
    println!("a = {}", a);

    a &= 2; //10 && 10 = 10 = 2
    println!("a = {}", a);
    a |= 5; //010 || 101 = 111 = 7
    println!("a = {}", a);
    a ^= 2; //111 != 010 = 101 = 5
    println!("a = {}", a);
    a <<= 1; //'101'+'0' = 1010 = 10
    println!("a = {}", a);
    a >>= 2; //1010 = 10 = 2
    println!("a = {}", a);

    // Type Casting Operator
    let a = 15;
    let b = (a as f64) / 2.0;

    println!("a = {}, b = {}", a, b);

    // Borrowing (& and &mut) and Dereference (*) Operators - Not gonna do it right now
}