fn main(){
    // If - Else if - Else
    
    let age = 13;
    if age < 18{
        println!("Hello, child!");
    }

    ////
    let i = 17;
    if i % 2 == 0{
        println!("Even");
    }
    else{
        println!("Odd");
    }

    ////
    let age: u8 = 13;
    let is_bellow_eighteen = if age < 18 {true} else {false};

    println!("{}", is_bellow_eighteen);

    ////
    let team_size = 7;
    if team_size < 5{
        println!("Small");
    }
    else if team_size < 10{
        println!("Medium");
    }
    else{
        println!("Large");
    }

    ////
    let team_size = 7;
    let team_size_in_text;

    if team_size < 5{
        team_size_in_text = "Small";
    }
    else if team_size < 10{
        team_size_in_text = "Medium";
    }
    else{
        team_size_in_text = "Large";
    }

    println!("Current team size: {}", team_size_in_text);

    ////
    let team_size = 7;
    let team_size = 
    if team_size < 5{
        "Small"
    }
    else if team_size < 10{
        "Medium"
    }
    else{
        "Large"
    };

    println!("Current team size : {}", team_size); 
}