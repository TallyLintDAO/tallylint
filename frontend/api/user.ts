import type { TransactionF } from ".dfx/ic/canisters/backend/backend.did"
import type { Currency, SyncedTransaction } from "@/types/tokens"
import type { ApiResult, ApiUserInfo } from "@/types/types"
import type {
  HistoryQueryParams,
  SyncedHistory,
  TaxReportData,
  UserConfig,
  WalletInfo,
  WalletTag,
  syncWalletParam,
} from "@/types/user"
import { TTL, getCache } from "@/utils/cache"
import { setCurrencyCode } from "@/utils/currencies"
import { showMessageError } from "@/utils/message"
import { getNNS } from "@/utils/nns"
import { getStorage, getTokenList, setStorage } from "@/utils/storage"
import moment from "moment"
import { getBackend, getCurrentPrincipal } from "./canister_pool"
import { getICPTransactions } from "./icp"
import { getTransactionsICRC1 } from "./icrc1"

const userTTL = TTL.hour12 //用户自身信息缓存时长。
const walletTTL = TTL.day1 //用户钱包信息缓存时长。

// （后端自动注册）并登录，如果有注册，就获取当前登录用户信息，如果没注册，就注册完了再获取信息
export async function getUserAutoRegister(): Promise<ApiResult<ApiUserInfo>> {
  return await getCache({
    key: "USER_INFO_" + getCurrentPrincipal().toUpperCase(),
    execute: () => getBackend().auto_register_user(),
    ttl: userTTL,
    isLocal: true, // 需要本地存储
  })
}

// 增加用户的钱包地址
export async function addUserWallet(
  address: string,
  name: string,
  from: string,
  principal?: string[],
): Promise<ApiResult<boolean>> {
  return getBackend().add_wallet({
    address: address,
    name: name,
    from: from,
    principal_id: principal,
  })
}

// 获得当前用户登记的钱包信息
export async function getUserWallet(
  refresh: boolean,
): Promise<ApiResult<WalletInfo[]>> {
  return await getCache({
    key: "USER_WALLET",
    execute: () => getBackend().query_all_wallets(),
    ttl: walletTTL,
    // isLocal: true, //使用内存储存就够了
    refresh: refresh, //是否刷新缓存，用于执行增删改操作后的刷新。
  })
}

// 获得当前用户登记的钱包信息
export async function getUserAllWallets(): Promise<WalletTag[]> {
  //TODO 好像用不上这方法了
  try {
    const [userWallets, neuronWallets, nnsWallets] = await Promise.all([
      getUserWallet(false),
      getUserNeuron(false),
      getNNS(),
    ])

    if (userWallets.Ok && neuronWallets.Ok) {
      const mapToWallet = (wallet: {
        id: bigint
        name: string
        address: string
        principal_id: string[]
        from: string
        last_sync_time: number
      }) => ({
        id: Number(wallet.id),
        name: wallet.name,
        address: wallet.address,
        principal: wallet.principal_id,
        from: wallet.from,
        last_sync_time: wallet.last_sync_time,
      })

      const userWalletList = userWallets.Ok.map(mapToWallet)
      const neuronWalletList = neuronWallets.Ok.map(mapToWallet)
      const nnsWalletList = nnsWallets.map((wallet, index) => ({
        id: 0,
        name: "hotkey " + (index + 1),
        address: wallet.address,
        principal: [],
        from: "hotkey",
      }))

      return [...userWalletList, ...neuronWalletList, ...nnsWalletList]
    } else {
      throw new Error("Failed to fetch user wallets or neuron wallets")
    }
  } catch (error) {
    showMessageError(String(error))
    throw error
  }
}

// 获得当前用户登记的钱包信息
export async function getUserWalletsTag(): Promise<WalletTag[]> {
  try {
    const userWallets = await getUserWallet(false)
    if (userWallets.Ok) {
      const mapToWallet = (wallet: {
        id: bigint
        name: string
        address: string
        principal_id: string[]
        from: string
        last_sync_time: number
      }) => ({
        id: Number(wallet.id),
        name: wallet.name,
        address: wallet.address,
        principal: wallet.principal_id,
        from: wallet.from,
        last_sync_time: wallet.last_sync_time,
      })
      const userWalletList = userWallets.Ok.map(mapToWallet)

      return userWalletList
    } else {
      throw new Error("Failed to fetch user wallets wallets")
    }
  } catch (error) {
    showMessageError(String(error))
    throw error
  }
}

// 编辑用户钱包
export async function editUserWallet(
  walletId: bigint,
  from: string,
  name: string,
): Promise<ApiResult<boolean>> {
  return getBackend().update_wallet({
    id: walletId,
    from: from,
    name: name,
  })
}

// 删除用户钱包
export async function deleteUserWallet(
  walletId: bigint,
): Promise<ApiResult<boolean>> {
  return getBackend().delete_wallet(walletId)
}

// 增加用户神经元的钱包地址
export async function addUserNeuron(
  address: string,
  name: string,
): Promise<ApiResult<boolean>> {
  return getBackend().add_neuron_wallet({
    address: address,
    name: name,
    from: "NNS",
    principal_id: [],
  })
}

// 编辑用户神经元钱包
export async function editUserNeuron(
  walletId: bigint,
  name: string,
): Promise<ApiResult<boolean>> {
  return getBackend().update_neuron_wallet({
    id: walletId,
    from: "NNS",
    name: name,
  })
}

