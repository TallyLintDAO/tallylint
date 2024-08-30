import axios from "axios"
import * as CryptoJS from "crypto-js"
// import { API_KEY } from "./constants/api_key"

// const API_KEY =""
// const SECRET_KEY = API_KEY.Binance.secret_key
const SECRET_KEY = ""
const BASE_URL = "https://api.binance.com"

// const OK_PASSPHRASE = API_KEY.OK.passphrase
// const OK_API_KEY = API_KEY.OK.api_key
// const OK_SECRET_KEY = API_KEY.OK.secret_key
const OK_PASSPHRASE = ""
const OK_API_KEY = ""
const OK_SECRET_KEY = ""
const baseUrl = "https://www.okx.com"

//OK服务器时间
const getTimestamp = async (): Promise<string> => {
  const response = await axios.get(`${baseUrl}/api/v5/public/time`)
  if (response.status === 200) {
    return response.data.data[0].ts
  } else {
    console.error("OK NET ERROR")
    return new Date().toISOString()
  }
}
//本地时间，本地测试此数据可以成功通过OK的API
const getLocalTimestamp = (): string => {
  return new Date().toISOString()
}

const getSignature = (
  timestamp: string,
  method: string,
  requestPath: string,
  body: string = "",
): string => {
  const preHash = timestamp + method + requestPath + body
  console.log("preHASH", preHash)
  return CryptoJS.enc.Base64.stringify(
    CryptoJS.HmacSHA256(preHash, OK_SECRET_KEY),
  )
}

// 获取请求头
const getHeaders = (
  method: string,
  requestPath: string,
  body: string = "",
): any => {
  const timestamp = getLocalTimestamp()
  return {
    "OK-ACCESS-KEY": OK_API_KEY,
    "OK-ACCESS-SIGN": getSignature(timestamp, method, requestPath, body),
    "OK-ACCESS-TIMESTAMP": timestamp,
    "OK-ACCESS-PASSPHRASE": OK_PASSPHRASE,
    "Content-Type": "application/json",
  }
}

const getTradeHistory = async () => {
  // const method = "GET"
  // const requestPath = "/api/v5/trade/fills"
  // // const requestPath = "/api/v5/account/bills-archive"
  // const headers = getHeaders(method, requestPath)
  // const response = await axios.get(`${baseUrl}${requestPath}`, { headers })
  // return response.data
  const method = "GET"
  const requestPath = "/api/v5/trade/fills-history"
  const queryParams = "?instType=SPOT"
  const headers = await getHeaders(method, requestPath, queryParams)

  const response = await axios.get(`${baseUrl}${requestPath}${queryParams}`, {
    headers,
  })
  return response.data
}

export async function getOKInfo() {
  getTradeHistory()
    .then((data) => {
      console.log("getOKInfo", data)
    })
    .catch((error) => {
      console.error(error)
    })
}
export async function getAllSymbols() {
  // const url = `${BINANCE_URL}/api/v3/exchangeInfo`
  // const response = await axios.get(url)
  // console.log("getAllSymbols", response)
  // const endpoint = "/sapi/v1/asset/assetDividend"
  // const timestamp = Date.now()
  // const params: Record<string, any> = {
  //   timestamp,
  // }
  // const queryString = new URLSearchParams(params).toString()
  // try {
  //   const response = await apiClient.get(
  //     `${endpoint}?${queryString}&signature=c8db56825ae71d6d79447849e617115f4a920fa2acdcab2b053c4b2838bd6b71`,
  //   )
  //   console.log("response", response)
  //   return response.data
  // } catch (error) {
  //   console.error("Error fetching asset dividend records:", error)
  //   throw error
  // }
}
