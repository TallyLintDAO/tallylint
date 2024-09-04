import {
  IC_LEDGER_URL,
  LEDGER_CANISTER_ID,
  MILI_PER_SECOND,
  NET_ID,
  ROSETTA_URL,
} from "@/api/constants/ic"
import { matchICPPrice } from "@/api/token"
import type { InferredTransaction, LedgerICPTransaction } from "@/types/tokens"
import type {
  DailyBalance,
  WalletHistory,
  WalletInfo,
  WalletTag,
} from "@/types/user"
import { currencyCalculate } from "@/utils/common"
import { showMessageError } from "@/utils/message"
import axios from "axios"
import { matchICRC1Price } from "./icrc1"
import { fetchAllSyncTransactions } from "./user"

const radixNumber = 4 //保留4位小数

export interface GetTransactionsResponse {
  total: number
  transactions: InferredTransaction[]
}

// https://ledger-api.internetcomputer.org/swagger-ui/#/Accounts/get_account_transactions
// 使用新版本的api节点请求，原先的rosetta节点已经被废弃
export const getICPTransactions = async (
  wallet: WalletTag,
): Promise<GetTransactionsResponse> => {
  const url = `${IC_LEDGER_URL}/accounts/${wallet.address}/transactions`
  let offset = 0
  const limit = 100
  let total = 0
  let allTransactions: any[] = []
  //因为一次只能请求100个记录，所以分页请求所有记录。
  do {
    const params = {
      limit,
      offset,
    }
    try {
      const response = await axios.get(url, { params })
      const data = response.data
      total = data.total
      allTransactions = allTransactions.concat(data.blocks)
      offset += limit
    } catch (error) {
      console.error("Error fetching transactions:", error)
      break
    }
  } while (offset < total)

  // console.log("ledger get: ", allTransactions)
  const transactionsInfo = await convertToTransactionF(wallet, allTransactions)
  return {
    total: transactionsInfo.length,
    transactions: transactionsInfo,
  }
}

export const convertToTransactionF = async (
  wallet: WalletTag,
  transactions: LedgerICPTransaction[],
): Promise<InferredTransaction[]> => {
  const currency = { decimals: 8, symbol: "ICP" }
  const result: InferredTransaction[] = [] // 用于存储处理后的交易
  for (const item of transactions) {
    // 排除 transfer_type 不为send 的数据，说明可能是approve，目前不需要统计和存储这种数据
    if (item.transfer_type !== "send") {
      continue
    }
    // api接口给的时间戳是10位，精确到秒，原先的是13位，精确到毫秒，所以这里加3位。
    const timestampNormal = item.created_at * 1000 //处理时间戳为正常格式
    const price = await matchICPPrice(timestampNormal) // 使用 await 获取价格
    const transaction = {
      wid: BigInt(wallet.id),
      hash: item.transaction_hash,
      t_type:
        item.to_account_identifier === wallet.address ? "RECEIVE" : "SEND",
      timestamp: timestampNormal,
      details: {
        to: item.to_account_identifier,
        status: "COMPLETED",
        ledgerCanisterId: LEDGER_CANISTER_ID,
        from: item.from_account_identifier,
        currency: currency,
        fee: Math.abs(currencyCalculate(item.fee, currency.decimals)),
        amount: currencyCalculate(item.amount, currency.decimals),
        cost: 0, // cost由后端计算
        profit: 0, // profit由后端计算
        price: price,
        value: 0, // value由前端，马上在下面代码中计算
      },
    }
    //计算价值
    transaction.details.value = parseFloat(
      (transaction.details.amount * transaction.details.price).toFixed(
        radixNumber,
      ),
    )
    //先入先出的成本计算法，以IC的精度，建议保留4位小数
    const cost = calculateCost(transaction)
    transaction.details.cost = parseFloat(cost.toFixed(radixNumber))
    if (transaction.t_type === "RECEIVE") {
      transaction.details.profit = 0
    } else if (transaction.t_type === "SEND") {
      // const factor = 10 ** radixNumber; //进位10的n次方，扩大倍数将其变成整数，再在计算完成后除以倍数换回小数点
      //TODO 本意是计算精度更准确，但有点bug，先注释了，用简单粗暴的
      // transaction.details.profit =
      //     (transaction.details.value * factor
      //         - transaction.details.cost * factor) / factor;
      transaction.details.profit = Number(
        (transaction.details.value - transaction.details.cost).toFixed(
          radixNumber,
        ),
      )
    }
    result.push(transaction) // 将处理后的交易添加到结果数组中
  }
  console.log("result", result)
  return result // 返回结果数组
}

//批量获取多个地址的交易记录
export const getAllTransactions = async (
  wallets: WalletTag[],
): Promise<GetTransactionsResponse> => {
  try {
    // 使用 Promise.all 并行地获取多个钱包的交易记录
    const transactionsPromises = wallets.map((wallet) =>
      getICPTransactions(wallet),
    )
    const transactionsResults = await Promise.all(transactionsPromises)
    console.log("transactionsResults", transactionsResults)
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
    response.transactions.sort((a, b) => b.timestamp - a.timestamp)

    return response
  } catch (error) {
    console.error("Error fetching transactions:", error)
    throw error
  }
}

const purchaseQueue: any[] = []

