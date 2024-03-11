use std::cell::RefCell;
use std::collections::HashMap;
use std::io;

struct Cache {
    pairs: RefCell<HashMap<u32, bool>>,
}

impl Cache {
    pub fn new() -> Self {
        Self {
            pairs: RefCell::new(HashMap::new()),
        }
    }
    pub fn is_stored(&self, number: u32) -> bool {
        let b_pairs = self.pairs.borrow();
        return b_pairs.contains_key(&number);
    }
    pub fn get_primality(&self, number: u32) -> bool {
        let b_pairs = self.pairs.borrow();
        match b_pairs.get(&number) {
            Some(b) => {
                return *b;
            }
            None => {
                return false;
            }
        }
    }
    pub fn store(&self, number: u32) {
        let mut b_pairs = self.pairs.borrow_mut();
        b_pairs.insert(number, is_prime(number));
    }
}

fn is_prime(a: u32) -> bool {
    if a < 2 {
        return false;
    }
    if a == 2 {
        return true;
    }
    if a % 2 == 0 {
        return false;
    }
    let mut d: u32 = 3;
    loop {
        if a % d == 0 {
            return false;
        }
        if d * d > a {
            break;
        }
        d += 2;
    }
    return true;
}

fn main() {
    let cache = Cache::new();
    let mut input: String = String::new();
    let mut number: u32;
    loop {
        println!("Number, or 'q' for quit: ");
        input.clear();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input.");

        input = String::from(input.trim());
        if input == "q" {
            print!("Ok bye.");
            break;
        }
        number = input.parse().expect("Expected natural number or 'q'.");

        if cache.is_stored(number) {
            println!("Found {} in Cache.", number);
        } else {
            cache.store(number);
            println!("Stored {} in Cache.", number);
        }

        print!("{} is ", number);
        if !cache.get_primality(number) {
            print!("not ");
        }
        println!("prime.");
    }
}
