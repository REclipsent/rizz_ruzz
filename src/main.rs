use std::{thread, time::Duration};

fn main() {
    println!("Starting Soon...");
    thread::sleep(Duration::from_secs(1));
    for n in 1..101 {
        let mut output: String = "".to_owned();
        if n % 3 == 0 {
            let fizz = "Fizz".to_owned();
            output.push_str(&fizz);
        }

        if n % 5 == 0 {
            let buzz = "Buzz".to_owned();
            output.push_str(&buzz)
        }


        if output.is_empty() {
            println!("{}", n)
        }
        else {
            println!("{}", output)
        }
    }

    println!("Complete");
    
    thread::sleep(Duration::from_secs(10));
}
