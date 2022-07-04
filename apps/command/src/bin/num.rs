fn main() {
    println!("0.1 + 0.2 -> {}", 0.1 + 0.2);

    let v = 12.34567_f64;
    let f = to_fixed(v);
    let s = format!("{:.2}", v);

    println!("{:.5} {:.5}", v, f);

    println!("{} -> to_fixed: {}", v, f);
    println!("{} -> to_fixed with format: {}", v, s);
}

fn to_fixed(x: f64) -> f64 {
    let y = (x * 100.0).round() / 100.0;

    y
}
