import { initAuth } from "@/api/auth"
import { HttpAgent } from "@dfinity/agent"
import { IcrcAccount, IcrcIndexCanister } from "@dfinity/ledger-icrc"
import { Principal } from "@dfinity/principal"
import ic from "./icblast"

export const getAllTransactionsICRC1 = async (canister_id: any) => {
  const ai = await initAuth()
  if (ai.info) {
    const identity = ai.info.identity
    const agent = new HttpAgent({ identity })
    const myPrincipal = Principal.fromText(
      "rintb-5nazg-thqf4-rnq2c-6geuh-ufcjx-fsfm7-qinyq-ma2gb-5rgny-7ae",
    )
    const canisterPrincipal = Principal.fromText(canister_id)

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
    console.log("transactionsInfo", transactionsInfo)
  }
}

export const getICRC1Price = async (tokenCanisterID) => {
  let tokenStorage = await ic("moe7a-tiaaa-aaaag-qclfq-cai")
  // @param1 text tokenId token canister id
  // @param2 int time start time of charts, default is 0
  // @param3 int interval time interval of charts(seconds), 1 day is 24 * 60 * 60
  // @param4 nat limit charts data length
  let priceHistory = await tokenStorage.getTokenPricesData(
    tokenCanisterID,
    0,
    0,
    1000,
  )
  console.log("getICRC1Price", priceHistory)
}
