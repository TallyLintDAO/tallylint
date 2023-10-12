// 统一用户信息的结构
// 注意和types.ts中的ApiUserInfo统一格式
export class UserInfo {
  // id = 0; //用户id
  owner = "" // 用户principal，唯一
  name = "" // 用户自己设置的用户名
  create_at = 0 //注册时间
}

export interface UserInfoElement {
  owner?: string
  name?: string
}

export interface WalletInfo {
  address: string
  from: string //'NNS' | 'Plug' | 'Stoic' | 'AstorMe'
  name: string
  transactions: number
}
export interface WalletHistory {
  price: number
  amount: number
  walletAmount: number
  timestamp: number
  walletValue: number
  type: string
}
