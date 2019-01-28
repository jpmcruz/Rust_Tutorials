use std::io;

fn main() {
    // println!("Enter temperature in Celsius and I would convert to Fahrenheit");
    // loop {
    // let mut input_c = String::new();
    // io::stdin().read_line(&mut input_c)
    //     .expect("Failed to read line");
    //
    //     let input_c: u32 = match input_c.trim().parse(){
    //             Ok(num) => num,
    //             Err(_) => continue,
    // };
    //
    //
    // let input_f = (input_c * (9/5)) + 32;
    //
    // println!("Fahrenheit value is {}", input_f);
    // }

    println!("Enter number for Fibonnaci");
    loop {
    let mut input_fib = String::new();
    io::stdin().read_line(&mut input_fib)
        .expect("Failed to read line");

        let input_fib: u128 = match input_fib.trim().parse(){
                Ok(num) => num,
                Err(_) => continue,
    };

    let fib = fib(input_fib);
    //let input_f = (input_c * (9/5)) + 32;

    println!("The value of x is: {}", fib);
    }


}

fn fib(x: u128) -> u128 {
    if (x <= 1) {x}
    else {fib(x-1) + fib(x-2)}
}



// println!("Guess the number!");
//
//     let secret_number = rand::thread_rng().gen_range(1, 101);
//
// //    println!("The secret number is: {}", secret_number);
//
//     loop {
//         println!("Please input your guess.");
//
//         let mut guess = String::new();
//
//         io::stdin().read_line(&mut guess)
//             .expect("Failed to read line");
//
//         // let guess: u32 = guess.trim().parse()
//         //     .expect("Please type a number!");
//
//         let guess: u32 = match guess.trim().parse(){
//             Ok(num) => num,
//             Err(_) => continue,
//         };
//
//         println!("You guessed: {}", guess);
//
//         match guess.cmp(&secret_number) {
//             Ordering::Less => println!("Too small!"),
//             Ordering::Greater => println!("Too big!"),
//             Ordering::Equal => {println!("You win!"); break;}
//         }
//     }
//
// (0°C × 9/5) + 32 = 32°F
