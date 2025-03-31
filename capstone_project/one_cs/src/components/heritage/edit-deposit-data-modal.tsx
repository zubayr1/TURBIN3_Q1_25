import toast from "react-hot-toast";
import { useState } from "react";

import { BN } from "@coral-xyz/anchor";
import { PublicKey } from "@solana/web3.js";
import { useWallet } from "@solana/wallet-adapter-react";

import { useOneCsProgramAccount } from "./one_cs-data-access";
import { ellipsify } from "../ui/ui-layout";

interface UseEditDepositDataProps {
  account: PublicKey;
  data: {
    label: string;
    creator: PublicKey;
    owner: PublicKey;
    data: {
      text: string | null;
      token: { tokenMint: PublicKey; tokenAmount: BN } | null;
    };
  };
  onSuccess: () => void;
}

export function useEditDepositData({
  account,
  data,
  onSuccess,
}: UseEditDepositDataProps) {
  const { editDepositTokenData } = useOneCsProgramAccount({ account });
  const { publicKey } = useWallet();

  const handleEditDepositData = async (amount: BN) => {
    try {
      if (!data?.data.token || !publicKey) {
        toast.error(
          publicKey ? "No token data found" : "Please connect your wallet"
        );
        return;
      }

      editDepositTokenData.mutateAsync({
        label: data?.label || "",
        creator: data?.creator,
        payer: publicKey,
        amount,
        tokenMint: data.data.token.tokenMint,
      });
      onSuccess();
    } catch (err) {
      toast.error("Error depositing tokens: " + err);
    }
  };

  return {
    handleEditDepositData,
    isEditDepositPending: editDepositTokenData.isPending,
  };
}

interface EditDepositDataModalProps {
  isOpen: boolean;
  onClose: () => void;
  onSubmit: (amount: BN) => void;
  tokenSymbol: string;
  availableAmount: BN;
  // decimals: number;
}

export function EditDepositDataModal({
  isOpen,
  onClose,
  onSubmit,
  tokenSymbol,
  availableAmount,
}: // decimals,
EditDepositDataModalProps) {
  const [amount, setAmount] = useState<string>("");

  if (!isOpen) return null;

  const handleSubmit = () => {
    try {
      const amountFloat = parseFloat(amount);
      if (isNaN(amountFloat) || amountFloat <= 0) {
        toast.error("Please enter a valid positive amount");
        return;
      }

      // Convert decimal amount to BN with proper decimals
      const amountBN = new BN(amountFloat);
      if (amountBN.gt(availableAmount)) {
        toast.error("Amount exceeds available balance");
        return;
      }

      onSubmit(amountBN);
    } catch (err) {
      toast.error("Please enter a valid number");
    }
  };

  const availableAmountFormatted = availableAmount
    ? (availableAmount.toNumber() / Math.pow(10, 9)).toFixed(9)
    : "0";

  return (
    <div className="modal modal-open">
      <div className="modal-box">
        <h3 className="font-bold text-lg">Deposit Token</h3>
        <div className="py-4 space-y-4">
          <div className="space-y-2">
            <p>
              <span className="font-medium">Token Symbol:</span>{" "}
              {ellipsify(tokenSymbol)}
            </p>
            <p>
              <span className="font-medium">Available Amount:</span>{" "}
              {availableAmountFormatted}
            </p>
            <div className="form-control">
              <label className="label">
                <span className="label-text">Amount to Deposit</span>
              </label>
              <input
                type="number"
                className="input input-bordered w-full"
                value={amount}
                onChange={(e) => setAmount(e.target.value)}
                placeholder={`Enter amount (max ${availableAmountFormatted})`}
                min="0"
              />
            </div>
          </div>
          <p className="text-sm opacity-70">
            This will deposit tokens back to your wallet. This action cannot be
            undone.
          </p>
        </div>
        <div className="modal-action">
          <button className="btn" onClick={onClose}>
            Cancel
          </button>
          <button className="btn btn-primary" onClick={handleSubmit}>
            Deposit
          </button>
        </div>
      </div>
    </div>
  );
}
