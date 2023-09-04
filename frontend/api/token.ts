import axios from "axios";
import { COINGECKO_URL } from "@/api/constants/ic";
import { getCache, TTL } from "@/utils/cache";

// export const getICPPrice = async (timesteam: string): Promise<string> => {
//     //获取ICP的所有价格历史数据，并通过getCache保存到本地缓存中，ttl为1天，方便调用。
//     await getCache({
//         key: 'ICP_Price_History',
//         execute: () => getICPPriceHistory(),
//         ttl: TTL.day1,
//         isLocal: true,
//     })
//
//     return
// }


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
            const priceData: [number, number][] = response.data.price;
            // priceData 包含时间戳和价格数据
            // 这里可以根据需要对数据进行处理
            console.log("priceData", priceData)
            return priceData;
        } else {
            throw new Error('Failed to fetch ICP price data');
        }
    } catch (error) {
        console.error('Error fetching ICP price data:', error);
        throw error;
    }
};
