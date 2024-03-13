export interface SnsInfo {
  canisters: {
    //Related canister id
    governance: string
    index: string
    ledger: string
    root: string
    swap: string
  }
  decimals: number
  fee: number
  meta: {
    description: string
    logo: string
    name: string
    url: string
  }
  name: string
  symbol: string
}

export interface Currency {
  decimals: number
  symbol: string
}

export interface IRCR1Price {
  id: number
  low: number //最低价
  high: number
  close: number //收盘价
  open: number
  timestamp: number
}

export interface InferredTransaction {
  hash: string
  timestamp: number
  t_type: string // rust中type为关键字，所以不能使用type作为字段名
  walletName: string
  details: {
    status: string
    fee: number
    to?: string
    from?: string
    amount: number
    price: number // 发生交易时代币的单价
    currency: Currency
    ledgerCanisterId: string
    cost: number
    profit: number
    value: number
  }
}

export interface transactionBackend {
  t_type: string
  timestamp: number
  hash: string
  tag?: string
  manual?: boolean //是否为手动导入
  commit?: string //用户给予的备注
}
