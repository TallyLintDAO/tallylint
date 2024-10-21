import { TOKENS } from "@/api/constants/tokens"
import type { ICRC1Info } from "@/types/tokens"
import type { UserInfo } from "@/types/user"

//通用存储方法
export const setStorage = (key: string, value: any) => {
  try {
    // 检查 key 是否为字符串
    if (typeof key !== "string" || key.trim() === "") {
      throw new Error("Storage key must be a non-empty string")
    }
    // 检查 value 是否是可以序列化的有效类型
    if (typeof value === "undefined") {
      throw new Error("Cannot store undefined value")
    }

    localStorage.setItem(key, JSON.stringify(value))
  } catch (error) {
    console.error("Error setting storage:", error)
  }
}
//通用读取存储方法
export const getStorage = (key: string): any | null => {
  const value = localStorage.getItem(key)
  if (null == value) return null
  try {
    return JSON.parse(value)
  } catch (e) {
    console.error(`read ${key} info failed:`, e)
  }
  return null
}

// 本地保存用户信息，没有网络访问时也可以显示
export const setUserInfoStorage = (user: UserInfo): void => {
  if (user.owner !== "") {
    localStorage.setItem(
      `USER_${user.owner.toUpperCase()}`,
      JSON.stringify(user),
    )
  }
}
// get方法注意缓存清没清
export const getUserInfoStorage = (principal: string): UserInfo | null => {
  const info = localStorage.getItem(`USER_${principal.toUpperCase()}`)
  if (null == info) return null
  try {
    const read = JSON.parse(info) as UserInfo
    return read
  } catch (e) {
    console.error(`read user ${principal} info failed:`, e)
  }
  return null
}

export const deleteUserInfoStorage = (principal: string): void => {
  localStorage.removeItem(`USER_${principal.toUpperCase()}`)
}

// 本地保存代币列表，方便用户调用
export const setTokenList = (tokens: ICRC1Info[]): void => {
  if (tokens) {
    localStorage.setItem(`USER_TOKEN_LIST`, JSON.stringify(tokens))
  }
}

// 获取用户保留在本地的同步代币列表
export const getTokenList = (): ICRC1Info[] | null => {
  initICPTokenCache()
  const info = localStorage.getItem(`USER_TOKEN_LIST`)
  if (null == info) return null
  try {
    const data = JSON.parse(info) as ICRC1Info[]

    return data
  } catch (e) {
    console.error(`read user token list failed:`, e)
  }
  return null
}

//去除ICP的token list。
export const getTokenListWithoutICP = (): ICRC1Info[] => {
  initICPTokenCache()
  const info = localStorage.getItem(`USER_TOKEN_LIST`)
  if (null == info) return []
  try {
    const data = JSON.parse(info) as ICRC1Info[]
    return data.filter((token) => token.symbol !== "ICP")
  } catch (e) {
    console.error(`read user token list failed:`, e)
  }
  return []
}

//默认储存ICP作为tokenlist的第一位
const initICPTokenCache = () => {
  const info = localStorage.getItem(`USER_TOKEN_LIST`)

  let tokens: ICRC1Info[] = []
  if (null !== info) tokens = JSON.parse(info) as ICRC1Info[]

  const hasICP = tokens.some((token) => token.symbol === "ICP")
  // 如果ICP不存在，添加代币 ICP 到列表中
  if (!hasICP) {
    tokens.unshift(TOKENS.ICP)
    setTokenList(tokens)
  }
}