// 计算FIFO成本
const calculateCost = (transaction: InferredTransaction): number => {
  if (transaction.t_type === "RECEIVE") {
    // 处理接收交易，保存价格和数量。
    const { price, amount } = transaction.details
    purchaseQueue.push({ price, amount })
    return 0
  } else if (transaction.t_type === "SEND") {
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
  const res = await getICPTransactions({
    id: 0,
    address: accountAddress,
    principal: [],
    name: "",
    from: "",
  })
  // 倒序交易数组，以确保最早的交易在前面
  const transactions = res.transactions.reverse()
  // 初始化钱包历史
  let walletHistory = { ...initialWalletHistory, accountAddress }
  // 清空历史记录，以保证每次数组都重置
  walletHistory.history = []
  // 遍历每一笔交易并更新钱包历史
  for (const transaction of transactions) {
    // 解构交易信息
    const { details, timestamp, t_type } = transaction
    const { price, amount } = details

    // 更新钱包余额
    if (t_type === "SEND") {
      walletHistory.amount -= amount
    } else if (t_type === "RECEIVE") {
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
      t_type,
    }

    // 将交易历史记录添加到钱包历史
    walletHistory.history.push(transactionRecord)
  }
  return walletHistory
}

//根据交易历史，手动生成钱包历史，以完成echarts图表中，展示用户所拥有的钱包余额变化历史
export const getAllWalletDailyBalance = async (
  wallets: WalletInfo[],
): Promise<DailyBalance> => {
  //先获取所有历史记录，再统计钱包余额
  const walletPromises = wallets.map(async (wallet) => {
    const res = await fetchAllSyncTransactions({
      id: Number(wallet.id),
      address: wallet.address,
      principal: wallet.principal_id,
      name: wallet.name,
      from: wallet.from,
    })
    return res
  })
  // allWalletHistories 包含了所有钱包的历史记录数组
  const allWalletHistories: InferredTransaction[] = (
    await Promise.all(walletPromises)
  ).flat()
  let dailyBalance: DailyBalance = {}
  // 对所有历史记录按时间戳进行排序
  allWalletHistories.sort((a, b) => a.timestamp - b.timestamp)
  allWalletHistories.map(async (transaction, index) => {
    const {
      timestamp,
      details: {
        amount,
        currency: { symbol },
        ledgerCanisterId,
      },
      t_type,
    } = transaction
    const currentDate = new Date(timestamp)
    const currentDateStr = currentDate.toISOString().split("T")[0]

    // 初始化当天的余额记录
    if (!dailyBalance[currentDateStr]) {
      dailyBalance[currentDateStr] = {}
    }

    // 初始化代币余额
    if (!dailyBalance[currentDateStr][symbol]) {
      dailyBalance[currentDateStr][symbol] = {
        amount: 0,
        value: 0,
        ledgerCanisterId,
      }
    }
    // 根据交易类型更新代币余额
    if (t_type === "RECEIVE") {
      dailyBalance[currentDateStr][symbol].amount += amount
    } else {
      // 假设其他类型是支出，这里可以根据实际情况修改
      dailyBalance[currentDateStr][symbol].amount -= amount
    }

    // 填充每个交易记录之间的缺失每日日期
    const nextTransaction = allWalletHistories[index + 1]
    if (nextTransaction) {
      const nextDate = new Date(nextTransaction.timestamp)
      const daysDifference =
        (nextDate.getTime() - currentDate.getTime()) / (1000 * 60 * 60 * 24)

      dailyBalance[nextDate.toISOString().split("T")[0]] = JSON.parse(
        JSON.stringify(dailyBalance[currentDateStr]),
      )
      if (daysDifference > 1) {
        //如果交易记录之间间隔大于1天，则需要手动填充缺失的每天交易记录
        for (let i = 1; i < daysDifference; i++) {
          const missingTimestamp =
            currentDate.getTime() + i * 24 * 60 * 60 * 1000

          const missingDate = new Date(missingTimestamp)
          const missingDateString = missingDate.toISOString().split("T")[0]
          // 复制上一个交易日的代币余额，注意使用深拷贝
          dailyBalance[missingDateString] = JSON.parse(
            JSON.stringify(dailyBalance[currentDateStr]),
          )
        }
      }
    } else {
      // 进入else说明这里是用户的最后一笔交易。
      // 处理最后一笔交易到今天之间的余额
      const today = new Date()
      const daysDifference =
        (today.getTime() - currentDate.getTime()) / (1000 * 60 * 60 * 24)

      if (daysDifference > 0) {
        for (let i = 1; i <= daysDifference; i++) {
          const missingTimestamp =
            currentDate.getTime() + i * 24 * 60 * 60 * 1000
          const missingDate = new Date(missingTimestamp)
          const missingDateString = missingDate.toISOString().split("T")[0]
          // 复制最后一个交易日的代币余额，注意使用深拷贝
          dailyBalance[missingDateString] = JSON.parse(
            JSON.stringify(dailyBalance[currentDateStr]),
          )
        }
      }
    }
  })
  return dailyBalance
}

// 获取dailyBalance对象的所有值
export const getDailyBalanceValue = async (
  dailyBalance: DailyBalance,
): Promise<number[]> => {
  const dates = Object.keys(dailyBalance)
  let balances: number[] = []
  for (const day of dates) {
    const balanceInfo = dailyBalance[day]
    const date = new Date(day)
    //这一天的钱包价值
    let value = 0
    // 遍历每个代币
    const tokens = Object.keys(dailyBalance[day])
    // 将每个代币的价值添加进来
    for (const token of tokens) {
      let tokenPrice = 0
      //获取token 单价
      if (token === "ICP") {
        tokenPrice = await matchICPPrice(date.getTime())
      } else {
        //ICRC1 token
        tokenPrice = await matchICRC1Price(
          date.getTime(),
          balanceInfo[token].ledgerCanisterId,
        )
      }
      //计算价值
      value += balanceInfo[token].amount * tokenPrice
    }
    balances.push(Number(value.toFixed(2)))
  }
  return balances
}

//获得当前account id所持有的icp balance
export const getICPBalance = async (accountId: string): Promise<number> => {
  //TODO rosetta已经被废弃，此接口可能出问题
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
