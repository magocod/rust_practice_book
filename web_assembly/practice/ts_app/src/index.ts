import { mul } from "practice_wasm";

let d = 0.04355565;
let a = 0.07;
let b = 0.03;

let mul_a = mul(a, d);
let mul_b = mul(b, d);

console.log("100", d);
console.log("70", mul_a);
console.log("30", mul_b);

console.log("0.1 + 0.2 -> ", 0.1 + 0.2);
