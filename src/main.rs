use std::io;
use std::process;

fn main() {
    println!("Welcome to degree convertor!");
    
    // Reading degree unit
    println!("What is the original degree unit?");
    let mut temp_type = String::new();
    io::stdin()
        .read_line(&mut temp_type)
        .expect("failed");
    let mut temp_type = temp_type.trim().to_uppercase();
    while temp_type != "F" && temp_type != "C" {
        println!("'{temp_type}' is not a valid unit, try again");
        temp_type.clear();
        io::stdin()
            .read_line(&mut temp_type)
            .expect("No way");
        // println!("OoooooH NO NOT Again {temp_type}");
        temp_type = temp_type.trim().to_uppercase();
        // println!("OH NO NOT Again {temp_type}");
    }
    
    // Reading degree value
    println!("And what is the original degree?");
    let mut degree = String::new();
    io::stdin()
        .read_line(&mut degree)
        .expect("failed again");
    let degree: f32 = match degree.to_lowercase().trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Not a number");
            500.00
        },
    };

    if degree == 500.00 {
        process::exit(1);
    }

    // Conversions

    let mut new_temp: f32 = 0.0;
    let mut new_degree_unit = "";

    if temp_type == "F" {
        new_temp = (degree as f32 - 32.0) * (5.0/9.0);
        new_degree_unit = "C";

        // println!("This is {new_temp} and {new_degree_unit}");
    }

    if temp_type == "C" {
        new_temp = (degree as f32 * (9.0/5.0)) + 32.0;
        new_degree_unit = "F";

        // println!("This is {new_temp} and {new_degree_unit}");
    }



    println!("{degree}˚{temp_type} is equal to about {new_temp}˚{new_degree_unit}");
}