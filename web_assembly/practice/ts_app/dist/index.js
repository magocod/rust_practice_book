"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
var practice_wasm_1 = require("practice_wasm");
// dinero v1
// import Dinero from "dinero.js";
// import { toPrice } from "./utils"
// dinero v2
var dinero_js_1 = require("dinero.js");
var currencies_1 = require("@dinero.js/currencies");
// import { dineroFromFloatV2 } from "./utils"
var d = 0.04355565;
var a = 0.07;
var b = 0.03;
var mul_a = (0, practice_wasm_1.mul)(a, d);
var mul_b = (0, practice_wasm_1.mul)(b, d);
var d_mul_a = (0, practice_wasm_1.decimal_mul)(a, d);
var d_mul_b = (0, practice_wasm_1.decimal_mul)(b, d);
console.log("rust std");
console.log("100", d);
console.log("70", mul_a);
console.log("30", mul_b);
console.log("");
console.log("rust decimal");
console.log("100", d);
console.log("70", d_mul_a);
console.log("30", d_mul_b);
console.log("");
console.log("float 64");
console.log("js std -> 0.1 + 0.2 = ", 0.1 + 0.2);
console.log("rust decimal -> 0.1 + 0.2 = ", (0, practice_wasm_1.decimal_sum)(0.1, 0.2));
console.log("");
console.log("rusty money - rate");
var v = 19000.0;
var r = (0, practice_wasm_1.rusty_money_usd_to_btc)(v);
// let v2 = 0.00914451;
var v2 = 0.00563675;
var r2 = (0, practice_wasm_1.rusty_money_btc_to_usd)(v2);
console.log("usd: ".concat(v, " to btc: ").concat(r));
console.log("btc: ".concat(v2, " to usd: ").concat(r2));
console.log("");
var originalValue = 999.99;
console.log("originalValue", originalValue);
var per50 = originalValue * 0.50;
console.log("per50", per50, "-> toFixed 2", per50.toFixed(2));
var updateValue = originalValue - per50;
console.log("updateValue", updateValue);
console.log("");
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
console.log("Dinero v2");
var transformer = function (_a) {
    var amount = _a.amount, currency = _a.currency;
    return "".concat(currency.code, " ").concat(amount);
};
var price = (0, dinero_js_1.dinero)({ amount: originalValue * Math.pow(10, 2), currency: currencies_1.USD });
// const price = toPrice(originalValue, 2)
// console.log("originalValue", price.toJSON())
console.log("originalValue", (0, dinero_js_1.toFormat)(price, transformer));
var _a = (0, dinero_js_1.allocate)(price, [50, 50]), dinPer50 = _a[0], dinPerB = _a[1];
// const dinPer50 = toPrice(per50, 2)
// console.log("per50", dinPer50.toJSON(), dinPerB.toJSON())
console.log("per50", (0, dinero_js_1.toFormat)(dinPer50, transformer), "<- ->", (0, dinero_js_1.toFormat)(dinPerB, transformer));
var dinUpdateValue = (0, dinero_js_1.subtract)(price, dinPer50);
// console.log("updateValue", dinUpdateValue.toJSON())
console.log("updateValue", (0, dinero_js_1.toFormat)(dinUpdateValue, transformer));
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
