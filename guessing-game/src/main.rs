fn main() {
    println!("Hello World");
    println!("Please input your guess.");
    let mut guess = String::new();

    std::io::stdin()
        .read_line(&mut guess)
        .expect("Read Failed!");

    println!("{}", guess);
}
