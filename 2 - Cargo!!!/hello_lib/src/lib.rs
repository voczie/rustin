pub fn hello() -> String {
    ("Hello, World!").to_string()
}

#[cfg(test)] // Configure of the tests
mod tests {
    use super::hello;

    #[test]
    fn test_hello() { //Function that tests hello() function
        assert_eq!(hello(), "Hello, World!"); //Checks if hello() = "Hello, World!"
    }
}

//tests were ok!