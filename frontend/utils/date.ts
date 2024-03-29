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

interface YearTimestamp {
  year: number
  start: number
  end: number
}

export const getYearTimestamps = () => {
  const currentYear = new Date().getFullYear()
  const timestamps: YearTimestamp[] = []

  for (let year = 2021; year <= currentYear; year++) {
    const startOfYear = new Date(year, 0, 1) // January 1st
    const endOfYear = new Date(year, 11, 31, 23, 59, 59) // December 31st, 23:59:59
    timestamps.push({
      year: year,
      start: startOfYear.getTime(),
      end: endOfYear.getTime(),
    })
  }

  return timestamps
}
