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
  id: bigint
  low: number //最低价
  high: number
  close: number //收盘价
  open: number
  timestamp: bigint
}
