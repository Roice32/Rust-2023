#[derive(Debug)]
enum Error {
    AdditionOverflow,
    MultiplcationOverflow,
}

fn checked_addition(a: u32, b: u32) -> Result<u32, Error> {
    if std::u32::MAX - b < a {
        return Err(Error::AdditionOverflow);
    }
    return Ok(a + b);
}

fn checked_multiplication(a: u32, b: u32) -> Result<u32, Error> {
    if std::u32::MAX / b < a {
        return Err(Error::MultiplcationOverflow);
    }
    return Ok(a * b);
}

fn use_functions() -> Result<bool, Error> {
    let a: u32 = 5;
    let b: u32 = 20;
    let c: u32 = 10;
    let d: u32 = std::u32::MAX - 1;
    let sum_a_b = checked_addition(a, b)?;
    let sum_a_d = checked_addition(a, c)?;
    let prod_a_c = checked_multiplication(a, c)?;
    let prod_a_d = checked_multiplication(a, d)?;
    return Ok(sum_a_b == sum_a_d && prod_a_c == prod_a_d); // Don't really know what to write here.
}

fn main() {
    match use_functions() {
        Ok(b) => println!("Successfully executed (returned {b})."),
        Err(e) => println!("An error occured ({:?})", e),
    }
}
