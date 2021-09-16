#  Complex numbers in Rust 

on Rust (stable 1.55)

for types like `Complex32, Complex64 or even Complex<i28>`

Part of [blog post](https://earvinkayonga.com/posts/implement-complex-numbers-in-rust/)

```
    extern crate complex;

    fn main() {
        let lo = Complex32::new(3.0, 3.0);
        let la = Complex32::new(3.0, 3.0);

        println!("ADD: {}", lo + la);
        println!("MULT: {}", lo * la);
        println!("DOT Product: ({}).({}) = {}", lo, la, lo.dot_product(la));
        println!("Conjugate: {}", lo.conjugate());
    }
```

```
    ADD: 6+6i
    MULT: 0+18i
    DOT Product: (3+3i).(3+3i) = 0
    Conjugate: 3-3i
```