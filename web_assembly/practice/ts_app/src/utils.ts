// dinero v1
// import Dinero from "dinero.js";

// dinero v2
import { dinero, Currency } from 'dinero.js'

// dinero v1
// export function toPrice(amount: number, precision: number, factor = Math.pow(10, 2)) {
//     return Dinero({ amount: amount * factor, precision })
// }

interface dineroFromFloatPayload {
    amount: number;
    currency: Currency<number>;
    scale: number;
}

// dinero v2
function dineroFromFloat({ amount, currency, scale }: dineroFromFloatPayload) {
    // const { amount, currency, scale } = config;

    const factor = currency.base ** currency.exponent || scale;
    const update = Math.round(amount * factor);

    return dinero({ amount: update, currency, scale });
}
