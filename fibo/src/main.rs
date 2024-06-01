use clap::Parser;

fn fibo_recursive(n: u32) -> u32{
    match n {
        0   => 0,
        1   => 1,
        _   => fibo_recursive(n - 1) + fibo_recursive(n -2)
    }
}

fn fibo_iterative(n: u32) -> Option<u32> {
    if n == 0 {
        return Some(0);
    } else if n == 1 {
        return Some(1);
    }

    let mut nbr1: u32 = 0;
    let mut nbr2: u32 = 1;
    let mut next;

    for _ in 2..=n {
        next = nbr1.checked_add(nbr2)?;
        nbr1 = nbr2;
        nbr2 = next;
    }

    Some(nbr2)
}

/// Simple program to get fibo number from CLI
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Number of times to run the loop
    #[arg(short, long)]
    count: u32,
}

fn main() {
    let args = Args::parse();

    for i in 0..=args.count {
        match fibo_iterative(i) {
            Some(result) => println!("fibo({i}) = {}", result),
            None => {
                println!("Reached max value of u32 at fibo({i})");
                break;
            }
        }
    }

}
