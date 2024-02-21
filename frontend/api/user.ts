import type { ApiResult, ApiUserInfo } from "@/types/types"
import type { WalletInfo } from "@/types/user"
import { TTL, getCache } from "@/utils/cache"
import { getBackend, getCurrentPrincipal } from "./canister_pool"
import type { InferredTransaction } from "@/types/sns"

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
  walletId: bigint,
  history: InferredTransaction[],
): Promise<ApiResult<boolean>> {
  return getBackend().sync_transaction_record({ walletId, history })
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
