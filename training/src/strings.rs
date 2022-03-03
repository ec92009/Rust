pub fn run() {
    let mut hello = String::from("Hello");
    println!("{:?}", (hello.len()));
    hello.push_str(" World!");
    println!("{hello}");

}