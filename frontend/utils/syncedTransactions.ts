import type {
  Details,
  TransactionB,
} from ".dfx/ic/canisters/backend/backend.did"
import { getSyncedTransactions } from "@/api/user"
import type { WalletTag } from "@/types/user"

//批量获取多个地址的交易记录
export const getAllSyncedTransactions = async (
  from_time: number,
  to_time: number,
  sort_method: string[],
  wallets: WalletTag[],
): Promise<any> => {
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
    console.log("getSyncedTransactions", res)
    const allTransactions: TransactionB[] = res.flatMap(
      (syncedHistory) => syncedHistory.history,
    )

    // 按照 timestamp 进行排序
    allTransactions.sort((a, b) => Number(a.timestamp) - Number(b.timestamp))
    console.log("getSyncedTransactions", allTransactions)
    return { total: allTransactions.length, transactions: allTransactions }
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
