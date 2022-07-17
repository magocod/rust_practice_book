"use strict";
// dinero v1
// import Dinero from "dinero.js";
Object.defineProperty(exports, "__esModule", { value: true });
exports.dineroFromFloat = void 0;
// dinero v2
var dinero_js_1 = require("dinero.js");
var calculator_bigint_1 = require("@dinero.js/calculator-bigint");
var dineroBigint = (0, dinero_js_1.createDinero)({ calculator: calculator_bigint_1.calculator });
// dinero v2
function dineroFromFloat(_a) {
    // const { amount, currency, scale } = config;
    var amount = _a.amount, currency = _a.currency, scale = _a.scale;
    var factor = Math.pow(currency.base, currency.exponent) || scale;
    var update = Math.round(amount * factor);
    return (0, dinero_js_1.dinero)({ amount: update, currency: currency, scale: scale });
}
exports.dineroFromFloat = dineroFromFloat;
// export function dineroFromFloatV2({ amount, currency, scale }: dineroFromFloatPayload<bigint>) {
//     // const { amount, currency, scale } = config;
//
//     const factor = currency.base ** currency.exponent || scale;
//     const update = Math.round(amount * factor);
//
//     return dineroBigint({ amount: update, currency, scale });
// }
