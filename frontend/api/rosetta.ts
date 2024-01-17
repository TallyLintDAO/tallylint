import {
  LEDGER_CANISTER_ID,
  MILI_PER_SECOND,
  NET_ID,
  ROSETTA_URL,
} from "@/api/constants/ic"
import { matchICPPrice } from "@/api/token"
import type { Currency } from "@/types/sns"
import type { WalletHistory, WalletTag } from "@/types/user"
import { currencyCalculate } from "@/utils/common"
import { showMessageError } from "@/utils/message"

const radixNumber = 4 //保留4位小数

export interface InferredTransaction {
  hash: string
  timestamp: number
  type: string
  name: string
  details: {
    status: string
    fee: {
      amount: number
    }
    to?: string
    from?: string
    amount: number
    price: number // 发生交易时代币的单价
    currency: Currency
    ledgerCanisterId: string
    cost: number
    profit: number
    value: number
  }
}

export interface GetTransactionsResponse {
  total: number
  transactions: InferredTransaction[]
}

export const getICPTransactions = async (
  wallet: WalletTag,
  requireFormat: boolean,
): Promise<GetTransactionsResponse> => {
  //... 需要添加一个同地址缓存方法，以免调用过于频繁
  const response = await fetch(`${ROSETTA_URL}/search/transactions`, {
    method: "POST",
    body: JSON.stringify({
      network_identifier: NET_ID,
      account_identifier: {
        address: wallet.address,
      },
    }),
    headers: {
      "Content-Type": "application/json",
      Accept: "*/*",
    },
  })
  if (!response.ok) {
    showMessageError(
      "Address: " +
        wallet.address +
        " unable to get information from ICP Rosetta Api, " +
        "please check that the wallet address and network are correct.",
    )
    throw Error("error for rosetta api" + response.statusText)
  }

  const { transactions, total_count } = await response.json()
  // console.log("rosetta api:", transactions)
  purchaseQueue.length = 0 //计算前先重置购买队列数组，防止出现问题。
  const transactionsInfo: InferredTransaction[] = []
  //是否需要处理，不需要则不处理
  if (requireFormat) {
    //由于是时间最新的排前，所以要倒序数组，以实现先入先出的税务计算方式
    transactions.reverse()
    for (const { transaction } of transactions) {
      const formattedTransaction = await formatIcpTransaccion(
        wallet,
        transaction,
      )
      transactionsInfo.push(formattedTransaction)
    }
    //将数组恢复正常。
    transactionsInfo.reverse()
  }
  return {
    total: total_count,
    transactions: transactionsInfo,
  }
}

//批量获取多个地址的交易记录
export const getAllTransactions = async (
  wallets: WalletTag[],
): Promise<GetTransactionsResponse> => {
  try {
    // 使用 Promise.all 并行地获取多个钱包的交易记录
    const transactionsPromises = wallets.map((wallet) =>
      getICPTransactions(wallet, true),
    )
    const transactionsResults = await Promise.all(transactionsPromises)
    // 使用 Array.reduce 将所有 total 相加，并将 transactions 拼接在一起
    const response: GetTransactionsResponse = transactionsResults.reduce(
      (acc, curr) => {
        return {
          total: acc.total + curr.total,
          transactions: acc.transactions.concat(curr.transactions),
        }
      },
      { total: 0, transactions: [] },
    )

    return response
  } catch (error) {
    console.error("Error fetching transactions:", error)
    throw error
  }
}

interface Operation {
  account: {
    address: string
  }
  amount: {
    value: string
    currency: {
      symbol: string
      decimals: number
    }
  }
  status: "COMPLETED" | "REVERTED" | "PENDING"
  type: "TRANSACTION" | "FEE"
}

interface RosettaTransaction {
  metadata: {
    block_height: number
    memo: number
    timestamp: number
    lockTime: number
  }
  operations: Operation[]
  transaction_identifier: { hash: string }
}

