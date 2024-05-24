import "dotenv/config"
import { getKeypairFromEnvironment } from "@solana-developers/helpers";
import { Keypair } from "@solana/web3.js";

const keypair = getKeypairFromEnvironment("SECRET_KEY");

console.log(
  `✅ Finished! We've loaded our secret key securely, using an env file!`
);
console.log('public key is :',keypair.publicKey.toBase58());
console.log('secare key is :',keypair.secretKey);