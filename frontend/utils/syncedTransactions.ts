import type { TransactionB } from ".dfx/ic/canisters/backend/backend.did"
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
