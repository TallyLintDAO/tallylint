export const distanceFromCurrentDate = (targetTimestamp: bigint): string => {
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
