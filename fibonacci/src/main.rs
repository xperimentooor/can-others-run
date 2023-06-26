
fn fibonacci (n: u32) -> u32 {
    match n{
        0 => 0,
        1 => 1,
        _ => fibonacci(n-1) + fibonacci(n-2),
    }
}

fn main() {

    let n = 10;
    println!("The {}th Foibonacci number {}", n, fibonacci(10));
}
