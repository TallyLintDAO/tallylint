import axios from "axios";
import { COINGECKO_URL } from "@/api/constants/ic";
import { getCache, TTL } from "@/utils/cache";

export const getICPPrice = async (timestamp: number): Promise<string | undefined> => {
    //将小数点的时间戳转为整数时间戳
    timestamp = Math.floor(timestamp)
    //获取ICP的所有价格历史数据，并通过getCache保存到本地缓存中，ttl为1天，方便调用。
    const priceHistory = await getCache({
        key: 'ICP_Price_History',
        execute: () => getICPPriceHistory(),
        ttl: TTL.day1,
        isLocal: true,
    })
    // 初始化最近时间戳和对应的币价
    let closestTimestamp = 0;
    let closestPrice = "";
    // 遍历价格历史数据数组，寻找最接近的时间戳
    for (const [ts, price] of priceHistory) {
        // console.log("Timestamp:", ts,"Target:",timestamp, "Price:", price);
        const currentTimestamp = ts;
        // 如果找到与输入时间戳相等的时间戳，直接返回对应的币价
        if (currentTimestamp === timestamp) {
            return price;
        }
        // 计算与输入时间戳的差值的绝对值
        const timeDiff = Math.abs(currentTimestamp - timestamp);
        // 如果当前时间戳更接近输入时间戳，则更新最近时间戳和币价
        if (timeDiff < Math.abs(closestTimestamp - timestamp) || closestTimestamp === 0) {
            closestTimestamp = currentTimestamp;
            closestPrice = price;
        }
    }
    // console.log("price back: ", closestPrice,"icpprice timestamp: ", timestamp)

    // 返回最接近时间戳对应的币价，如果没有找到则返回 undefined
    return closestPrice || undefined;

}


export const getICPPriceHistory = async (): Promise<[number, number][]> => {
    try {
        //获取coingecko的所有ICP价格历史数据。
        const url = `${COINGECKO_URL}/api/v3/coins/internet-computer/market_chart`;
        const params = {
            vs_currency: 'usd',
            days: 'max',
            interval: 'daily',
            precision: '2'
        };

        const response = await axios.get(url, {params});

        if (response.status === 200) {
            // 解析响应数据
            const priceData: [number, number][] = response.data.prices;
            // priceData 包含时间戳和价格数据
            // 这里可以根据需要对数据进行处理
            // console.log("priceData", priceData)
            return priceData;
        } else {
            throw new Error('Failed to fetch ICP price data');
        }
    } catch (error) {
        console.error('Error fetching ICP price data:', error);
        throw error;
    }
};
