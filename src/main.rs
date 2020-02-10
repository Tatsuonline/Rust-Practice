/* 
This is just some recommended practice from chapter three of "the book".

This simple program is expected to do the following:

  0) Convert temperatures between Fahrenheit and Celsius.
  1) Generate the nth Fibonacci number.
  2) Print the lyrics to the Christmas carol “The Twelve Days of Christmas,” taking advantage of the repetition in the song.

*/

use std::io;
const GOLDEN_RATIO: f64 = 1.618034;
    
fn main() {

    println!("\nWhat would you like to do?\n\n0) Convert Fahrenheit to Celsius.\n1) Convert Celsius to Fahrenheit.\n2) Generate the nth Fibonacci number.\n3) Print lyrics of \"The Twelve Days of Christmas\".\n\nPlease input a number indicating your choice.");

    let mut choice = String::new();
    let mut input_value = String::new();
    
    io::stdin().read_line(&mut choice)
	.expect("Failed to read line"); // Only triggered with an operating system error.

    let choice: u32 = match choice.trim().parse() { // Shadowing the guess variable and converting it to an unsigned 32 bit integer.
	Ok(num) => num,
	Err(_) => 25, // Not sure how to handle this properly with a type match.
    };

    if choice == 0 {
	println!("\nEnter the number you wish to covert from Fahrenheit to Celsius:");
	io::stdin().read_line(&mut input_value)
	    .expect("Failed to read line"); // Only triggered with an operating system error.

	let input_value: f64 = match input_value.trim().parse() {
	    Ok(num) => num,
	    Err(_) => { // Not sure how to handle this properly with a type match.
		panic!("Enter a proper number, you dolt!\n")
	    },
	};	    

	let result = temperature_conversion_f_to_c(input_value);

	println!("{} degrees Fahrenheit is {:.2} degrees Celsius.\n", input_value, result)

    } else if choice == 1 {
	println!("\nEnter the number you wish to covert from Celsius to Fahrenheit:");
	io::stdin().read_line(&mut input_value)
	    .expect("Failed to read line"); // Only triggered with an operating system error.

	let input_value: f64 = match input_value.trim().parse() {
	    Ok(num) => num,
	    Err(_) => { // Not sure how to handle this properly with a type match.
		panic!("Enter a proper number, you dolt!\n")
	    },
	};	    

	let result = temperature_conversion_c_to_f(input_value);

	println!("{} degrees Celsius is {:.2} degrees Fahrenheit.\n", input_value, result)

    } else if choice == 2 {
	println!("\nEnter the nth Fibonacci number you want:");
	io::stdin().read_line(&mut input_value)
	    .expect("Failed to read line"); // Only triggered with an operating system error.

	let input_value: f64 = match input_value.trim().parse() {
	    Ok(num) => num,
	    Err(_) => { // Not sure how to handle this properly with a type match.
		panic!("Enter a proper number, you dolt!\n")
	    },
	};	    

	let result = fibonacci(input_value);

	println!("The {}th Fibonacci number is {:.0}.\n", input_value, result)

    } else if choice == 3 {

	lyrics();
	
    } else {
	println!("You sir, are an idiot.\n");
    }
	    
    
}

fn temperature_conversion_f_to_c (input_value: f64) -> f64 {

    (input_value - 32.0) * (5.0/9.0)
    
}

fn temperature_conversion_c_to_f (input_value: f64) -> f64 {

    (input_value * (9.0/5.0)) + 32.0
    
}

fn fibonacci (input_value: f64) -> f64 {

    let five = 5.0_f64;
    
    ((GOLDEN_RATIO.powf(input_value)) - ((1.0-GOLDEN_RATIO).powf(input_value))) / (five.sqrt())

}

fn lyrics () {

    let gifts = ["Partridge in a Pear Tree", "Turtle Doves", "French Hens", "Calling Birds", "Golden Rings", "Geese a Laying", "Swans a Swimming", "Maids a Milking", "Ladies Dancing", "Lords a Leaping", "Pipers Piping", "Drummers Drumming"];

    let mut postfix = "th";
    
    for number in 0..gifts.len() {

	if number == 0 {
	    postfix = "st";
	} else if number == 1 {
	    postfix = "nd";
	} else if number == 2 {
	    postfix = "rd";
	} else {
	    postfix = "th";
	}

	println!("\nOn the {}{} day of Christmas, my true love sent to me:", number + 1, postfix);

	for item in (0..number+1).rev() {
	    println!("{} {}", item + 1, gifts[item]);
	}

    }

}
