import axios from "axios"

const AGGREGATOR_PAGE_SIZE = 10
const SNS_AGGREGATOR_CANISTER_URL =
  "https://3r4gx-wqaaa-aaaaq-aaaia-cai.icp0.io"
const AGGREGATOR_CANISTER_VERSION = "v1"

const AGGREGATOR_URL = `${SNS_AGGREGATOR_CANISTER_URL}/${AGGREGATOR_CANISTER_VERSION}/sns`

const aggregatorPageUrl = (page) => `list/page/${page}/slow.json`

export const querySnsAggregator = async (page = 0) => {
  const res = await axios.get(`${AGGREGATOR_URL}/${aggregatorPageUrl(page)}`)
  if (res.status !== 200) {
    if (page > 0) {
      return []
    }
    throw new Error("Error loading SNS projects from aggregator canister")
  }
  const data = res.data
  if (data.length === AGGREGATOR_PAGE_SIZE) {
    //由于一页只能查10个数据，所以这里自动自增查完所有sns。
    const nextPageData = await querySnsAggregator(page + 1)
    return [...data, ...nextPageData]
  }
  return data
}

export const getAllSNSInfo = async () => {
  try {
    const data = await querySnsAggregator()
    console.log("getSNSInfo", data)
    // 3 === Committed
    const snses = data
      .filter(
        ({
          swap_state: {
            swap: { lifecycle },
          },
        }) => lifecycle === 3,
      )
      .map((sns) => {
        const {
          list_sns_canisters: {
            governance: governance_canister_id,
            index: index_canister_id,
            ledger: ledger_canister_id,
            root: root_canister_id,
            swap: swap_canister_id,
          },
          icrc1_metadata,
          icrc1_fee,
          meta,
        } = sns

        const assembledStructure = {
          canister_ids: {
            governance_canister_id,
            index_canister_id,
            ledger_canister_id,
            root_canister_id,
            swap_canister_id,
          },
          icrc1_metadata,
          icrc1_fee,
          meta,
        }

        return assembledStructure
      })
    console.log("filter sns", snses)
    // writeFileSync(join(DATA_FOLDER, "snses.json"), JSON.stringify(snses))

    // await saveLogos(snses)
  } catch (err) {
    throw new Error("Error querying Snses")
  }
}
