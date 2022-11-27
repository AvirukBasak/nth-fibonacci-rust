use std::io;
use std::io::Write;
use std::process;

fn fibo_helper(n: u32, a: u32, b: u32) -> u32 {
    if n == 1 {
        a
    } else if n == 2 {
        b
    } else {
        fibo_helper(n -1, b, a + b)
    }
}

fn nth_fibnum(n: u32) -> u32 {
    fibo_helper(n, 1, 1)
}

fn main() {
    loop {
        print!("enter n or 0 to exit: ");
        io::stdout().flush().unwrap();
        let mut inp = String::new();
        io::stdin().read_line(&mut inp).unwrap();
        let inp: u32 = match inp.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("error: expected a +ve number");
                continue;
            }
        };
        if inp < 1 {
            process::exit(0);
        }
        /* for single variables, one can pass "{<varname>}" to format!()
         * that won't work for function calls tho i.e. "{<fn_name>()}"
         * named parameters: https://doc.rust-lang.org/std/fmt/#named-parameters
         */
        println!("{inp}th Fibonacci number = {}", nth_fibnum(inp));
    }
}
