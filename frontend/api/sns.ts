import type { ICRC1Info } from "@/types/sns"
import { TTL, getCache } from "@/utils/cache"
import { showMessageError } from "@/utils/message"
import axios from "axios"

const AGGREGATOR_PAGE_SIZE = 10
export const SNS_AGGREGATOR_CANISTER_URL =
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
    //由于一页只能查10个数据，所以这里自动自增页码，查完所有sns。
    const nextPageData = await querySnsAggregator(page + 1)
    return [...data, ...nextPageData]
  }
  return data
}

export const getSNSInfoCache = async (): Promise<ICRC1Info[]> => {
  return await getCache({
    key: "SNS_LIST",
    execute: () => getAllSNSInfo(),
    ttl: TTL.day7,
    isLocal: true,
    update: true, //返回之前缓存的数据，并且异步加载新的数据
  })
}

export const getAllSNSInfo = async (): Promise<ICRC1Info[]> => {
  try {
    const data = await querySnsAggregator()
    // lifecycle === 3 才是通过的SNS项目，这里筛选通过的SNS项目，没有通过的全部抛弃
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
          list_sns_canisters: { governance, index, ledger, swap, root },
          icrc1_metadata,
          icrc1_fee,
          meta,
        } = sns
        return {
          //按自己的需求重新组装SNS的字段
          canisters: { governance, index, ledger, swap, root },
          name: icrc1_metadata.find(([key]) => key.endsWith(`:name`))[1].Text,
          symbol: icrc1_metadata.find(([key]) => key.endsWith(`:symbol`))[1]
            .Text,
          fee: icrc1_fee[0],
          decimals: icrc1_metadata.find(([key]) => key.endsWith(`:decimals`))[1]
            .Nat[0],
          meta: {
            ...meta,
            logo: SNS_AGGREGATOR_CANISTER_URL + meta.logo, //直接将logo地址一步到位，免得后面还需要手动加URL
          },
        }
      })
    return snses
  } catch (err) {
    showMessageError("Failed to get SNS information.")
    throw new Error("Error querying Snses")
  }
}
