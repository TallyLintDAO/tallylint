import { initAuth } from "@/api/auth"
import { MILI_PER_SECOND } from "@/api/constants/ic"
import type { WalletTag } from "@/types/user"
import { HttpAgent } from "@dfinity/agent"
import { IcrcAccount, IcrcIndexCanister } from "@dfinity/ledger-icrc"
import type { TransactionWithId } from "@dfinity/ledger-icrc/dist/candid/icrc_index"
import { Principal } from "@dfinity/principal"
import ic from "./icblast"

export const getAllTransactionsICRC1 = async (
  wallet: WalletTag,
  indexCanisterId: string,
  decimals: number,
) => {
  console.log("getAllTransactionsICRC1", indexCanisterId)
  const ai = await initAuth()
  if (ai.info) {
    const identity = ai.info.identity
    const agent = new HttpAgent({ identity })
    const myPrincipal = Principal.fromText(wallet.address)
    const canisterPrincipal = Principal.fromText(indexCanisterId)

    const { getTransactions: ICRC1_getTransactions } = IcrcIndexCanister.create(
      {
        agent: agent,
        canisterId: canisterPrincipal,
      },
    )
    const ICRC1getTransactions = await ICRC1_getTransactions({
      account: {
        owner: myPrincipal,
      } as IcrcAccount,
      max_results: BigInt(10000),
    })

    const transactionsInfo = ICRC1getTransactions.transactions
    console.log("getAllTransactionsICRC1", transactionsInfo)
    return transactionsInfo.map((transaction) => {
      formatICRC1Transaction(wallet, transaction, decimals)
    })
  }
}

const formatICRC1Transaction = (
  wallet: WalletTag,
  transactionWithId: TransactionWithId,
  decimals: number,
) => {
  const transaction = transactionWithId.transaction
  const hash = transactionWithId.id
  const timestampNormal = Number(transaction.timestamp) / MILI_PER_SECOND //处理时间戳为正常格式
  transaction["name"] = wallet.name
  transaction["decimals"] = decimals
  const type = transaction[transaction.kind]
  return {
    ...transaction,
    hash,
    timestamp: timestampNormal,
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
