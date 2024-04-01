import { BINANACE_URL } from "@/api/constants/ic"
import { TTL, getCache } from "@/utils/cache"
import { getYearTimestamps } from "@/utils/date"
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

export const getICPPriceHistory = async (): Promise<any> => {
  try {
    //获取binance的所有ICP价格历史数据，目前coingecko只允许调用一年以内的数据，无法使用。
    const url = `${BINANACE_URL}/api/v3/klines`
    let priceData = []
    //由于币安一次只能请求500条数据，所以这里就分别请求每年的ICP价格历史再组装。
    for (const {
      value: { start, end },
    } of getYearTimestamps()) {
      //获取从2021年开始的每年数据
      const params = {
        symbol: "ICPUSDT",
        startTime: start,
        endTime: end,
        interval: "1d",
      }
      const response = await axios.get(url, { params })
      if (response.status === 200) {
        // 解析响应数据response.data
        // [
        //   1499040000000, // k线开盘时间
        //   "0.01634790", // 开盘价
        //   "0.80000000", // 最高价
        //   "0.01575800", // 最低价
        //   "0.01577100", // 收盘价(当前K线未结束的即为最新价)
        //   "148976.11427815", // 成交量
        //   1499644799999, // k线收盘时间
        //   "2434.19055334", // 成交额
        //   308, // 成交笔数
        //   "1756.87402397", // 主动买入成交量
        //   "28.46694368", // 主动买入成交额
        //   "17928899.62484339", // 请忽略该参数
        // ]

        // priceData 包含时间戳和价格数据
        const timestampAndPrice = response.data.map((item) => [
          item[0],
          Number(item[1]),
        ])
        priceData = priceData.concat(timestampAndPrice)
      }
    }
    console.log("priceData", priceData)
    return priceData

    if (priceData) {
    } else {
      showMessageError(
        "Can not connect Binance api, please check if you have access to Binance",
      )
      throw new Error("Failed to fetch ICP price data")
    }
  } catch (error) {
    showMessageError(
      "Can not connect Binance api, please check if you have access to Binance",
    )
    console.error("Error fetching ICP price data:", error)
    throw error
  }
}

export const getICPNowPrice = async (): Promise<number> => {
  try {
    //获取Binance的当前ICP历史数据。
    const url = `${BINANACE_URL}/api/v3/ticker/price`
    const params = { symbol: "ICPUSDT" }

    const response = await axios.get(url, { params })
    console.log("getICPNowPrice", response)
    if (response.status === 200) {
      //直接返回价格
      return Number(response.data.price)
    } else {
      showMessageError(
        "Can not connect Binance api, please check if you have access to Binance",
      )
      throw new Error("Failed to fetch ICP price data")
    }
  } catch (error) {
    showMessageError(
      "Can not connect Binance api, please check if you have access to Binance",
    )
    console.error("Error fetching ICP price data:", error)
    throw error
  }
}
