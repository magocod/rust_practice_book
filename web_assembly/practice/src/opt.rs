use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn mul(a: f64, b: f64) -> f64 {
    a * b
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let d = 0.04355565;
        let a = 0.07;
        let b = 0.03;

        let mul_a = mul(a, d);
        let mul_b = mul(b, d);

        println!("100 {} {:.3}", d, d);
        println!("70 {} {:.8}", mul_a, mul_a);
        println!("30 {} {:.8}", mul_b, mul_b);

        println!("0.1 + 0.2: {:.8}", 0.1 + 0.2);
    }
}