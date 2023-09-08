use std::io;

fn main() {
    // if exercise
    discount(31);
    
    // loop exercise
    loop {
        println!("What is the secret word?");
        let mut word = String::new();
        io::stdin().read_line(&mut word).expect("Failed to read line");
        
        if word.trim() == "rust" {
            break;
        }
    }
    
    println!("You know the secred word. Please proceed!");
    
    // while exercise
    let mut word = String::new();
    while word.trim() != "rust" {
        println!("What is the secret word?");
        word = String::new();
        io::stdin().read_line(&mut word).expect("Failed to read line");
    }
    
    println!("You know the secred word. Please proceed!");
    
    // for exercise
    for i in 1..11 {
        println!("Now serving number {}", i);
    }
}


fn discount(day_of_month: u8) {
    let amount = if day_of_month % 2 == 0 {
        50
    } else {
        10
    };
    
    println!("Your discount is {}%!", amount);
}
