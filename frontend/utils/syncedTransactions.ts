import type { Details } from ".dfx/ic/canisters/backend/backend.did"
import { MILI_PER_SECOND } from "@/api/constants/ic"
import { getSyncedTransactions } from "@/api/user"
import type { syncedTransaction } from "@/types/sns"
import type { WalletTag } from "@/types/user"

//批量获取多个地址的交易记录
export const getAllSyncedTransactions = async (
  from_time: number,
  to_time: number,
  sort_method: string[],
  wallets: WalletTag[],
): Promise<{ total: number; transactions: syncedTransaction[] }> => {
  const addresses = wallets.map((walletTag) => walletTag.address)
  try {
    const res = await getSyncedTransactions(
      {
        from_time,
        to_time,
        sort_method,
        address: addresses,
      },
      true,
    )

    const transactions = res.map((transaction) => ({
      ...transaction,
      timestamp: Number(transaction.timestamp) / MILI_PER_SECOND,
    }))
    console.log("getSyncedTransactions", transactions)
    return { total: transactions.length, transactions: transactions }
  } catch (error) {
    console.error("Error fetching transactions:", error)
    throw error
  }
}

//获取每个交易记录对应的钱包名字
export const getTransactionWalletName = (
  type: string,
  detail: Details,
  wallets: WalletTag[],
): string => {
  let walletName = ""

  if (type === "SEND") {
    // 如果是发送交易，查找发送者地址对应的钱包名字
    const wallet = wallets.find((wallet) => wallet.address === detail.from)
    if (wallet) {
      walletName = wallet.name
    }
  } else if (type === "RECEIVE") {
    // 如果是接收交易，查找接收者地址对应的钱包名字
    const wallet = wallets.find((wallet) => wallet.address === detail.to)
    if (wallet) {
      walletName = wallet.name
    }
  }

  return walletName
}
