// dinero v1
// import Dinero from "dinero.js";

// dinero v2
import {dinero, Currency, createDinero} from 'dinero.js'
import {calculator} from "@dinero.js/calculator-bigint";

const dineroBigint = createDinero({ calculator });

// dinero v1
// export function toPrice(amount: number, precision: number, factor = Math.pow(10, 2)) {
//     return Dinero({ amount: amount * factor, precision })
// }

interface dineroFromFloatPayload<T> {
    amount: T;
    currency: Currency<T>;
    scale: T;
}

// dinero v2
export function dineroFromFloat({ amount, currency, scale }: dineroFromFloatPayload<number>) {
    // const { amount, currency, scale } = config;

    const factor = currency.base ** currency.exponent || scale;
    const update = Math.round(amount * factor);

    return dinero({ amount: update, currency, scale });
}

// export function dineroFromFloatV2({ amount, currency, scale }: dineroFromFloatPayload<bigint>) {
//     // const { amount, currency, scale } = config;
//
//     const factor = currency.base ** currency.exponent || scale;
//     const update = Math.round(amount * factor);
//
//     return dineroBigint({ amount: update, currency, scale });
// }
