pub fn run() {
    // print to console
    println!("Hello from print function");

    // basic formatting
    println!("Number: {}", 1);
    println!("{} is from {}", "Elie", "France");

    // positional argument
    println!("{} is from {} and {0} likes to {}", "Elie", "France", "eat");

    // named argument
    println!("{name} likes to {activity}", name="Elie", activity="eat");

    // traits
    println!("Binary: {:b}, Hex: {:x}, Octal: {:o} ", 10, 10, 10);

    // debug traits
    println!("{:?}", (12, true, "Hello", 10+10));

}