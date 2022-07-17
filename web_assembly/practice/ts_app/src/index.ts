import {
    mul,
    decimal_mul,
    decimal_sum,
    rusty_money_usd_to_btc,
    rusty_money_btc_to_usd
} from "practice_wasm";

// dinero v1
// import Dinero from "dinero.js";
// import { toPrice } from "./utils"

// dinero v2
import {dinero, toFormat, Transformer, allocate, subtract, Currency, createDinero} from 'dinero.js';
import { calculator } from '@dinero.js/calculator-bigint';
import { USD } from '@dinero.js/currencies';
// import { dineroFromFloatV2 } from "./utils"

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

console.log("")

console.log("rust decimal");

console.log("100", d);
console.log("70", d_mul_a);
console.log("30", d_mul_b);

console.log("")

console.log("float 64")
console.log("js std -> 0.1 + 0.2 = ", 0.1 + 0.2);
console.log("rust decimal -> 0.1 + 0.2 = ", decimal_sum(0.1, 0.2));

console.log("")

console.log("rusty money - rate")

let v = 19000.0;
let r = rusty_money_usd_to_btc(v);

// let v2 = 0.00914451;
let v2 = 0.00563675;
let r2 = rusty_money_btc_to_usd(v2);

console.log(`usd: ${v} to btc: ${r}`)
console.log(`btc: ${v2} to usd: ${r2}`)

console.log("")

let originalValue = 999.99
console.log("originalValue", originalValue)

const per50 = originalValue * 0.50;
console.log("per50", per50, "-> toFixed 2", per50.toFixed(2))

const updateValue = originalValue - per50;
console.log("updateValue", updateValue)

console.log("")

// console.log("Dinero v1")
// // const price = Dinero({ amount: originalValue, currency: 'USD', precision: 2 })
// const price = toPrice(originalValue, 2)
// console.log("originalValue", price.toFormat())
//
// const dinPer50 = price.percentage(50, 'DOWN');
// // const dinPer50 = toPrice(per50, 2)
// console.log("per50", dinPer50.toFormat())
//
// const dinUpdateValue = price.subtract(dinPer50);
// console.log("updateValue", dinUpdateValue.toFormat())

console.log("Dinero v2")

const transformer: Transformer<number> = ({ amount, currency }) => `${currency.code} ${amount}`;

const price = dinero({ amount: originalValue * Math.pow(10, 2), currency: USD })
// const price = toPrice(originalValue, 2)
// console.log("originalValue", price.toJSON())
console.log("originalValue", toFormat(price, transformer))

const [dinPer50, dinPerB] = allocate(price, [50, 50]);
// const dinPer50 = toPrice(per50, 2)
// console.log("per50", dinPer50.toJSON(), dinPerB.toJSON())
console.log("per50", toFormat(dinPer50, transformer), "<- ->", toFormat(dinPerB, transformer))

const dinUpdateValue = subtract(price, dinPer50);
// console.log("updateValue", dinUpdateValue.toJSON())
console.log("updateValue", toFormat(dinUpdateValue, transformer))

// const BTC: Currency<number> = {
//     code: 'BTC',
//     base: 10,
//     exponent: 8,
// };
//
// console.log("")
//
// const dineroBigint = createDinero({ calculator });
//
// const bigPrice = dineroBigint({ amount: 1000000000000000n, currency: BTC })
// // const price = toPrice(originalValue, 2)
// // console.log("originalValue", price.toJSON())
// console.log("originalValue", toFormat(price, transformer))
