
// 统一处理显示用户名的逻辑
export function showUsername(username: string, principalId: string): string {
    const MAX_LENGTH = 15; // 显示的最长字符
    if (username) {
        if (username.length >= MAX_LENGTH) {
            return username.substring(0, MAX_LENGTH - 3) + '...';
        }
        return username;
    }
    //没有名字返回principalId作为名字，保留前5位，后3位
    return principalId ?
        principalId.substring(0, 5) + "..." +
        principalId.substring(principalId.length - 3, principalId.length)
        : "";
}

//根据精度计算对应的代币数值
export function currencyCalculate(amount: string, decimals: number): number {
    const floatValue = parseInt(amount) / Math.pow(10, decimals);
    return floatValue;
}
