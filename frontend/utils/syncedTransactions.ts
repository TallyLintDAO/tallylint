import type { Details } from ".dfx/ic/canisters/backend/backend.did"
import { getUserCurrencyRate } from "@/api/baseCurrencies"
import { getSyncedTransactions } from "@/api/user"
import type { SyncedTransaction } from "@/types/tokens"
import type { WalletTag } from "@/types/user"
import { processNumber } from "./math"

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
    if (res.Ok) {
      const transactions = res.Ok.map((transaction) => ({
        ...transaction,
        timestamp: Number(transaction.timestamp),
        details: {
          ...transaction.details,
          price: processNumber(transaction.details.price * rate),
          cost: processNumber(transaction.details.cost * rate),
          profit: processNumber(transaction.details.profit * rate),
          value: processNumber(transaction.details.value * rate),
        },
      }))
      return { total: transactions.length, transactions: transactions }
    } else {
      return { total: 0, transactions: [] }
    }
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
  //根据type是send还是receive，来搜索这条交易记录对应的用户钱包地址
  let walletAddress = type === "SEND" ? detail.from : detail.to
  const foundWallet = wallets.find((wallet) => {
    if (detail.currency.symbol === "ICP") {
      //通过account id匹配
      return wallet.address === walletAddress
    } else {
      //如果token不是ICP，则通过principal id匹配
      return wallet.principal?.[0] === walletAddress
    }
  })

  return foundWallet?.name || ""
}
