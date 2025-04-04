import { createUmi } from "@metaplex-foundation/umi-bundle-defaults";
import {
  createSignerFromKeypair,
  signerIdentity,
  generateSigner,
  percentAmount,
} from "@metaplex-foundation/umi";
import {
  createNft,
  mplTokenMetadata,
} from "@metaplex-foundation/mpl-token-metadata";

import wallet from "../wba-wallet.json";
import base58 from "bs58";

const RPC_ENDPOINT = "https://api.devnet.solana.com";
const umi = createUmi(RPC_ENDPOINT);

let keypair = umi.eddsa.createKeypairFromSecretKey(new Uint8Array(wallet));
const myKeypairSigner = createSignerFromKeypair(umi, keypair);
umi.use(signerIdentity(myKeypairSigner));
umi.use(mplTokenMetadata());

const mint = generateSigner(umi);

(async () => {
  let tx = createNft(umi, {
    mint,
    name: "Jeff NFT",
    symbol: "JEFFDGOAT",
    uri: "https://devnet.irys.xyz/943kBsUET3WK2D3Pkkvd5tuq9XCduMQ6gzc9t9VwkGrB",
    sellerFeeBasisPoints: percentAmount(0, 2),
    isCollection: false,
  });
  let result = await tx.sendAndConfirm(umi);
  const signature = base58.encode(result.signature);

  console.log(
    `Succesfully Minted! Check out your TX here:\nhttps://explorer.solana.com/tx/${signature}?cluster=devnet`
  );

  console.log("Mint Address: ", mint.publicKey);
})();

// Succesfully Minted! Check out your TX here:
// https://explorer.solana.com/tx/25y5HPAu6w2BZmyawEvvEPKHwdqith38aMxS9tmtuwy57npq1X1FfRd4Jz5ivu1UumZxDehDqCsDSxXwQb71jUz2?cluster=devnet
// Mint Address:  A5HHsn8sugPmGSPjZu4GBBPW9tW84Qm3DPNuFvL8cwMD
