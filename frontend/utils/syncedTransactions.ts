import type { Details } from ".dfx/ic/canisters/backend/backend.did"
import { getUserCurrencyRate } from "@/api/baseCurrencies"
import { getSyncedTransactions } from "@/api/user"
import type { SyncedTransaction } from "@/types/sns"
import type { WalletTag } from "@/types/user"
import { numberToFixed } from "./math"

//批量获取多个地址的交易记录
export const getAllSyncedTransactions = async (
  from_time: number,
  to_time: number,
  sort_method: string[],
  wallets: WalletTag[],
): Promise<{ total: number; transactions: SyncedTransaction[] }> => {
  const ids = wallets.map((walletTag) => walletTag.id)
  try {
    const res = await getSyncedTransactions(
      {
        from_time,
        to_time,
        sort_method,
        wids: ids,
      },
      true,
    )
    //如果获取汇率的过程中发生了错误或者返回的结果中没有汇率，则使用原值
    const rate = (await getUserCurrencyRate()).rate || 1
    const transactions = res.map((transaction) => ({
      ...transaction,
      timestamp: Number(transaction.timestamp),
      details: {
        ...transaction.details,
        price: numberToFixed(transaction.details.price * rate, 2),
        cost: numberToFixed(transaction.details.cost * rate, 2),
        profit: numberToFixed(transaction.details.profit * rate, 2),
        value: numberToFixed(transaction.details.value * rate, 2),
      },
    }))
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
