extern crate num_traits;

use num_traits::identities::Zero;
use num_traits::Num as Number;

use std::fmt::{Display, Formatter, Result};
use std::ops::{Add, Mul, Neg};

/// ## Example.
///
///
///
/// ```
///  extern crate complex;
///
///   fn main() {
///    let lo = complex::Complex32::new(3.0, 3.0);
///
///    let la = complex::Complex32::new(3.0, 3.0);
///
///     println!("ADD: {}", lo + la);
///     println!("MULT: {}", lo * la);
///     println!("DOT Product: ({}).({}) = {}", lo, la, lo.dot_product(la));
///     println!("Conjugate: {}", lo.conjugate());
///   }
/// ```
#[derive(PartialEq, PartialOrd, Eq, Copy, Clone, Hash, Debug)]
pub struct Complex<T: Number + Zero> {
    pub Imaginaire: T,
    pub Real: T,
}

impl<T: Copy + Number + Neg<Output = T> + Zero + Mul<T>> Complex<T> {
    //  Dot product between two complex numbers.
    pub fn dot_product(&self, another: Complex<T>) -> T {
        (self.Real * another.Imaginaire) - (self.Imaginaire * another.Real)
    }

    //  Get conjugated for a given complex number.
    pub fn conjugate(&self) -> Complex<T> {
        Complex {
            Imaginaire: -self.Imaginaire,
            Real: self.Real,
        }
    }

    pub fn new(im: T, re: T) -> Complex<T> {
        Complex {
            Real: re,
            Imaginaire: im,
        }
    }
}

impl<T: Copy + Number> Add<Complex<T>> for Complex<T> {
    type Output = Complex<T>;

    // + overloading for better clearness.
    fn add(self, another: Complex<T>) -> Complex<T> {
        Complex {
            Imaginaire: another.Imaginaire + self.Imaginaire,
            Real: another.Real + self.Real,
        }
    }
}

impl<T: Copy + Number + Neg<Output = T>> Mul<Complex<T>> for Complex<T> {
    type Output = Complex<T>;

    // * overloading for better clearness.
    fn mul(self, another: Complex<T>) -> Complex<T> {
        let real = (self.Real * another.Real) - (self.Imaginaire * another.Imaginaire);
        let im = (self.Real * another.Imaginaire) + (self.Imaginaire * another.Real);

        Complex {
            Real: real,
            Imaginaire: im,
        }
    }
}

impl<T: Clone + Copy + Neg<Output = T> + Number + Zero> Zero for Complex<T> {
    fn zero() -> Complex<T> {
        Complex::new(T::zero(), T::zero())
    }

    fn is_zero(&self) -> bool {
        self.Real.is_zero() && self.Imaginaire.is_zero()
    }
}

impl<T: Copy + Number + Zero + PartialOrd + Display> Display for Complex<T> {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match self {
            &Complex {
                Real: real,
                Imaginaire: im,
            } if real.is_zero() && !im.is_zero() => write!(f, "{}i", im),
            &Complex {
                Real: real,
                Imaginaire: im,
            } if !real.is_zero() && im.is_zero() => write!(f, "{}", real),

            &Complex {
                Real: real,
                Imaginaire: im,
            } if im < T::zero() => write!(f, "{}{}i", real, im),
            &Complex {
                Real: real,
                Imaginaire: im,
            } => write!(f, "{}+{}i", real, im),
        }
    }
}

pub type Complex32 = Complex<f32>;
pub type Complex64 = Complex<f64>;

#[cfg(test)]
mod test {
    use super::{Complex, Complex32, Complex64};

    #[test]
    fn test_f32() {
        let lo = Complex32::new(3.0, 3.0);
        let la = Complex32::new(3.0, 3.0);

        assert_eq!(lo + la, Complex32::new(6.0, 6.0), "ADD");
        assert_eq!(lo * la, Complex32::new(18.0, 0.0), "MULT");
        assert_eq!(lo.dot_product(la), 0.0, "DOT Product");
        assert_eq!(lo.conjugate(), Complex32::new(-3.0, 3.0), "Conjugate");
    }

    #[test]
    fn test_f64() {
        let lo = Complex64::new(3.0, 3.0);
        let la = Complex64::new(3.0, 3.0);

        assert_eq!(lo + la, Complex64::new(6.0, 6.0), "ADD");
        assert_eq!(lo * la, Complex64::new(18.0, 0.0), "MULT");
        assert_eq!(lo.dot_product(la), 0.0, "DOT Product");
        assert_eq!(lo.conjugate(), Complex64::new(-3.0, 3.0), "Conjugate");
    }

    #[test]
    fn test_128() {
        let lo = Complex::<i128>::new(3, 3);
        let la = Complex::<i128>::new(3, 3);

        assert_eq!(lo + la, Complex::<i128>::new(6, 6), "ADD");
        assert_eq!(lo * la, Complex::<i128>::new(18, 0), "MULT");
        assert_eq!(lo.dot_product(la), 0, "DOT Product");
        assert_eq!(lo.conjugate(), Complex::<i128>::new(-3, 3), "Conjugate");
    }

    #[test]
    fn test_display() {
        let lo = Complex32::new(3.0, 3.0);
        let la = Complex32::new(3.0, 3.0);

        println!("ADD: {}", lo + la);
        println!("MULT: {}", lo * la);
        println!("DOT Product: ({}).({}) = {}", lo, la, lo.dot_product(la));
        println!("Conjugate: {}", lo.conjugate());
    }
}
