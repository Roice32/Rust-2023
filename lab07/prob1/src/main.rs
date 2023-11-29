use std::fmt;
use std::ops;

#[derive(Clone, Copy)]
struct Complex {
    real: f64,
    imag: f64,
}

impl Complex {
    fn new<T1, T2>(r: T1, i: T2) -> Complex
    where
        f64: From<T1> + From<T2>,
    {
        return Complex {
            real: f64::from(r),
            imag: f64::from(i),
        };
    }
    fn conjugate(&self) -> Complex {
        return Complex {
            real: self.real,
            imag: -self.imag,
        };
    }
}

impl From<i32> for Complex {
    fn from(s: i32) -> Complex {
        return Complex {
            real: s as f64,
            imag: 0 as f64,
        };
    }
}

impl From<f64> for Complex {
    fn from(s: f64) -> Complex {
        return Complex {
            real: s,
            imag: 0 as f64,
        };
    }
}

impl ops::Add for Complex {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        return Self {
            real: self.real + other.real,
            imag: self.imag + other.imag,
        };
    }
}

impl ops::Sub for Complex {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        return Self {
            real: self.real - other.real,
            imag: self.imag - other.imag,
        };
    }
}

impl ops::Mul for Complex {
    type Output = Self;
    fn mul(self, other: Self) -> Self {
        return Self {
            real: self.real * other.real - self.imag * other.imag,
            imag: self.real * other.imag + self.imag * other.real,
        };
    }
}

impl ops::Neg for Complex {
    type Output = Self;
    fn neg(self) -> Self {
        return Self {
            real: -self.real,
            imag: -self.imag,
        };
    }
}

impl fmt::Display for Complex {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Complex {
                real: 0.0,
                imag: 0.0,
            } => write!(f, "0"),
            Complex { real: 0.0, imag } => write!(f, "{}i", self.imag),
            Complex { real, imag: 0.0 } => write!(f, "{}", self.real),
            Complex { real, imag } if imag > 0 as f64 => write!(f, "{}+{}i", self.real, self.imag),
            _ => write!(f, "{}{}i", self.real, self.imag),
        }
    }
}

fn eq_rel(x: f64, y: f64) -> bool {
    (x - y).abs() < 0.001
}
// This is a macro that panics if 2 floats are not equal using an epsilon.
// You are not required to understand it yet, just to use it.
macro_rules! assert_eq_rel {
    ($x:expr, $y: expr) => {
        let x = $x as f64;
        let y = $y as f64;
        let r = eq_rel(x, y);
        assert!(r, "{} != {}", x, y);
    };
}

fn main() {
    let a = Complex::new(1.0, 2.0);
    assert_eq_rel!(a.real, 1);
    assert_eq_rel!(a.imag, 2);

    let b = Complex::new(2.0, 3);
    let c = a + b;
    assert_eq_rel!(c.real, 3);
    assert_eq_rel!(c.imag, 5);

    let d = c - a;
    assert_eq!(b, d);

    let e = (a * d).conjugate();
    assert_eq_rel!(e.imag, -7);

    let f = (a + b - d) * c;
    assert_eq!(f, Complex::new(-7, 11));

    // Note: .to_string() uses Display to format the type
    assert_eq!(Complex::new(1, 2).to_string(), "1+2i");
    assert_eq!(Complex::new(1, -2).to_string(), "1-2i");
    assert_eq!(Complex::new(0, 5).to_string(), "5i");
    assert_eq!(Complex::new(7, 0).to_string(), "7");
    assert_eq!(Complex::new(0, 0).to_string(), "0");

    let h = Complex::new(-4, -5);
    let i = h - (h + 5) * 2.0;
    assert_eq_rel!(i.real, -6);

    let j = -i + i;
    assert_eq_rel!(j.real, 0);
    assert_eq_rel!(j.imag, 0);

    println!("ok!");
}
