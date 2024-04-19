import { TTL, getCache } from "@/utils/cache"
import { showMessageError } from "@/utils/message"
import axios from "axios"

// 获取汇率缓存，防止多次调用api导致封锁
export async function getBaseCurrencyPriceCache(
  currency: string,
): Promise<any> {
  return await getCache({
    key: "Currency_" + currency.toUpperCase(),
    execute: () => getBaseCurrencyPrice(currency),
    ttl: TTL.day1,
    isLocal: true,
  })
}

export const getBaseCurrencyPrice = async (currency: string): Promise<any> => {
  try {
    // https://www.exchangerate-api.com/docs/free
    // 以USD为基础货币单位，获得其他货币单位的汇率
    const url = `https://open.er-api.com/v6/latest/USD`

    const response = await axios.get(url)
    if (response.status === 200) {
      //找到对应美元需要转换的汇率
      const rate = response.data.rates[currency]

      return {
        rate: rate,
        time_last_update_unix: response.data.time_last_update_unix,
      }
    } else {
      showMessageError(`Sorry, we can't get ${currency} rate right now.`)
    }
  } catch (error) {
    showMessageError(`Sorry, we can't get ${currency} rate right now.`)
    console.error("Error fetching exchangerate-api currency data: ", error)
    throw error
  }
}
