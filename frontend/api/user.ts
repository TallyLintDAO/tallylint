import type { ApiResult, ApiUserInfo } from "@/types/types"
import type {
  HistoryQueryParams,
  WalletInfo,
  WalletTag,
  syncWalletParam,
} from "@/types/user"
import { TTL, getCache } from "@/utils/cache"
import { showMessageError } from "@/utils/message"
import { getNNS } from "@/utils/nns"
import { getBackend, getCurrentPrincipal } from "./canister_pool"

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
  try {
    const [userWallets, neuronWallets, nnsWallets] = await Promise.all([
      getUserWallet(false),
      getUserNeuron(false),
      getNNS(),
    ])

    if (userWallets.Ok && neuronWallets.Ok) {
      const mapToWallet = (wallet: { name: any; address: any; from: any }) => ({
        name: wallet.name,
        address: wallet.address,
        from: wallet.from,
      })

      const userWalletList = userWallets.Ok.map(mapToWallet)
      const neuronWalletList = neuronWallets.Ok.map(mapToWallet)
      const nnsWalletList = nnsWallets.map((wallet, index) => ({
        name: "hotkey " + (index + 1),
        address: wallet.address,
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

// 同步钱包交易记录到后端
export async function syncWallet(
  walletTransactionHistoryArray: syncWalletParam[],
): Promise<ApiResult<boolean>> {
  return getBackend().sync_transaction_record(walletTransactionHistoryArray)
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

// 查询用户已存储的交易记录
export async function getSyncedTransactions(
  params: HistoryQueryParams,
  refresh: boolean,
): Promise<ApiResult<any>> {
  return await getCache({
    key: "USER_SyncedTransactions",
    execute: () => getBackend().wallet_history(params),
    ttl: walletTTL,
    refresh: refresh, //是否刷新缓存，用于执行增删改操作后的刷新。
  })
}
