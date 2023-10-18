fn add_space(s: &mut String, mut n: u32) {
    while n > 0 {
        s.push(' ');
        n -= 1;
    }
}

fn add_str(s: &mut String, slice: &str) {
    s.push_str(slice);
}

fn add_integer(s: &mut String, mut x: i32) {
    if x == 0 {
        s.push('0');
        return;
    }

    if x < 0 {
        s.push('-');
        x = -x;
    }

    let mut digit_place: i64 = 1;
    let mut no_digits: u8 = 0;
    while digit_place < x as i64 {
        digit_place *= 10;
        no_digits += 1;
    }
    if digit_place != x as i64 {
        digit_place /= 10;
    } else {
        no_digits += 1;
    }

    while no_digits >= 1 {
        s.push(((x as i64 / digit_place % 10) as u8 + b'0') as char);
        digit_place /= 10;
        no_digits -= 1;
        if no_digits != 0 && no_digits % 3 == 0 {
            s.push(',');
        }
    }
}

fn add_float(s: &mut String, mut x: f32) {
    if x == 0 as f32 {
        s.push('0');
        return;
    }
    if x < 0 as f32 {
        s.push('-');
        x = -x;
    }

    let whole_part: f32 = (x as u32) as f32;
    let mut fract_part: f32 = x - whole_part;

    add_integer(s, whole_part as i32);

    s.push('.');
    while fract_part != fract_part as i32 as f32 {
        fract_part *= 10 as f32;
        s.push(((fract_part as u32 % 10) as u8 + b'0') as char);
    }
}

fn main() {
    let mut s: String = String::from("");
    add_space(&mut s, 40);
    add_str(&mut s, "I 💚\n");
    add_space(&mut s, 40);
    add_str(&mut s, "RUST.\n\n");
    add_space(&mut s, 4);
    add_str(&mut s, "Most");
    add_space(&mut s, 12);
    add_str(&mut s, "create");
    add_space(&mut s, 5);
    add_integer(&mut s, 306437);
    add_space(&mut s, 11);
    add_str(&mut s, "and");
    add_space(&mut s, 5);
    add_str(&mut s, "latest");
    add_space(&mut s, 9);
    add_str(&mut s, "is\n");
    add_space(&mut s, 9);
    add_str(&mut s, "downloaded");
    add_space(&mut s, 8);
    add_str(&mut s, "has");
    add_space(&mut s, 9);
    add_str(&mut s, "downloads");
    add_space(&mut s, 5);
    add_str(&mut s, "the");
    add_space(&mut s, 8);
    add_str(&mut s, "version");
    add_space(&mut s, 4);
    add_float(&mut s, 2.038);
    add_str(&mut s, ".");
    println!("{s}");
}
/*
                                        I 💚
                                        RUST.

    Most            crate      306_437_968           and     lastest         is
         downloaded        has             downloads     the         version    2.038.

(Aproximativ, mie-mi baga cifre dupa 38; also nu stau sa caut un caracter ASCII pt. inima.)
*/
