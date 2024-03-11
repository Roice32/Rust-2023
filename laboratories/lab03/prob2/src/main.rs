fn checked_addition(a: u32, b: u32) -> u32 {
    if std::u32::MAX - b < a {
        panic!("at the Disco (Addition with overflow).");
    }
    return a + b;
}

fn checked_multiplication(a: u32, b: u32) -> u32 {
    if std::u32::MAX / b < a {
        panic!("at the Disco (Multiplication with overflow).");
    }
    return a * b;
}

fn main() {
    let x = checked_addition(std::u32::MAX - 5, 4);
    println!("Addition: {x}");
    let y: u32 = checked_multiplication(std::u32::MAX, 5);
    println!("Multiplcation: {y}");
}
