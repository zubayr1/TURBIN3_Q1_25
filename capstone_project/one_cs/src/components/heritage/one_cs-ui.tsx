"use client";

import { useState, useEffect } from "react";
import toast from "react-hot-toast";

import { PublicKey } from "@solana/web3.js";
import { useWallet, useConnection } from "@solana/wallet-adapter-react";
import { BN } from "@coral-xyz/anchor";
import { Connection } from "@solana/web3.js";
import { TOKEN_PROGRAM_ID } from "@solana/spl-token";
import { createUmi } from "@metaplex-foundation/umi-bundle-defaults";
import { mplTokenMetadata } from "@metaplex-foundation/mpl-token-metadata";
import { fetchDigitalAsset } from "@metaplex-foundation/mpl-token-metadata";
import { publicKey as toUmiPublicKey } from "@metaplex-foundation/umi";

import { useOneCsProgram } from "./one_cs-data-access";

export async function getTokenMintAddresses(
  connection: Connection,
  userPubkey: PublicKey
): Promise<
  {
    mint: PublicKey;
    balance: string;
    name: string;
    symbol: string;
    decimals: number;
  }[]
> {
  const umi = createUmi(connection.rpcEndpoint).use(mplTokenMetadata());

  // Fetch all token accounts for the user
  const tokenAccounts = await connection.getParsedTokenAccountsByOwner(
    userPubkey,
    {
      programId: TOKEN_PROGRAM_ID,
    }
  );

  // Process each token account
  const tokens = await Promise.all(
    tokenAccounts.value.map(async (tokenAccount) => {
      const parsedInfo = tokenAccount.account.data.parsed.info;
      const mint = new PublicKey(parsedInfo.mint);
      let name = "";
      let symbol = "";
      let decimals = parsedInfo.tokenAmount.decimals;

      // Skip NFTs (tokens with 0 decimals)
      if (parsedInfo.tokenAmount.decimals === 0) return null;

      // Attempt to fetch metadata using the Metaplex Token Metadata program
      try {
        const asset = await fetchDigitalAsset(
          umi,
          toUmiPublicKey(mint.toString())
        );
        name = asset.metadata.name;
        symbol = asset.metadata.symbol;
      } catch (error) {
        // Fallback to using mint address if metadata fetch fails
        name = `Token ${mint.toBase58().slice(0, 4)}...${mint
          .toBase58()
          .slice(-4)}`;
        symbol = mint.toBase58().slice(0, 4);
      }

      return {
        mint,
        balance: parsedInfo.tokenAmount.uiAmount.toString(),
        name,
        symbol,
        decimals,
      };
    })
  );

  // Filter out null values (NFTs) and return
  return tokens.filter(
    (token): token is NonNullable<typeof token> => token !== null
  );
}

export function OneCsCreate() {
  const { initEscrow, depositTokens, encapsulateToken } = useOneCsProgram();
  const { publicKey } = useWallet();
  const { connection } = useConnection();
  const [availableTokens, setAvailableTokens] = useState<
    {
      mint: PublicKey;
      balance: string;
      name: string;
      symbol: string;
      decimals: number;
    }[]
  >([]);

  const [label, setLabel] = useState("");
  const [tokenMint, setTokenMint] = useState("");
  const [tokenAmount, setTokenAmount] = useState("");

  useEffect(() => {
    if (publicKey) {
      getTokenMintAddresses(connection, publicKey)
        .then(setAvailableTokens)
        .catch(console.error);
    }
  }, [connection, publicKey]);

  const handleSubmit = async () => {
    if (!publicKey || !label || !tokenMint || !tokenAmount) {
      toast.error("Please fill in all fields");
      return;
    }

    const tokenData = availableTokens.find(
      (t) => t.mint.toBase58() === tokenMint
    );
    if (!tokenData) {
      toast.error("Selected token not found");
      return;
    }

    try {
      const tokenMintPubkey = new PublicKey(tokenMint);

      // First init the escrow
      await initEscrow.mutateAsync({
        label,
        creator: publicKey,
        tokenMint: tokenMintPubkey,
      });

      // If init succeeds, immediately deposit tokens
      await depositTokens.mutateAsync({
        label,
        creator: publicKey,
        tokenMint: tokenMintPubkey,
        amount: new BN(
          parseFloat(tokenAmount) * Math.pow(10, tokenData.decimals)
        ),
      });

      // Encapsulate the token
      await encapsulateToken.mutateAsync({
        label,
        creator: publicKey,
        tokenMint: tokenMintPubkey,
      });

      toast.success("Token encapsulated successfully!");
    } catch (err) {
      toast.error("Error in operation: " + err);
    }
  };

  return (
    <div className="card bg-base-200 shadow-xl">
      <div className="card-body">
        <h2 className="card-title">Encapsulate Token</h2>

        <div className="form-control w-full">
          <label className="label">
            <span className="label-text font-medium">Label</span>
          </label>
          <input
            type="text"
            placeholder="Enter a unique identifier"
            className="input input-bordered w-full"
            value={label}
            onChange={(e) => setLabel(e.target.value)}
          />
        </div>

        <div className="form-control w-full">
          <label className="label">
            <span className="label-text font-medium">Token Mint</span>
          </label>
          <select
            className="select select-bordered w-full"
            value={tokenMint}
            onChange={(e) => setTokenMint(e.target.value)}
          >
            <option value="">Select a token</option>
            {availableTokens.map((token) => (
              <option key={token.mint.toBase58()} value={token.mint.toBase58()}>
                {token.symbol} (Balance: {token.balance})
              </option>
            ))}
          </select>
        </div>

        <div className="form-control w-full">
          <label className="label">
            <span className="label-text font-medium">Token Amount</span>
          </label>
          <input
            type="number"
            placeholder="Enter amount of tokens"
            className="input input-bordered w-full"
            value={tokenAmount}
            onChange={(e) => setTokenAmount(e.target.value)}
          />
        </div>

        <div className="card-actions justify-end mt-4">
          <button
            className="btn btn-primary"
            onClick={handleSubmit}
            disabled={
              initEscrow.isPending || !label || !tokenMint || !tokenAmount
            }
          >
            {initEscrow.isPending ? (
              <>
                <span className="loading loading-spinner loading-sm"></span>
                Encapsulating...
              </>
            ) : (
              "Encapsulate"
            )}
          </button>
        </div>
      </div>
    </div>
  );
}
