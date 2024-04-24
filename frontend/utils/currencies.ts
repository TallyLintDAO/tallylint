import { getUserCurrencyCode } from "@/api/user"
import { numberToFixed } from "./math"

export interface baseCurrency {
  code: string
  name: string
}

export const baseCurrencies: baseCurrency[] = [
  { code: "USD", name: "United States Dollar" },
  { code: "EUR", name: "Euro" },
  { code: "CAD", name: "Canadian Dollar" },
  { code: "AED", name: "United Arab Emirates Dirham" },
  { code: "ARS", name: "Argentine Peso" },
  { code: "AUD", name: "Australian Dollar" },
  { code: "BDT", name: "Bangladeshi Taka" },
  { code: "BRL", name: "Brazilian Real" },
  { code: "CHF", name: "Swiss Franc" },
  { code: "CLP", name: "Chilean Peso" },
  { code: "CNY", name: "Chinese Yuan" },
  { code: "COP", name: "Colombian Peso" },
  { code: "CZK", name: "Czech Koruna" },
  { code: "DKK", name: "Danish Krone" },
  { code: "EGP", name: "Egyptian Pound" },
  { code: "GBP", name: "British Pound Sterling" },
  { code: "HKD", name: "Hong Kong Dollar" },
  { code: "HRK", name: "Croatian Kuna" },
  { code: "HUF", name: "Hungarian Forint" },
  { code: "IDR", name: "Indonesian Rupiah" },
  { code: "ILS", name: "Israeli New Shekel" },
  { code: "INR", name: "Indian Rupee" },
  { code: "IQD", name: "Iraqi Dinar" },
  { code: "JPY", name: "Japanese Yen" },
  { code: "KES", name: "Kenyan Shilling" },
  { code: "KRW", name: "South Korean Won" },
  { code: "LKR", name: "Sri Lankan Rupee" },
  { code: "MAD", name: "Moroccan Dirham" },
  { code: "MXN", name: "Mexican Peso" },
  { code: "MYR", name: "Malaysian Ringgit" },
  { code: "NOK", name: "Norwegian Krone" },
  { code: "NZD", name: "New Zealand Dollar" },
  { code: "PEN", name: "Peruvian Sol" },
  { code: "PHP", name: "Philippine Peso" },
  { code: "PKR", name: "Pakistani Rupee" },
  { code: "PLN", name: "Polish Zloty" },
  { code: "RON", name: "Romanian Leu" },
  { code: "RUB", name: "Russian Ruble" },
  { code: "SAR", name: "Saudi Riyal" },
  { code: "SEK", name: "Swedish Krona" },
  { code: "SGD", name: "Singapore Dollar" },
  { code: "THB", name: "Thai Baht" },
  { code: "TRY", name: "Turkish Lira" },
  { code: "TWD", name: "New Taiwan Dollar" },
  { code: "UAH", name: "Ukrainian Hryvnia" },
  { code: "UZS", name: "Uzbekistani Som" },
  { code: "VND", name: "Vietnamese Dong" },
  { code: "ZAR", name: "South African Rand" },
]

// 用户所选择的货币符号代码
let currencyCode = "USD"
//获取用户设置的货币代码
getUserCurrencyCode().then((code) => {
  currencyCode = code
})
//手动设置货币代码
export const setCurrencyCode = (code: string) => {
  currencyCode = code
}

//将数字转换为对应货币适合的格式，且添加对应的货币符号
export const convertCurrency = (amount: number): string => {
  // 此方法应用于展示金额时更改样式和增加货币代码， rate应在展示之前，刚获取数据时进行修改，而不是在这里。
  const amountConverted = numberToFixed(amount, 2)
  //设置为undefined，不对地区做限制
  const formattedAmount = new Intl.NumberFormat(undefined, {
    style: "currency",
    currency: currencyCode,
  }).format(amountConverted)
  // 如果货币代码为 USD，则去除 "US" 字样，只保留美元符号
  if (currencyCode === "USD") {
    return formattedAmount.replace("US", "")
  } else {
    return formattedAmount
  }
}

export default baseCurrencies
