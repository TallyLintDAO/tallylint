import moment from "moment-timezone"

export const distanceFromCurrentDate = (
  targetTimestamp: bigint | number,
): string => {
  const targetDate = new Date(Number(targetTimestamp))
  const currentDate = new Date()
  const timeDifference = currentDate.getTime() - targetDate.getTime()

  const millisecondsInDay = 1000 * 60 * 60 * 24
  const daysAgo = Math.floor(timeDifference / millisecondsInDay)
  const monthsAgo = Math.floor(daysAgo / 30)

  if (monthsAgo > 0) {
    return `${monthsAgo} month${monthsAgo > 1 ? "s" : ""} ago`
  } else if (daysAgo > 0) {
    return `${daysAgo} day${daysAgo > 1 ? "s" : ""} ago`
  } else {
    return "Today"
  }
}

export interface YearTimestamp {
  year: number | string
  timestamp: { start: number; end: number }
}

export const getYearTimestamps = (): YearTimestamp[] => {
  const currentYear = new Date().getFullYear()
  const timestamps: YearTimestamp[] = []
  //从ICP的2021年上线开始，至今。
  for (let year = 2021; year <= currentYear; year++) {
    const startOfYear = new Date(year, 0, 1) // January 1st
    const endOfYear = new Date(year, 11, 31, 23, 59, 59) // December 31st, 23:59:59
    timestamps.push({
      year: year,
      timestamp: {
        start: startOfYear.getTime(),
        end: endOfYear.getTime(),
      },
    })
  }

  return timestamps
}

//通过设置默认时区，可以影响到使用moment转换的所有时间。
//只返回时分秒 moment会根据设置的时区来转换 moment.tz.setDefault(moment.tz.guess())
export const showCustomTimezoneTime = (timestamp: number | bigint): any => {
  const time = moment(Number(timestamp)).format("HH:mm:ss")
  return time
}
//只返回年月日
export const showCustomTimezoneDate = (timestamp: number | bigint): any => {
  const date = moment(Number(timestamp)).format("YYYY/MM/DD")
  return date
}
