// 将目标数值和现有数值转化为百分比（保留2位小数），满100%则计算为100%
export function calculatePercent(
  currentValue: number | bigint,
  totalValue: number | bigint,
): number {
  currentValue = Number(currentValue)
  totalValue = Number(totalValue)
  const percent = (currentValue / totalValue) * 100
  return Number((percent >= 100 ? 100 : percent).toFixed(2))
}
