extern crate num_traits;

pub mod complex {
    use num_traits::identities::Zero;
    use num_traits::Num as Number;

    use std::fmt::{Display, Formatter, Result};
    use std::ops::{Add, Mul, Neg};

    #[derive(PartialEq, PartialOrd, Eq, Copy, Clone, Hash, Debug)]
    pub struct Complex<T: Number + Zero> {
        pub Imaginaire: T,
        pub Real: T,
    }

    impl<T: Copy + Number + Neg<Output = T> + Zero + Mul<T>> Complex<T> {
        pub fn dot_product(&self, another: Complex<T>) -> T {
            (self.Real * another.Imaginaire) - (self.Imaginaire * another.Real)
        }

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

    impl<T: Copy + Number + Neg<Output = T>> Add<Complex<T>> for Complex<T> {
        type Output = Complex<T>;

        fn add(self, another: Complex<T>) -> Complex<T> {
            Complex {
                Imaginaire: another.Imaginaire + self.Imaginaire,
                Real: another.Real + self.Real,
            }
        }
    }

    impl<T: Copy + Number + Neg<Output = T>> Mul<Complex<T>> for Complex<T> {
        type Output = Complex<T>;

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
            match self.Imaginaire {
                number if number.is_zero() => write!(f, "{}", self.Real),
                number if number < T::zero() => write!(f, "{}{}i", self.Real, number),
                _ => write!(f, "{}+{}i", self.Real, self.Imaginaire),
            }
        }
    }

    pub type Complex32 = Complex<f32>;
    pub type Complex64 = Complex<f64>;
}

#[cfg(test)]
mod test {
    #![allow(non_upper_case_globals)]

    use super::{Complex32, Complex64};

}

fn main() {
    let lo = complex::Complex32::new(3.0, 3.0);

    let la = complex::Complex32::new(3.0, 3.0);

    println!("ADD: {}", lo + la);
    println!("MULT: {}", lo * la);
    println!("DOT Product: ({}).({}) = {}", lo, la, lo.dot_product(la));
    println!("Conjugate: {}", lo.conjugate());
}