// 删除用户神经元钱包
export async function deleteUserNeuron(
  walletId: bigint,
): Promise<ApiResult<boolean>> {
  return getBackend().delete_neuron_wallet(walletId)
}

// 查询用户神经元的钱包地址
export async function getUserNeuron(
  refresh: boolean,
): Promise<ApiResult<WalletInfo[]>> {
  // return getBackend().query_all_neuron_wallet()
  return await getCache({
    key: "USER_Neurons",
    execute: () => getBackend().query_all_neuron_wallet(),
    ttl: walletTTL,
    // isLocal: true, //使用内存储存就够了
    refresh: refresh, //是否刷新缓存，用于执行增删改操作后的刷新。
  })
}

// 获取单个钱包的所有待同步的交易记录，包括ICRC1 Token的和ICP的交易记录。
export async function fetchAllSyncTransactions(
  wallet: WalletTag,
): Promise<TransactionF[]> {
  const tokenList = getTokenList()
  let transactions: TransactionF[] = []
  const res = await getICPTransactions(wallet)
  transactions = res.transactions
  // console.log("getICPTransactions", res)
  if (tokenList && wallet.principal[0]) {
    const noICPTokenList = tokenList.filter((token) => token.symbol !== "ICP")
    // 使用 Promise.all 并行获取所有 ICRC1 交易记录
    const icrcResults = await Promise.all(
      noICPTokenList.map(async (token) => {
        const currency: Currency = {
          decimals: token.decimals,
          symbol: token.symbol,
        }
        return getTransactionsICRC1(
          wallet,
          token.canisters.index,
          token.canisters.ledger,
          currency,
        )
      }),
    )

    // 合并所有 ICRC1 交易记录
    icrcResults.forEach((icrcArray) => {
      transactions = transactions.concat(icrcArray)
    })
  }
  return transactions.sort((a, b) => b.timestamp - a.timestamp) //最新的数据排在排在数组的最前面 index0
}

// 同步钱包交易记录到后端
export async function syncWallet(
  walletTransactionHistoryArray: syncWalletParam[],
): Promise<ApiResult<boolean>> {
  return getBackend().sync_transaction_record(walletTransactionHistoryArray)
}

// 手动添加单条交易记录
export async function addManualTransaction(
  transaction: SyncedTransaction,
): Promise<ApiResult<boolean>> {
  return getBackend().add_transaction(transaction)
}

// 编辑单条交易记录
export async function editUserTransaction(
  transaction: SyncedTransaction,
): Promise<ApiResult<boolean>> {
  return getBackend().update_transaction(transaction)
}

// 单独设置用户交易记录的tag
export async function setTransactionTag(
  id: bigint | number,
  tag: string,
): Promise<ApiResult<TaxReportData>> {
  return getBackend().update_transaction_tag(id, tag)
}

// 删除用户交易记录的tag
export async function removeTransactionTag(
  id: bigint | number,
): Promise<ApiResult<TaxReportData>> {
  return getBackend().remove_transaction_tag(id)
}

// 查询用户已存储的交易记录
export async function getSyncedTransactions(
  params: HistoryQueryParams,
  refresh: boolean,
): Promise<ApiResult<SyncedHistory>> {
  return getBackend().query_synced_transactions(params)
  // return await getCache({
  //   key: "USER_SyncedTransactions",
  //   execute: () => getBackend().query_synced_transactions(params),
  //   ttl: walletTTL,
  //   refresh: refresh, //是否刷新缓存，用于执行增删改操作后的刷新。
  // })
}

// 删除用户已存储的交易记录
export async function deleteSyncedTransactions(
  transactionId: bigint | number,
): Promise<ApiResult<boolean>> {
  return getBackend().delete_transaction(transactionId)
}

// 获取用户税务报告的相关利润
export async function getUserTaxProfit(
  start: number,
  end: number,
): Promise<ApiResult<TaxReportData>> {
  return getBackend().my_summary(start, end)
}

export async function getUserConfig(): Promise<UserConfig | null> {
  //如果存在本地userconfig，则直接引用
  let userConfig = getStorage("USER_CONFIG")
  if (!userConfig) {
    //如果本地不存在配置类，则尝试读取后端canister中的userconfig
    const res = await getBackend().get_user_config()
    console.log("getBackend getUserConfig", res)
    if (res.Ok) {
      //读取完userconfig再将userconfig保存至本地，方便下次调用
      userConfig = res.Ok
    }
  }
  if (userConfig && userConfig.time_zone && userConfig.time_zone !== "") {
    console.log("userConfig.time_zone", userConfig.time_zone)
    //根据用户设置的时区设置默认时区
    moment.tz.setDefault(userConfig.time_zone)
  } else {
    //后端生成的默认配置会将time_zone设置为空，获取到时区以后，如果时区为空，自动将moment.tz的时区设置为对应的时区
    userConfig.time_zone = moment.tz.guess()
  }
  console.log("end getUserconfig", userConfig)
  setCurrencyCode(userConfig.base_currency)
  setStorage("USER_CONFIG", userConfig)
  return userConfig
}

export async function setUserConfig(userConfig: UserConfig): Promise<any> {
  return getBackend().set_user_config(userConfig)
}
