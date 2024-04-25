import { TTL, getCache } from "@/utils/cache"
import { setCurrencyCode } from "@/utils/currencies"
import { showMessageError } from "@/utils/message"
import axios from "axios"
import { getUserCurrencyCode } from "./user"

//全局存储rate变量，方便调用
export let rate: number
export let rate_time_last_update: number
;(async () => {
  const { rate: initialRate, time_last_update_unix: initialTime } =
    await getUserCurrencyRate()
  rate = initialRate
  rate_time_last_update = initialTime * 1000
})()

// 获取用户所选择货币与美元的汇率
export async function getUserCurrencyRate(): Promise<any> {
  // 默认为USD
  let currencyCode = "USD"
  // 获取用户设置的货币符号方法
  const res = await getUserCurrencyCode()
  currencyCode = res
  const data = await getBaseCurrencyPriceCache(currencyCode)
  rate = data.rate
  rate_time_last_update = data.time_last_update_unix * 1000
  return data
}

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

export const getBaseCurrencyPrice = async (
  currency: string,
): Promise<{ rate: number; time_last_update_unix: number }> => {
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
      showMessageError(
        `Sorry, we can't get ${currency} rate right now. We had to temporarily set currency display to USD`,
      )
      setCurrencyCode("USD")
      return {
        rate: 1,
        time_last_update_unix: 0,
      }
    }
  } catch (error) {
    showMessageError(
      `Sorry, we can't get ${currency} rate right now. We had to temporarily set currency display to USD`,
    )
    console.error("Error fetching exchangerate-api currency data: ", error)
    setCurrencyCode("USD")
    return {
      rate: 1,
      time_last_update_unix: 0,
    }
  }
}
