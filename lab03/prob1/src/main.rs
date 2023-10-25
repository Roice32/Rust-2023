use std::u16::MAX;

fn is_prime(a: u16) -> bool {
    if a==2 {
        return true;
    }
    if a<2 || a%2==0 {
        return false;
    }
    let mut d: u16 = 3;
    while d as u32 * d as u32 <= a as u32 {
        if a%d==0 {
            return false;
        }
        d += 2;
    }
    return true;
}

fn next_prime(x: u16) -> Option<u16> {
    let mut next : u32 = x as u32 + 1;
    while next<=std::u16::MAX as u32 && !is_prime(next as u16) {
        next += 1;
    }
    if next>std::u16::MAX as u32 {
        return None;
    }
    return Some(next as u16);
}

fn main() { // std::u16::MAX = 65535
    let mut number: u16 = 65500;
    loop {
        match next_prime(number) {
            Some(x) => println!("{} is the next prime number after {}.", x, number),
            None => { println!("Exceeded max value of u16 trying to find next prime after {}.", number); break}
        }
        number += 1;
    }
}
