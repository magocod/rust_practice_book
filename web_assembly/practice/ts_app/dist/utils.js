"use strict";
// dinero v1
// import Dinero from "dinero.js";
Object.defineProperty(exports, "__esModule", { value: true });
// dinero v2
var dinero_js_1 = require("dinero.js");
// dinero v2
function dineroFromFloat(_a) {
    // const { amount, currency, scale } = config;
    var amount = _a.amount, currency = _a.currency, scale = _a.scale;
    var factor = Math.pow(currency.base, currency.exponent) || scale;
    var update = Math.round(amount * factor);
    return (0, dinero_js_1.dinero)({ amount: update, currency: currency, scale: scale });
}
