import {
    mul,
    decimal_mul,
    decimal_sum,
    rusty_money_usd_to_btc,
    rusty_money_btc_to_usd
} from "practice_wasm";

let d = 0.04355565;
let a = 0.07;
let b = 0.03;

let mul_a = mul(a, d);
let mul_b = mul(b, d);

let d_mul_a = decimal_mul(a, d);
let d_mul_b = decimal_mul(b, d);

console.log("rust std");
console.log("100", d);
console.log("70", mul_a);
console.log("30", mul_b);

console.log("rust decimal");

console.log("100", d);
console.log("70", d_mul_a);
console.log("30", d_mul_b);

console.log("float 64")
console.log("js std -> 0.1 + 0.2 = ", 0.1 + 0.2);
console.log("rust decimal -> 0.1 + 0.2 = ", decimal_sum(0.1, 0.2));

console.log("rusty money - rate")

let v = 19000.0;
let r = rusty_money_usd_to_btc(v);

// let v2 = 0.00914451;
let v2 = 0.00563675;
let r2 = rusty_money_btc_to_usd(v2);

console.log(`usd: ${v} to btc: ${r}`)
console.log(`btc: ${v2} to usd: ${r2}`)

