fn prime(a: u32) -> bool {
    if a == 2 {
        return true;
    } else if a < 2 || a % 2 == 0 {
        return false;
    } else {
        let mut d = 3u32;
        while d * d <= a {
            if a % d == 0 {
                return false;
            }
            d += 2;
        }
        return true;
    }
}

fn main() {
    let mut n = 0u32;
    println!("Prime numbers in [0,100]:");
    while n <= 100 {
        if prime(n) == true {
            print!("{} ", n);
        }
        n += 1;
    }
}
