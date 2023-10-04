fn coprimes(mut a: u32, mut b: u32) -> bool {
    if a * b == 0 {
        return false;
    };
    loop {
        if a < b {
            b -= a;
        } else if a > b {
            a -= b;
        } else {
            break;
        }
    }
    if a == 1 {
        return true;
    } else {
        return false;
    }
}

fn main() {
    let mut x = 0u32;
    let mut y;
    print!("Pairs of coprime numbers in [0,100]X[0,100]:");
    while x <= 100 {
        print!("\n\t");
        y = 0;
        while y <= 100 {
            if coprimes(x, y) == true {
                print!("({},{})", x, y);
            }
            y += 1;
        }
        x += 1;
    }
}
