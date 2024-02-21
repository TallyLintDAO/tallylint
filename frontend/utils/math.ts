import type { IRCR1Price } from "@/types/sns"

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
//二分法匹配ICP交易价格
export function binarySearchClosestICPPrice(
  array: number[][],
  targetTimestamp: number,
): number[] {
  let leftIndex = 0
  let rightIndex = array.length - 1
  // 判断数组的排列顺序（升序或降序）
  const isAscending = array[leftIndex][0] < array[rightIndex][0]
  while (leftIndex <= rightIndex) {
    const midIndex = Math.floor((leftIndex + rightIndex) / 2)
    const midTimestamp = array[midIndex][0]

    if (midTimestamp === targetTimestamp) {
      return array[midIndex] // 找到完全匹配的时间戳
    }

    // 根据数组的排列顺序调整二分查找方向
    if (
      (isAscending && midTimestamp < targetTimestamp) ||
      (!isAscending && midTimestamp > targetTimestamp)
    ) {
      leftIndex = midIndex + 1
    } else {
      rightIndex = midIndex - 1
    }
  }

  // 此时，left 指向比目标时间戳大的最小值，right 指向比目标时间戳小的最大值
  // 比较这两个值，找到最接近的值
  if (rightIndex < 0) {
    return array[leftIndex]
  } else if (leftIndex >= array.length) {
    return array[rightIndex]
  } else {
    const leftDifference = Math.abs(targetTimestamp - array[leftIndex][0])
    const rightDifference = Math.abs(targetTimestamp - array[rightIndex][0])

    return leftDifference < rightDifference
      ? array[leftIndex]
      : array[rightIndex]
  }
}
//二分法匹配ICRC1 TOken交易价格
export function binarySearchClosestICRC1Price(
  array: IRCR1Price[],
  targetTimestamp: number,
): IRCR1Price {
  // icpswap的时间戳是unix时间戳，需要*1000来转换成js时间戳。
  let leftIndex = 0
  let rightIndex = array.length - 1
  // console.log("binarySearchClosestICRC1Price", array, rightIndex)
  // 判断数组的排列顺序（升序或降序）
  const isAscending = array[leftIndex].timestamp < array[rightIndex].timestamp
  while (leftIndex <= rightIndex) {
    const midIndex = Math.floor((leftIndex + rightIndex) / 2)
    const midTimestamp = Number(array[midIndex].timestamp)

    if (midTimestamp === targetTimestamp) {
      return array[midIndex] // 找到完全匹配的时间戳
    }

    // 根据数组的排列顺序调整二分查找方向
    if (
      (isAscending && midTimestamp < targetTimestamp) ||
      (!isAscending && midTimestamp > targetTimestamp)
    ) {
      leftIndex = midIndex + 1
    } else {
      rightIndex = midIndex - 1
    }
  }
  // console.log(
  //   "icpprice timestamp: ",
  //   targetTimestamp,
  //   array[leftIndex],
  //   leftIndex,
  //   rightIndex,
  // )
  // 此时，left 指向比目标时间戳大的最小值，right 指向比目标时间戳小的最大值
  // 比较这两个值，找到最接近的值
  if (rightIndex < 0) {
    return array[leftIndex]
  } else if (leftIndex >= array.length) {
    return array[rightIndex]
  } else {
    const leftDifference = Math.abs(
      targetTimestamp - Number(array[leftIndex].timestamp),
    )
    const rightDifference = Math.abs(
      targetTimestamp - Number(array[rightIndex].timestamp),
    )
    return leftDifference < rightDifference
      ? array[leftIndex]
      : array[rightIndex]
  }
}
