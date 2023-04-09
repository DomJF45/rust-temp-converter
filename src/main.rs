use std::io;

fn main() {
    
    println!("Convert f to c!");

    let temp: i32; 
    let unit: char;

    loop {

        println!("Enter the Temp!");

        let mut user_temp = String::new();

        io::stdin()
            .read_line(&mut user_temp)
            .expect("Failed to read line");

        let user_temp: i32 = match user_temp.trim().parse() {
            Ok(num) => num,
            Err(_) => continue
        };

        temp = user_temp;
        break;
    }

    loop {

        println!("Enter the unit!");

        let mut user_unit = String::new();

        io::stdin()
            .read_line(&mut user_unit)
            .expect("Failed to read line");

        let user_unit: char = match user_unit.trim().parse() {
            Ok(char) => char,
            Err(_) => continue
        };

        unit = user_unit;
        break;
    }

    if unit == 'c' || unit == 'C' {
        let conversion: i32 = convert_c_to_f(temp);
        println!("Temp in F: {conversion}");
    } else if unit == 'f' || unit == 'F' {
        let conversion: i32 = convert_f_to_c(temp);
        println!("Temp in C: {conversion}");
    } else {
        println!("Error");
    }

        
}

fn convert_c_to_f(temp: i32) -> i32 {
    return (temp * 9/5) + 32;
}

fn convert_f_to_c(temp: i32) -> i32 {
    return (temp -32) * 5/9;
}
