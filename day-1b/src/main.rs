use std::fs;

fn main() {
    let file = fs::read_to_string("input.txt")
        .expect("Something went wrong reading the file");
    
    let mut increases = 0;      // counter for amount of increases
    let mut prev = i32::MAX;    // starting value
    let mut rolling_window = vec![0, 0, 0];
    let mut i = 0;

    for line in file.lines() {
        // parse string to integer
        let value = parse_int(line);

        // shift the values in the rolling window, there is likely to be a native method to do this
        rolling_window = shift_rolling_window(rolling_window, value);
        let sum: i32 = rolling_window.iter().sum();

        if sum > prev {
            increases += 1;
        }

        // skip the first 2 times bcs of the rolling window
        i += 1;
        if i > 2 {
            prev = sum;
        }
    }

    println!("Submarine depth increased {} times", increases);
}

fn parse_int(input: &str) -> i32 {
    return input.parse::<i32>().unwrap();
}

fn shift_rolling_window(mut window: Vec<i32>, n: i32) -> Vec<i32> {
    window[2] = window[1];
    window[1] = window[0];
    window[0] = n;
    window
}