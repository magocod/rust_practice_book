import Dinero from "dinero.js";

export function toPrice(amount: number, precision: number, factor = Math.pow(10, 2)) {
    return Dinero({ amount: amount * factor, precision })
}
