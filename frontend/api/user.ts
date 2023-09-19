import { Principal } from "@dfinity/principal/lib/cjs";

import { getCache, TTL } from '@/utils/cache';
import { getCurrentPrincipal, getBackend } from './canister_pool';
import type { ApiResult, ApiUserInfo } from "@/types/types";

//TODO demo阶段用户字段修改频繁，暂时用短缓存时间。
const userTTL = TTL.minute1; //用户自身信息缓存时长。

// （后端自动注册）并登录，如果有注册，就获取当前登录用户信息，如果没注册，就注册完了再获取信息
export async function getUserAutoRegister(): Promise<ApiResult<ApiUserInfo>> {
    return await getCache({
        key: 'USER_INFO_' + getCurrentPrincipal().toUpperCase(),
        execute: () => getBackend().auto_register_user(),
        ttl: userTTL,
        isLocal: false, // 需要本地存储
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
export async function getUserWallet(): Promise<ApiResult<any>> {
    return getBackend().query_all_wallets();
}

// 删除用户钱包
export async function deleteUserWallet(walletId: number): Promise<ApiResult<any>> {
    return getBackend().delete_wallet(walletId);
}
