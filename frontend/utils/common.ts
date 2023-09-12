//根据精度计算对应的代币数值
export function currencyCalculate(amount: string, decimals: number): number {
    const floatValue = parseInt(amount) / Math.pow(10, decimals);
    return floatValue;
}
