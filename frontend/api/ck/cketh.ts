import type { Currency, InferredTransaction } from "@/types/tokens"
import type { WalletTag } from "@/types/user"
import ic from "@/utils/icblast"
import { formatICRC1Transaction } from "../icrc1"

export const CKETH_LEDGER_CANISTER_ID = "ss2fx-dyaaa-aaaar-qacoq-cai"
export const CKETH_INDEX_CANISTER_ID = "s3zol-vqaaa-aaaar-qacpa-cai"

export const getTransactionsCK = async (
  wallet: WalletTag,
  indexCanisterId: string,
  ledgerCanisterId: string,
  currency: Currency,
): Promise<InferredTransaction[]> => {
  //返回值格式存在问题，报错了，具体字段看有道笔记
  const principalId = wallet.principal[0]
  let ICRCTransactions: InferredTransaction[] = []

  const can = await ic(CKETH_INDEX_CANISTER_ID)

  const transactions = await can.get_account_transactions({
    account: {
      owner: principalId,
    },
    max_results: BigInt(10000),
  })
  console.log("transactions", transactions)
  const transactionsInfo = transactions.transactions
  ICRCTransactions = await Promise.all(
    transactionsInfo.map((transaction) => {
      return formatICRC1Transaction(
        wallet,
        transaction,
        currency,
        CKETH_LEDGER_CANISTER_ID,
      )
    }),
  )
  return ICRCTransactions
}
