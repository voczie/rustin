fn main(){
    let (s1, s2) = ("some", "thing"); //They are both "&str" type

    let s = String::from(s1) + s2; //s1 = String + s2 = &str

    println!("{}", s);

    let mut s = String::from(s1); //String
    s.push_str(s2); //+ &str

    println!("{}", s);

    let s = format!("{}{}", s1, s2); //&str/String + &str/String

    println!("{}", s);

    let s = [s1, s2].concat(); //&str or String array

    println!("{}", s);

}