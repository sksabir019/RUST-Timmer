//Countdown Rust
//input - 10
//10 9 8 7 6 5 ...1
use std::io;
use std::thread::sleep;
use std::time::Duration;


fn main() {
    let mut input = String::new();
    println!("Please enter the timer:");
    io::stdin().read_line(&mut input).expect("Invalid Input");


    let timer: u16 = input.trim().parse().expect("Invalid number");
    start_timer(timer);
}
//1 2 3 4 5 6 .. 10 -> (1..=10)
//10 9 8 7 6 5 ... 1 -> (1..=10).rev()
fn start_timer(timer: u16) {
    for i in (1..=timer).rev() {
        println!("Timer countdown..{}", i);
        sleep(Duration::from_secs(1)); //to have a delay of 1 second
    }
}


fn main2() {
    loop {
        let mut input = String::new();
        println!("Please enter the timer:");
        io::stdin().read_line(&mut input).expect("Invalid Input");


        //let timer: u16 = input.trim().parse().expect("Invalid number");
        let timer: u16 = match input.trim().parse() {
            Ok(timer) => timer,
            Err(_) => {
                println!("Invalid Number");
                continue;
            }
        };
        start_timer2(timer);
        break;
    }
}
//1 2 3 4 5 6 .. 10 -> (1..=10)
//10 9 8 7 6 5 ... 1 -> (1..=10).rev()
fn start_timer2(timer: u16) {
    for i in (1..=timer).rev() {
        println!("Timer countdown..{}", i);
        sleep(Duration::from_secs(1)); //to have a delay of 1 second
    }
}