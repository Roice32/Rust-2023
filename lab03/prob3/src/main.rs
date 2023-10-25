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

fn use_functions() {
    let a: u32 = 5;
    let b: u32 = 20;
    let c: u32 = 10;
    let d: u32 = std::u32::MAX - 1;
    match checked_addition(a, b) {
        Ok(s) => println!("{a} + {b} = {s}"),
        Err(e) => println!("{a} + {b} overflows u32 ({:?})", e),
    }
    match checked_addition(a, d) {
        Ok(s) => println!("{a} + {d} = {s}"),
        Err(e) => println!("{a} + {d} overflows u32 ({:?})", e),
    }
    match checked_multiplication(a, c) {
        Ok(p) => println!("{a} * {c} = {p}"),
        Err(e) => println!("{a} * {c} overflows u32 ({:?})", e),
    }
    match checked_multiplication(a, d) {
        Ok(p) => println!("{a} * {d} = {p}"),
        Err(e) => println!("{a} * {d} overflows u32 ({:?})", e),
    }
}

fn main() {
    use_functions();
}
