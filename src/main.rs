
// Formulas 
// C to F
// C = (F-32) * (5/9)

// F to C
// F = C * (9/5) + 32

use std::env;
use std::process;

fn main() {

    let in_arges: Vec<String> = env::args().collect();
    
    // Uncomment for debug purposes
    //for i in 0..in_arges.len(){
    //    println!("Arges[{}]: {}", i, in_arges[i]);
    //}

    if in_arges.len() < 2 && in_arges[1] != "--help"{
        println!("Not enough arguments, run c2f --help");
        std::process::exit(0);
    } else if in_arges.len() >= 5 {
        println!("Too Many arguments, run c2f --help");
        std::process::exit(0); 
    }

    if in_arges[1] == "--help"{
        help();
        std::process::exit(0);
    }else if in_arges[1] == "--c2f" {
        let input = in_arges[2].parse::<f32>().unwrap();
        let results = c2f(input);
        println!("C2F: {}", results);
    }else if in_arges[1] == "--f2c" {
        let input = in_arges[2].parse::<f32>().unwrap();
        let results = f2c(input);
        println!("F2C: {}", results);
    }else if in_arges[1] == "--f2ct"{
        let arg1 = in_arges[2].parse::<i32>().unwrap();
        let arg2 = in_arges[3].parse::<i32>().unwrap();
        range_table("f".to_string(), arg1, arg2);
    }else if in_arges[1] == "--c2ft"{
        let arg1 = in_arges[2].parse::<i32>().unwrap();
        let arg2 = in_arges[3].parse::<i32>().unwrap();
        range_table("c".to_string(), arg1, arg2);
    } else if in_arges[1] == "--f2b"{
        freezing_to_boiling();
    }
}

// Conavert Celsius to Fahrenheit 
fn c2f(degree: f32) -> f32{
    let resualts = degree * (1.8) + 32.0;
    resualts
}

// Converts Fahrenheit  to Celsius
fn f2c(degree: f32) -> f32{
    let resualts = (degree - 32.0) * (0.555555556);
    resualts
}

fn help(){
    println!("--help  Prints this menu");
    println!("--c2f   <number>, Converts a Celsius degree to Fahrenheit");
    println!("--f2c   <number>, Converts a Fahrenheit degree to Celsius");
    println!("--f2ct  <staring_number> <ending_number>, Prints a table converting between a range of Fahrenheit to Celsius");
    println!("--c2ft  <staring_number> <ending_number>, Prints a table converting between a range of Celsius to Fahrenheit");
    println!("--f2b   Prints the table from freezing ot boiling, C to F");
    println!("--ver   Prints the program version");
}

// tempeture c or f as options
fn range_table(tempeture_type: String, staring_range: i32, ending_range: i32){

    println!("Staring temp: {},| Ending Temp {}", &staring_range, ending_range);
    println!("-------------------------------------");

    if tempeture_type == "c"{

        for i in staring_range..ending_range + 1 {
            let temp_i = i as f32;
            let resualt = c2f(temp_i);
            println!("F: {}  |  C: {}", i , resualt);
        }
    }else if tempeture_type == "f"{
        for i in staring_range..ending_range + 1{
            let temp_i = i as f32;
            let resualt = f2c(temp_i);
            println!("C: {}  |  F: {}", i , resualt);
        }
    }else{
        println!("Not a valid input");
    }
}

fn freezing_to_boiling(){
    range_table("c".to_string(), 0, 100);
}