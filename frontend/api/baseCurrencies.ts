import axios from "axios"

export const getBaseCurrencyPrice = async (currency: string): Promise<any> => {
  // https://www.exchangerate-api.com/docs/free
  const url = `https://open.er-api.com/v6/latest/${currency}`

  const response = await axios.get(url)
  if (response.status === 200) {
    //找到对应的美元/
    const rate = response.data.rates[currency]
    console.log("getBaseCurrencyPrice", response)
    if (rate) {
      return {
        rate: rate,
        time_last_update_unix: response.data.time_last_update_unix,
      }
    }
  }
}
