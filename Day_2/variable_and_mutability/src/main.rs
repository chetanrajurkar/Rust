fn main() {
    let mut x = 6;
    println!("the number is {} ", x);
    x = 7;
    println!("the number is {x} now");

    // Const declaration
    const THREE_HOURS_IN_SECONDS: u16 = 60 * 60 * 3;
    println!("{THREE_HOURS_IN_SECONDS}");

    // shadowing
    let x = 5;
    println!("{x}");
    let x = x + 11;
    {
        let x = x * 3;
        println!("{x}");
    }
    println!("{x}");
}
