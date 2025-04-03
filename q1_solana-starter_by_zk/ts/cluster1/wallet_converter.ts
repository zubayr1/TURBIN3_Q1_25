import pkg from "bs58";
import wallet from "../wba-wallet.json";

const { decode } = pkg;

const key = wallet;
const decoded = decode(key);

console.log(JSON.stringify(Array.from(decoded)));
