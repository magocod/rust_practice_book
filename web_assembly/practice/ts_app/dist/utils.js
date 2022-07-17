"use strict";
var __importDefault = (this && this.__importDefault) || function (mod) {
    return (mod && mod.__esModule) ? mod : { "default": mod };
};
Object.defineProperty(exports, "__esModule", { value: true });
exports.toPrice = void 0;
var dinero_js_1 = __importDefault(require("dinero.js"));
function toPrice(amount, precision, factor) {
    if (factor === void 0) { factor = Math.pow(10, 2); }
    return (0, dinero_js_1.default)({ amount: amount * factor, precision: precision });
}
exports.toPrice = toPrice;
