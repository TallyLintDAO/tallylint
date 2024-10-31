import type { Details } from ".dfx/ic/canisters/backend/backend.did"
import { initAuth } from "@/api/auth"
import { MILI_PER_SECOND } from "@/api/constants/ic"
import type {
  Currency,
  ICRC1BalanceResult,
  ICRC1Info,
  IRCR1Price,
  InferredTransaction,
} from "@/types/tokens"
import type { WalletInfo, WalletTag } from "@/types/user"
import { TTL, getCache } from "@/utils/cache"
import { currencyCalculate } from "@/utils/common"
import ic from "@/utils/icblast"
import { binarySearchClosestICRC1Price, processNumber } from "@/utils/math"
import { showMessageError } from "@/utils/message"
import { getTokenListWithoutICP } from "@/utils/storage"
import { HttpAgent } from "@dfinity/agent"
import { IcrcAccount, IcrcIndexCanister } from "@dfinity/ledger-icrc"
import type { TransactionWithId } from "@dfinity/ledger-icrc/dist/candid/icrc_index"
import { Principal } from "@dfinity/principal"

// Docs: https://github.com/dfinity/ic-js/tree/main/packages/ledger-icrc

const radixNumber = 4 //保留4位小数

//TODO 筛选IC用户除了ICP以外，任何还有balance的，尚未导入已同步代币列表的新代币。
export const queryIcUserNewAssetsListWithoutICP = async (
  tokenList: ICRC1Info[],
  wallets: WalletInfo[],
): Promise<ICRC1Info[]> => {
  if (!tokenList) {
    return []
  }
  const knownTokens = getTokenListWithoutICP() //用户已导入的代币列表
  const newTokens: ICRC1Info[] = [] //新的代币列表
  const promises = tokenList.map(async (token) => {
    try {
      const can = await ic(token.canisters.ledger)
      // Ledger Canister Method,
      // *icrc1_balance_of: (record { owner: principal; subaccount:opt vec nat8 }) → (nat) query
      // 遍历 wallets 数组，逐个获取余额
      for (const wallet of wallets) {
        if (!wallet.principal_id[0]) {
          console.warn(
            `Wallet ${wallet.name} does not have a valid principal ID`,
          )
          continue
        }

        const resBalance = await can.icrc1_balance_of({
          owner: Principal.fromText(wallet.principal_id[0]),
        })
        //转换balance为正常数字。
        const balance = currencyCalculate(resBalance, token.decimals)
        // 如果该钱包中的代币余额不为 0，且代币不在 knownTokens 列表中
        if (
          balance > 0 &&
          !knownTokens.some((knownToken) => knownToken.symbol === token.symbol)
        ) {
          newTokens.push(token)
          return // 已经找到有余额的钱包，可以跳过后续钱包
        }
      }
    } catch (e) {
      const error = e instanceof Error ? e : new Error(String(e))
      console.error(`${token.symbol} Error with Token Request:`, error)
      // showMessageError(`${token.symbol} Error with Token Request:` + error)
    }
  })
  await Promise.all(promises)
  return newTokens
}

export const getICRC1Balance = async (
  principalId: string,
): Promise<ICRC1BalanceResult[]> => {
  const tokenList = getTokenListWithoutICP()
  if (!tokenList) {
    return []
  }
  const promises = tokenList.map(async (token): Promise<ICRC1BalanceResult> => {
    try {
      const can = await ic(token.canisters.ledger)
      // Ledger Canister Method,
      // *icrc1_balance_of: (record { owner: principal; subaccount:opt vec nat8 }) → (nat) query
      const resBalance = await can.icrc1_balance_of({
        owner: Principal.fromText(principalId),
      })
      const balance = currencyCalculate(resBalance, token.decimals)
      const price = await matchICRC1Price(
        Date.now(),
        token.canisters.ledger,
        token.symbol,
      )
      const value = processNumber(balance * price)
      return {
        symbol: token.symbol,
        logo: token.meta.logo,
        balance,
        price,
        value,
      }
    } catch (e) {
      const error = e instanceof Error ? e : new Error(String(e))
      console.error(`${token.symbol} Error with Token Request:`, error)
      showMessageError(`${token.symbol} Error with Token Request:` + error)
      return {
        symbol: token.symbol,
        logo: token.meta.logo,
        balance: 0,
        price: 0,
        value: 0,
        error,
      }
    }
  })

  const tokens = await Promise.all(promises)
  return tokens
}

