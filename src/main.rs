use std::io;

fn main() {
    let number = 11;

    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else {
        println!("Number is not divisible by 4 or 3 ");
    }

    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 5 {
                break;
            }
            if count == 5 {
                break 'counting_up; // this will exit the outer loops
            }
            remaining -= 1;
        }

        count += 1;
    }

    println!("End count = {count}");

    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {result}");

    let mut launch = 5;

    while launch != 0 {
        println!("{launch}!");
        launch -= 1;
    }

    println!("LIFTOFF");

    let b = [15, 20 , 30 , 40 , 50];
    

    for element in b {

        println!("the value is: {element}");
    }

    for countdown in (1..4).rev() {
        println!("{countdown}");

    }
    
    println!("Enter temperature in Celcius:");

    let mut celcius = String::new();

    io::stdin().read_line(&mut celcius).expect("failed to read line");

    let celcius: f64 = celcius.trim().parse().expect("Please type a number!");

    let farenheit = celcius_farenheit(celcius);

    println!("{celcius} Celcius is equal to {farenheit} farenheit");
    

}

fn celcius_farenheit(celcius:f64) -> f64 {
    (celcius * 9.0 / 5.0) + 32.0
}