export const formatIcpTransaccion = async (
  wallet: WalletTag,
  rosettaTransaction: RosettaTransaction,
): Promise<InferredTransaction> => {
  const {
    operations,
    metadata: { timestamp },
    transaction_identifier: { hash },
  } = rosettaTransaction
  const transaction: any = { details: { status: "COMPLETED", fee: {} } }
  const timestampNormal = timestamp / MILI_PER_SECOND //处理时间戳为正常格式
  const price = await matchICPPrice(timestampNormal) // 使用 await 获取价格
  operations.forEach((operation) => {
    const value = BigInt(operation.amount.value)
    const amount = value.toString()
    if (operation.type === "FEE") {
      //直接输出真实的数量，不再使用浮点数
      transaction.details.fee.amount = currencyCalculate(
        amount,
        operation.amount.currency.decimals,
      )
      // transaction.details.fee.currency = operation.amount.currency
      return
    }

    if (value >= 0) transaction.details.to = operation.account.address
    if (value <= 0) transaction.details.from = operation.account.address

    if (
      transaction.details.status === "COMPLETED" &&
      operation.status !== "COMPLETED"
    )
      transaction.details.status = operation.status

    transaction.type =
      transaction.details.to === wallet.address ? "RECEIVE" : "SEND"
    transaction.name = wallet.name
    //直接输出真实的数量，不再使用浮点数
    transaction.details.amount = currencyCalculate(
      amount,
      operation.amount.currency.decimals,
    )
    transaction.details.price = price // 设置价格为获取的价格
    transaction.details.value = parseFloat(
      (transaction.details.amount * transaction.details.price).toFixed(
        radixNumber,
      ),
    ) //计算总价值
    transaction.details.currency = operation.amount.currency
    transaction.details.ledgerCanisterId = LEDGER_CANISTER_ID

    //先入先出的成本计算法，以IC的精度，建议保留4位小数
    const cost = calculateCost(transaction)
    transaction.details.cost = parseFloat(cost.toFixed(radixNumber))
    if (transaction.type === "RECEIVE") {
      transaction.details.profit = 0
    } else if (transaction.type === "SEND") {
      // const factor = 10 ** radixNumber; //进位10的n次方，扩大倍数将其变成整数，再在计算完成后除以倍数换回小数点
      //TODO 本意是计算精度更准确，但有点bug，先注释了，用简单粗暴的
      // transaction.details.profit =
      //     (transaction.details.value * factor
      //         - transaction.details.cost * factor) / factor;
      transaction.details.profit = (
        transaction.details.value - transaction.details.cost
      ).toFixed(radixNumber)
    }
  })
  return {
    ...transaction,
    hash,
    timestamp: timestampNormal,
  } as InferredTransaction
}

const purchaseQueue: any[] = []

// 计算FIFO成本
const calculateCost = (transaction: InferredTransaction): number => {
  if (transaction.type === "RECEIVE") {
    // 处理接收交易，保存价格和数量。
    const { price, amount } = transaction.details
    purchaseQueue.push({ price, amount })
    return 0
  } else if (transaction.type === "SEND") {
    let cost = 0
    let sendAmount = transaction.details.amount // 存储本次交易发送的代币数量

    while (sendAmount > 0 && purchaseQueue.length > 0) {
      // 从最早购买的交易开始卖出
      const earliestPurchase = purchaseQueue[0]

      if (earliestPurchase.amount <= sendAmount) {
        // 如果购买数量小于等于发送数量，则完全卖出该购买交易
        cost += earliestPurchase.price * earliestPurchase.amount
        sendAmount -= earliestPurchase.amount
        purchaseQueue.shift() // 从队列中移除已卖出的交易
      } else {
        // 如果购买数量大于发送数量，则部分卖出该购买交易
        cost += earliestPurchase.price * sendAmount
        earliestPurchase.amount -= sendAmount
        sendAmount = 0
      }
    }

    return cost
  } else {
    return 0
  }
}

// 钱包历史对象
const initialWalletHistory = {
  accountAddress: "", // 用户账户ID
  amount: 0, // 初始余额为0
  history: [] as Array<WalletHistory>, // 交易历史记录
}

//根据交易历史，手动生成钱包历史
export const getWalletHistory = async (accountAddress: string) => {
  const res = await getICPTransactions(
    { address: accountAddress, name: "", from: "" },
    true,
  )
  // 倒序交易数组，以确保最早的交易在前面
  const transactions = res.transactions.reverse()
  // 初始化钱包历史
  let walletHistory = { ...initialWalletHistory, accountAddress }
  // 清空历史记录，以保证每次数组都重置
  walletHistory.history = []
  // 遍历每一笔交易并更新钱包历史
  for (const transaction of transactions) {
    // 解构交易信息
    const { details, timestamp, type } = transaction
    const { price, amount } = details

    // 更新钱包余额
    if (type === "SEND") {
      walletHistory.amount -= amount
    } else if (type === "RECEIVE") {
      walletHistory.amount += amount
    }
    // 计算交易金额
    const walletValue = parseFloat(
      (price * walletHistory.amount).toFixed(radixNumber),
    )
    // 创建交易历史记录对象
    const transactionRecord = {
      price,
      amount,
      walletAmount: walletHistory.amount,
      timestamp,
      walletValue,
      type,
    }

    // 将交易历史记录添加到钱包历史
    walletHistory.history.push(transactionRecord)
  }
  return walletHistory
}

interface Balance {
  value: string
  decimals: number
  error?: string
}

export const getICPBalance = async (accountId: string): Promise<number> => {
  const response = await fetch(`${ROSETTA_URL}/account/balance`, {
    method: "POST",
    body: JSON.stringify({
      network_identifier: NET_ID,
      account_identifier: {
        address: accountId,
      },
    }),
    headers: {
      "Content-Type": "application/json",
      Accept: "*/*",
    },
  })
  if (!response.ok) {
    console.error("getICPBalance Error: " + response.statusText)
    showMessageError("getICPBalance Error: " + response.statusText)
    return 0
  }
  const { balances } = await response.json()
  const [{ value, currency }] = balances
  return currencyCalculate(value, currency.decimals)
}
