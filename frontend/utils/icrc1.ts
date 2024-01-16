import { initAuth } from "@/api/auth"
import { MILI_PER_SECOND } from "@/api/constants/ic"
import type { Currency } from "@/types/sns"
import type { WalletTag } from "@/types/user"
import { HttpAgent } from "@dfinity/agent"
import { IcrcAccount, IcrcIndexCanister } from "@dfinity/ledger-icrc"
import type { TransactionWithId } from "@dfinity/ledger-icrc/dist/candid/icrc_index"
import { Principal } from "@dfinity/principal"
import { currencyCalculate } from "./common"
import ic from "./icblast"

export const getAllTransactionsICRC1 = async (
  wallet: WalletTag,
  indexCanisterId: string,
  currency: Currency,
) => {
  console.log("getAllTransactionsICRC1", indexCanisterId)
  const ai = await initAuth()
  if (ai.info) {
    const identity = ai.info.identity
    const agent = new HttpAgent({ identity })
    const address = Principal.fromText(wallet.address)
    const canisterPrincipal = Principal.fromText(indexCanisterId)

    const { getTransactions: ICRC1_getTransactions } = IcrcIndexCanister.create(
      {
        agent: agent,
        canisterId: canisterPrincipal,
      },
    )
    const ICRC1getTransactions = await ICRC1_getTransactions({
      account: {
        owner: address,
      } as IcrcAccount,
      max_results: BigInt(10000),
    })

    const transactionsInfo = ICRC1getTransactions.transactions
    console.log("getAllTransactionsICRC1", transactionsInfo)
    return transactionsInfo.map((transaction) => {
      const res = formatICRC1Transaction(wallet, transaction, currency)
      console.log("formation icrc1", res)
      return res
    })
  }
}

const formatICRC1Transaction = (
  wallet: WalletTag,
  transactionWithId: TransactionWithId,
  currency: Currency,
) => {
  const { transaction, id } = transactionWithId
  const timestampNormal = Number(transaction.timestamp) / MILI_PER_SECOND //处理时间戳为正常格式
  let detail = transaction[transaction.kind][0]
  detail.amount = currencyCalculate(detail.amount, currency.decimals)
  detail.to = detail.to.owner.toString()
  delete detail.created_at_time

  if (transaction.kind === "transfer") {
    detail.fee = currencyCalculate(detail.fee[0], currency.decimals)
    detail.from = detail.from.owner.toString()
  } else if (transaction.kind === "mint") {
    //mint没有fee，且没有from地址
    detail.fee = 0
    detail.from = "Minting Account"
    detail.tag = "mint"
  } else {
    //kind == burn || approve
  }
  const type = detail.to === wallet.address ? "RECEIVE" : "SEND"
  return {
    detail,
    name: wallet.name,
    currency,
    hash: id,
    timestamp: timestampNormal,
    type,
  }
}

export const getICRC1Price = async (ledgerCanisterId: string) => {
  //从记录罐子中获取存储罐子的id
  let recordStorageCanister = await ic("ggzvv-5qaaa-aaaag-qck7a-cai")
  let tokenStorage = await ic(
    await recordStorageCanister.tokenStorage(ledgerCanisterId),
  )
  // @param1 text tokenId token canister id
  // @param2 int time start time of charts, default is 0
  // @param3 int interval time interval of charts(seconds), 1 day is 24 * 60 * 60
  // @param4 nat limit charts data length
  let priceHistory = await tokenStorage.getTokenPricesData(
    ledgerCanisterId,
    0,
    0,
    1000,
  )
  console.log("getICRC1Price", priceHistory)
}
