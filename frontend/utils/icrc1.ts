import { initAuth } from "@/api/auth"
import { HttpAgent } from "@dfinity/agent"
import { IcrcAccount, IcrcIndexCanister } from "@dfinity/ledger-icrc"
import { Principal } from "@dfinity/principal"
import ic from "./icblast"

export const getAllTransactionsICRC1 = async (indexCanisterId: string) => {
  console.log("getAllTransactionsICRC1", indexCanisterId)
  const ai = await initAuth()
  if (ai.info) {
    const identity = ai.info.identity
    const agent = new HttpAgent({ identity })
    const myPrincipal = Principal.fromText(
      "rintb-5nazg-thqf4-rnq2c-6geuh-ufcjx-fsfm7-qinyq-ma2gb-5rgny-7ae",
    )
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
      const hash = transaction.id
    })
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
