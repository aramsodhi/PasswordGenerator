// import libraries for getting input from user and generating random alphanumeric characters
use std::io::{self, Write};
use rand::distributions::Alphanumeric;
use rand::Rng;


fn main() {
    // print a welcome message to the password generator app and a new line under
    println!("");
    println!("");
    println!("--------- Password Generator ---------");
    println!("");

    // create a mutable unsigned 8 bit integer to store the password length
    let _password_length: u8;

    // repeat asking the user for a password length until it is valid
    loop {
        println!("Enter a password length between 0-255: ");
        io::stdout().flush().unwrap();

        // create a mutable string to hold the user's input
        let mut user_input = String::new();

        // read the user's input and store in user_input string
        io::stdin()
            .read_line(&mut user_input)
            .expect("Could not read user input");

        // attempt to convert the user's input into an unsigned 8 bit integer
        match user_input.trim().parse::<u8>() {
            Ok(number) => {
                // check that the password length is within the valid range for a u8
                if number <= u8::MAX {
                    // if the number is within a valid range, set password length to the number
                    _password_length = number;
                    break;
                } else {
                    // if the number is not within the range, tell the user
                    println!("Input is not within valid range");
                }
            }

            // if there is an error, tell the user to try again
            Err(_) => {
                println!("Invalid input. Try again.");
            }
        }
    }

    // init random generator
    let mut rng = rand::thread_rng();

    // generate a string of randomly selected ascii characters of _password_length length
    let password: String = (0.._password_length)
        .map(|_| rng.sample(Alphanumeric) as char)
        .collect();

    println!("Password: {password}");

}
