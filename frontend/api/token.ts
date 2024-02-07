import { COINGECKO_URL } from "@/api/constants/ic"
import { getCache, TTL } from "@/utils/cache"
import { binarySearchClosestICPPrice } from "@/utils/math"
import { showMessageError } from "@/utils/message"
import axios from "axios"

export const matchICPPrice = async (timestamp: number): Promise<number> => {
  //将小数点的时间戳转为整数时间戳
  timestamp = Math.floor(timestamp)
  //获取ICP的所有价格历史数据，并通过getCache保存到本地缓存中，ttl为1天，方便调用。
  const priceHistory = await getCache({
    key: "ICP_Price_History",
    execute: () => getICPPriceHistory(),
    ttl: TTL.day1,
    isLocal: true,
  })
  // 返回最接近时间戳对应的币价，如果没有找到则返回 undefined
  return binarySearchClosestICPPrice(priceHistory, timestamp)[1]
}

export const getICPPriceHistory = async (): Promise<[number, number][]> => {
  try {
    //获取coingecko的所有ICP价格历史数据，目前只有coingecko可以获得这么详细的历史数据。
    const url = `${COINGECKO_URL}/api/v3/coins/internet-computer/market_chart`
    const params = {
      vs_currency: "usd",
      days: "max",
      interval: "daily",
      precision: "2",
    }

    const response = await axios.get(url, { params })
    if (response.status === 200) {
      // 解析响应数据
      // priceData 包含时间戳和价格数据
      const priceData: [number, number][] = response.data.prices
      return priceData
    } else {
      showMessageError(
        "Can not connect CoinGecko api, please check if you have access to CoinGecko",
      )
      throw new Error("Failed to fetch ICP price data")
    }
  } catch (error) {
    showMessageError(
      "Can not connect CoinGecko api, please check if you have access to CoinGecko",
    )
    console.error("Error fetching ICP price data:", error)
    throw error
  }
}

export const getICPNowPrice = async (): Promise<number> => {
  try {
    //获取coingecko的当前ICP历史数据。
    const url = `${COINGECKO_URL}/api/v3/simple/price`
    const params = { ids: "internet-computer", vs_currencies: "usd" }

    const response = await axios.get(url, { params })
    if (response.status === 200) {
      // 解析响应数据
      // priceData 包含时间戳和价格数据
      const priceData = response.data["internet-computer"].usd
      return priceData
    } else {
      showMessageError(
        "Can not connect CoinGecko api, please check if you have access to CoinGecko",
      )
      throw new Error("Failed to fetch ICP price data")
    }
  } catch (error) {
    showMessageError(
      "Can not connect CoinGecko api, please check if you have access to CoinGecko",
    )
    console.error("Error fetching ICP price data:", error)
    throw error
  }
}
