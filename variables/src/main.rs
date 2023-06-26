fn main() {
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is : {x}");

    const THREE_HOURS_IN_SECONDS: i32 = 60 * 60 *3 ;

    println!("our constant is : {THREE_HOURS_IN_SECONDS}");
    
    let y = 1;
    let y = y +5;
    {
        let y = y * 2;
        println!("inner scope y: {y}");
    }

    println!("Outerscope y: {y}");

    let y = "six";

    println!("shadowed y to string: {y}");

    let spaces = "   ";
    let spaces = spaces.len();

    println!("number of spaces: {spaces}");

}