export const getTransactionsICRC1 = async (
  wallet: WalletTag,
  indexCanisterId: string,
  ledgerCanisterId: string,
  currency: Currency,
): Promise<InferredTransaction[]> => {
  const ai = await initAuth()
  let ICRCTransactions: InferredTransaction[] = []
  if (ai.info && wallet.principal) {
    const identity = ai.info.identity
    const agent = new HttpAgent({ identity })
    const principal = Principal.fromText(wallet.principal[0]) //用户address为account id，无法转换为principal。必须要输入principal id才行
    const canisterPrincipal = Principal.fromText(indexCanisterId)

    const { getTransactions: ICRC1_getTransactions } = IcrcIndexCanister.create(
      {
        agent: agent,
        canisterId: canisterPrincipal,
      },
    )
    const ICRC1getTransactions = await ICRC1_getTransactions({
      account: {
        owner: principal,
      } as IcrcAccount,
      max_results: BigInt(10000),
    })

    const transactionsInfo = ICRC1getTransactions.transactions
    ICRCTransactions = await Promise.all(
      transactionsInfo
        .filter(
          (transaction) =>
            transaction.transaction.kind !== "approve" &&
            transaction.transaction.kind !== "burn",
        ) // TODO kind == burn时貌似也应该记录进来，但暂时先一起判空粗暴点解决了
        // 直接过滤掉kind为approve和burn的交易
        .map((transaction) => {
          return formatICRC1Transaction(
            wallet,
            transaction,
            currency,
            ledgerCanisterId,
          )
        }),
    )
  }
  return ICRCTransactions
}

export const formatICRC1Transaction = async (
  wallet: WalletTag,
  transactionWithId: TransactionWithId,
  currency: Currency,
  ledgerCanisterId: string,
): Promise<InferredTransaction> => {
  const { transaction, id } = transactionWithId
  const timestampNormal = Number(transaction.timestamp) / MILI_PER_SECOND // 处理时间戳从19位ns改为常用的13位ms，直接除会有一些小数点。
  let detail = transaction[transaction.kind][0]
  delete detail.created_at_time
  let details: Details = detail
  details.amount = currencyCalculate(detail.amount, currency.decimals)
  details.to = detail.to.owner.toString()
  details.price = await matchICRC1Price(
    timestampNormal,
    ledgerCanisterId,
    currency.symbol,
  ) // 使用 await 获取价格
  details.cost = 0 // 置0，交给后端计算
  details.profit = 0 // 置0，交给后端计算
  details.value = parseFloat(
    (details.amount * details.price).toFixed(radixNumber),
  )
  details.currency = currency
  details.ledgerCanisterId = ledgerCanisterId
  details.status = "COMPLETED"
  if (transaction.kind === "transfer") {
    // icrc1的代币中的fee必定为自身代币。
    details.fee = currencyCalculate(details.fee[0], currency.decimals)
    details.from = detail.from.owner.toString()
  } else if (transaction.kind === "mint") {
    //mint没有fee，且没有from地址
    details.fee = 0
    details.from = "Minting Account"
    // details.tag = "mint"
  } else {
    //TODO kind == burn || approve 这两种类型还没有写
  }
  const t_type = details.to === wallet.principal[0] ? "RECEIVE" : "SEND"
  return {
    details,
    wid: BigInt(wallet.id),
    t_type,
    hash: id.toString(),
    timestamp: timestampNormal,
  }
}

export const getICRC1Price = async (
  ledgerCanisterId: string,
): Promise<IRCR1Price[]> => {
  //从icpswap记录罐子中获取存储罐子的id
  let recordStorageCanister = await ic("ggzvv-5qaaa-aaaag-qck7a-cai")
  // Chat ledger canister: 2ouva-viaaa-aaaaq-aaamq-cai
  // recordStorageCanister.tokenStorage: return : moe7a-tiaaa-aaaag-qclfq-cai
  let tokenStorage = await ic(
    await recordStorageCanister.tokenStorage(ledgerCanisterId),
  )
  // @param1 text tokenId token canister id
  // @param2 int time start time of charts, default is 0
  // @param3 int interval time interval of charts(seconds), 1 day is 24 * 60 * 60
  // @param4 nat limit charts data length
  let priceHistory = await tokenStorage.getTokenPricesData(
    ledgerCanisterId,
    0,
    0,
    1000,
  )
  // console.log("getICRC1Price", priceHistory)
  return priceHistory
}

export const matchICRC1Price = async (
  timestamp: number,
  ledgerCanisterId: string,
  symbol: string,
): Promise<number> => {
  //将小数点的时间戳转为整数时间戳
  const targetTimestamp = Math.floor(timestamp)
  //获取ICP的所有价格历史数据，并通过getCache保存到本地缓存中，ttl为1天，方便调用。
  try {
    const priceHistory = await getCache({
      key: "ICRC1_Price_History_" + ledgerCanisterId,
      execute: () => getICRC1Price(ledgerCanisterId),
      ttl: TTL.day1,
      isLocal: false,
    })
    // 返回最接近时间戳对应的币价，如果没有找到则返回 undefined
    const price = binarySearchClosestICRC1Price(
      priceHistory,
      Math.floor(targetTimestamp / 1000),
    ).open
    return processNumber(price)
  } catch (error) {
    showMessageError(`Failed to get price history for token ${symbol}`)
    console.error(`Failed to get price history for token ${symbol}:`, error)

    return NaN // 返回 NaN 表示获取价格失败
  }
}
