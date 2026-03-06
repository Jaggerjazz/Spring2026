
//TEMPERATURE CHANGE
const FREEZINF_POINT_F: f64 = 32.0;

fn farenheit_to_celsius(f: f64) -> f64 {
    (f - FREEZINF_POINT_F) * 5.0 / 9.0
}

fn is_even(n: i32) -> bool {
    n % 2 == 0
}
fn check_guess(guess: i32, secret: i32) -> i32 {
    if guess == secret {
        0
    } else if guess > secret {
        1
    } else {
        -1
    }
}

fn main() {
    println!("\n--- Farenheit to Celsius ---");
    //Declare a mutable variable with an intial temp
    let mut temp_f = 32.0;

    //Convert and print the first result
    let intial_c = farenheit_to_celsius(temp_f);
    println!("{:.2} °F is {:.2}°C", temp_f, intial_c);

    //Use a loop to convert and print the next 5 whole numbers
    let mut counter = 0;
    while counter < 5 {
        temp_f +=  1.0;
        let c = farenheit_to_celsius(temp_f);
        println!("{:.2} °F is {:.2}°C",temp_f, c);
        counter += 1;
    }

//NUMBER ANALYZER 
println!("\n--- Number Analyzer ---");

    //Array of 10 integers
    let numbers: [i32; 10] = [3, 10, 15, 7, 20, 12, 9, 30, 4, 11];

    //For loop for even/odd
    for &num in numbers.iter() {
     if num % 3 == 0 && num % 5 == 0 {
        println!("{}: FizzBuzz", num);
    }   else if num % 3 == 0 {
        println!("{}: Fizz", num);
    }   else if num % 5 == 0 {
        println!("{}: Buzz", num);
    }   else {
         let even_str = if is_even(num) { "even" } else { "odd" };
            println!("{}: {}", num, even_str);
    }
    
}
    //While loop to find the sum
    let mut sum = 0;
    let mut i = 0;
    while i < numbers.len() {
        sum += numbers[i];
        i += 1;
    }
    println!{"Sum of numbers: {}", sum};

    //Let loop to find the largest number
    let mut largest = numbers[0];
    for &num in numbers.iter(){
        if num > largest {
            largest = num;
        }
    }
    println!("Largest number: {}", largest);

//GUESSING GAME
println!("\n--- Guessing Game ---");

    let secret_number = 42;
    let mut guess_count = 0;

    //Simulating a sequence of guesses
    let simulated_guesses = [10, 50, 42];
    let mut current_index = 0;

    loop {
        //Simulating user input by grabbing from array
        let current_guess = simulated_guesses[current_index];
        guess_count += 1;

        let result = check_guess(current_guess, secret_number);

        if result == 0 {
            println!("Guess {}: Correct! ", current_guess);
            break;
        } else if result == 1 {
            println!("Guess {}: Too high", current_guess);
        } else {
            println!("Guess {}: Too low", current_guess);
        }

        current_index += 1;

    }
    println!("It took you {} guesses to find the secret number. ", guess_count);
}