#![allow(dead_code)]

use rust_decimal::prelude::*;
use rust_decimal_macros::dec;
use wasm_bindgen::prelude::*;

use rusty_money::{crypto, define_currency_set, ExchangeRate, Money};

define_currency_set!(
  currencies {
    BTC: {
        code: "BTC",
        exponent: 8,
        locale: EnUs,
        minor_units: 100_000_000,
        name: "Bitcoin",
        symbol: "₿",
        symbol_first: true,
    },
    USD : {
        code: "USD",
        exponent: 2,
        locale: EnUs,
        minor_units: 1,
        name: "United States Dollar",
        symbol: "$",
        symbol_first: true,
    }
  }
);

#[wasm_bindgen]
pub fn mul(a: f64, b: f64) -> f64 {
    a * b
}

#[wasm_bindgen]
pub fn decimal_mul(a: f64, b: f64) -> f64 {
    let d = Decimal::from_f64(a).unwrap() * Decimal::from_f64(b).unwrap();
    d.to_f64().unwrap()
}

#[wasm_bindgen]
pub fn decimal_sum(a: f64, b: f64) -> f64 {
    let d = Decimal::from_f64(a).unwrap() + Decimal::from_f64(b).unwrap();
    d.to_f64().unwrap()
}

#[wasm_bindgen]
pub fn rusty_money_call(a: f64) -> f64 {
    let m = Money::from_major(a.to_i64().unwrap(), crypto::BTC);
    m.amount().to_f64().unwrap()
}

#[wasm_bindgen]
pub fn rusty_money_usd_to_btc(a: f64) -> f64 {
    let rate = ExchangeRate::new(currencies::USD, currencies::BTC, dec!(0.000052)).unwrap();
    let m = rate
        .convert(Money::from_decimal(
            Decimal::from_f64(a).unwrap(),
            currencies::USD,
        ))
        .unwrap();
    // println!("{:?}", m);

    m.amount().round_dp(8).to_f64().unwrap()
}

#[wasm_bindgen]
pub fn rusty_money_btc_to_usd(a: f64) -> f64 {
    let rate = ExchangeRate::new(currencies::BTC, currencies::USD, dec!(19000.446)).unwrap();
    let m = rate
        .convert(Money::from_decimal(
            Decimal::from_f64(a).unwrap(),
            currencies::BTC,
        ))
        .unwrap();
    // println!("{:?}", m);

    m.amount().round_dp(2).to_f64().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let d = 0.04355565;
        let a = 0.07;
        let b = 0.03;

        // decimal
        let dd = Decimal::from_f64(d).unwrap();
        let ad = Decimal::from_f64(a).unwrap();
        let bd = Decimal::from_f64(b).unwrap();

        let mul_a = mul(a, d);
        let mul_b = mul(b, d);

        let d_mul_a = dd * ad;
        let d_mul_b = dd * bd;

        println!("rust std");
        println!("100 {} {:.3}", d, d);
        println!("70 {} {:.8}", mul_a, mul_a);
        println!("30 {} {:.8}", mul_b, mul_b);

        println!("rust decimal");
        println!("100 {} {}", dd, dd.round_dp(3));
        println!("70 {} {}", d_mul_a, d_mul_a.round_dp(8));
        println!("30 {} {}", d_mul_b, d_mul_b.round_dp(8));

        println!("float 64");
        println!("std -> 0.1 + 0.2 = {}", 0.1 + 0.2);
        println!(
            "rust decimal -> 0.1 + 0.2 = {}",
            Decimal::from_f64(0.1).unwrap() + Decimal::from_f64(0.2).unwrap()
        );
    }

    #[test]
    fn money_works() {
        let v = 19000.0;
        let r = rusty_money_usd_to_btc(v);

        // let v2 = 0.00914451;
        let v2 = 0.00563675;
        let r2 = rusty_money_btc_to_usd(v2);

        println!("usd: {} to btc: {}", v, r);
        println!("btc: {} to usd: {}", v2, r2);
    }
}
