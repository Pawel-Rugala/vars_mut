fn main() {
    // # Immutability
    // let x = 5;
    // println!("The value of x is: {x}");
    // x = 6;  // error: cannot assign twice to immutable variable `x`
    // println!("The value of x is: {x}")

    // # Mutability
    // let mut x = 5;
    // println!("The value of x is: {x}");
    // x = 6;  // this is ok because x is mutable
    // println!("The value of x is: {x}")

    // # Constants
    // const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

    // # Shadowing
    let x = 5;
    let x = x + 1;
    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}")
    }
    println!("The value of x is: {x}")

}
