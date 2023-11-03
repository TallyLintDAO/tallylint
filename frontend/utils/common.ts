import { AccountIdentifier } from "@dfinity/ledger-icp"
import { Principal } from "@dfinity/principal"

//根据精度计算对应的代币数值
export function currencyCalculate(amount: string, decimals: number): number {
  const floatValue = parseInt(amount) / Math.pow(10, decimals)
  return floatValue
}

//将principal id 转换为 account id
export function p2a(principal: string): string {
  const principalId = Principal.from(principal)
  const identity = AccountIdentifier.fromPrincipal({ principal: principalId })
  return identity.toHex()
}
