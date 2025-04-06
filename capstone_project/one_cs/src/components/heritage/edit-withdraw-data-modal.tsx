import toast from "react-hot-toast";

import { BN } from "@coral-xyz/anchor";
import { PublicKey } from "@solana/web3.js";
import { useWallet } from "@solana/wallet-adapter-react";

import { useOneCsProgramAccount } from "./one_cs-data-access";
import { ellipsify } from "../ui/ui-layout";

interface UseEditWithdrawDataProps {
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

export function useEditWithdrawData({
  account,
  data,
  onSuccess,
}: UseEditWithdrawDataProps) {
  const { editWithdrawTokenData } = useOneCsProgramAccount({ account });
  const { publicKey } = useWallet();

  const handleEditWithdrawData = async () => {
    try {
      if (!data?.data.token || !publicKey) {
        toast.error(
          publicKey ? "No token data found" : "Please connect your wallet"
        );
        return;
      }

      editWithdrawTokenData.mutateAsync({
        label: data?.label || "",
        creator: data?.creator,
        payer: publicKey,
        owner: data?.owner,
        taker: publicKey,
        amount: data.data.token.tokenAmount,
        tokenMint: data.data.token.tokenMint,
      });
      onSuccess();
    } catch (err) {
      toast.error("Error withdrawing tokens: " + err);
    }
  };

  return {
    handleEditWithdrawData,
    isEditWithdrawPending: editWithdrawTokenData.isPending,
  };
}

interface EditWithdrawDataModalProps {
  isOpen: boolean;
  onClose: () => void;
  onSubmit: () => void;
  tokenAmount: BN;
  tokenMint: PublicKey;
  decimals: number;
}

export function EditWithdrawDataModal({
  isOpen,
  onClose,
  onSubmit,
  tokenAmount,
  tokenMint,
  decimals,
}: EditWithdrawDataModalProps) {
  if (!isOpen) return null;

  const formatAmount = (amount: BN) => {
    const rawAmount = amount.toNumber();
    if (rawAmount === 0) return "0";

    const amountWithDecimals = (rawAmount / Math.pow(10, decimals)).toString();
    return amountWithDecimals;
  };

  const formattedAmount = formatAmount(tokenAmount);

  return (
    <div className="modal modal-open">
      <div className="modal-box">
        <h3 className="font-bold text-lg">Withdraw Token</h3>
        <div className="py-4 space-y-4">
          <div className="space-y-2">
            <p>
              <span className="font-medium">Token Mint:</span>{" "}
              {ellipsify(tokenMint.toString())}
            </p>
            <p>
              <span className="font-medium">Amount to Withdraw:</span>{" "}
              {formattedAmount}
            </p>
          </div>
          <p className="text-sm opacity-70">
            This will withdraw all tokens back to your wallet. This action
            cannot be undone.
          </p>
        </div>
        <div className="modal-action">
          <button className="btn" onClick={onClose}>
            Cancel
          </button>
          <button className="btn btn-primary" onClick={onSubmit}>
            Withdraw
          </button>
        </div>
      </div>
    </div>
  );
}
