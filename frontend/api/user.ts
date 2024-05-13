import type { TransactionF } from ".dfx/ic/canisters/backend/backend.did"
import type { Currency, SyncedTransaction } from "@/types/sns"
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
import { showMessageError } from "@/utils/message"
import { getNNS } from "@/utils/nns"
import { getStorage, getTokenList } from "@/utils/storage"
import moment from "moment"
import { getBackend, getCurrentPrincipal } from "./canister_pool"
import { getTransactionsICRC1 } from "./icrc1"
import { getICPTransactions } from "./rosetta"

//TODO demo阶段用户字段修改频繁，暂时用短缓存时间。
const userTTL = TTL.minute1 //用户自身信息缓存时长。
const walletTTL = TTL.day1 //用户自身信息缓存时长。

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
  const res = await getICPTransactions(wallet, true)
  transactions = res.transactions
  console.log("tokenList", tokenList)
  console.log("wallet", wallet)
  if (tokenList && wallet.principal) {
    for (let index = 0; index < tokenList.length; index++) {
      const token = tokenList[index]
      const currency: Currency = {
        decimals: token.decimals,
        symbol: token.symbol,
      }
      const icrcArray = await getTransactionsICRC1(
        wallet,
        token.canisters.index,
        token.canisters.ledger,
        currency,
      )
      console.log("icrcRes", currency, icrcArray)
      // 合并数组
      transactions = transactions.concat(icrcArray)
    }
  }
  return transactions.sort((a, b) => a.timestamp - b.timestamp)
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
): Promise<SyncedHistory> {
  return getBackend().query_wallets_synced_transactions(params)
  // return await getCache({
  //   key: "USER_SyncedTransactions",
  //   execute: () => getBackend().query_wallets_synced_transactions(params),
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
  const userConfig = getStorage("USER_CONFIG")
  if (userConfig && userConfig.timezone !== "") {
    moment.tz.setDefault(userConfig.timezone)
  }

  return userConfig
}
