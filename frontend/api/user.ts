import { Principal } from "@dfinity/principal/lib/cjs";

import { getCache, TTL } from '@/utils/cache';
import { getCurrentPrincipal, getBackend } from './canister_pool';
import type { ApiResult, ApiUserInfo } from "@/types/types";
import type { WalletInfo } from "@/types/user";

//TODO demo阶段用户字段修改频繁，暂时用短缓存时间。
const userTTL = TTL.minute1; //用户自身信息缓存时长。
const walletTTL = TTL.day1; //用户自身信息缓存时长。

// （后端自动注册）并登录，如果有注册，就获取当前登录用户信息，如果没注册，就注册完了再获取信息
export async function getUserAutoRegister(): Promise<ApiResult<ApiUserInfo>> {
    return await getCache({
        key: 'USER_INFO_' + getCurrentPrincipal().toUpperCase(),
        execute: () => getBackend().auto_register_user(),
        ttl: userTTL,
        isLocal: true, // 需要本地存储
    });
}

// 增加用户的钱包地址
export async function addUserWallet(address, name, from): Promise<ApiResult<boolean>> {
    return getBackend().add_wallet({
        address: address,
        name: name,
        from: from
    })
}

// 获得当前用户登记的钱包信息
export async function getUserWallet(refresh: boolean): Promise<ApiResult<WalletInfo[]>> {
    return await getCache({
        key: 'USER_WALLET',
        execute: () => getBackend().query_all_wallets(),
        ttl: walletTTL,
        // isLocal: true, //TODO 是否需要本地存储还需考虑，理论上来说内存存储就足够了
        refresh: refresh //是否刷新缓存，用于执行增删改操作后的刷新。
    });
}

// 删除用户钱包
export async function deleteUserWallet(walletId: bigint): Promise<ApiResult<boolean>> {
    return getBackend().delete_wallet(walletId);
}
