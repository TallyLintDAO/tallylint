<template>
  <div class="taxreport-container">
    <div class="text-h4 row q-gutter-md items-center">
      <span> Tax Report For </span>
      <q-select
        outlined
        v-model="selectedYear"
        :options="dateOptions"
        option-label="year"
        style="max-width: 200px; font-size: 2.125rem"
      />
    </div>
    <div
      v-if="selectedYear.timestamp.start !== 0"
      class="text-subtitle2 q-mb-md text-grey"
    >
      {{
        `Jan 1, ${new Date(
          selectedYear.timestamp.start,
        ).getFullYear()} to Dec 31, ${new Date(
          selectedYear.timestamp.start,
        ).getFullYear()}`
      }}
    </div>
    <div class="row">
      <div class="col-8 q-pa-md" style="padding-left: 0">
        <q-card class="my-card q-mb-md">
          <q-card-section class="bg-grey-1">
            <div class="text-h6">Summary</div>
          </q-card-section>
          <q-card-section>
            <div>
              TallyLint needs your full transaction history (fiat → crypto →
              crypto → fiat) in order to calculate your tax reports correctly.
              The transactions used in this report are summarized below.
              <br />
              <div class="q-gutter-sm">
                <q-badge color="teal" v-if="!walletLoading">
                  {{ walletAmount }} wallet{{ walletAmount !== 1 ? "s" : "" }}
                </q-badge>
                <q-badge color="blue" v-if="!walletLoading">
                  {{ transactionAmount }} synced transaction{{
                    transactionAmount !== 1 ? "s" : ""
                  }}
                </q-badge>
              </div>
            </div>
          </q-card-section>
          <q-separator />
          <q-list separator>
            <q-item clickable v-ripple>
              <q-item-section>Capital gains / P&L</q-item-section>
              <q-skeleton v-if="walletLoading" type="text" />
              <q-item-section v-else side>
                <CurrencyUSD :amount="taxReportData.capital_gain_or_loss" />
              </q-item-section>
            </q-item>
            <q-item clickable v-ripple>
              <q-item-section>
                Other gains (futures, derivatives etc)
              </q-item-section>
              <q-skeleton v-if="walletLoading" type="text" />
              <q-item-section v-else side>
                <CurrencyUSD :amount="taxReportData.other_gain" />
              </q-item-section>
            </q-item>
            <q-item clickable v-ripple>
              <q-item-section overline>Income</q-item-section>
              <q-skeleton v-if="walletLoading" type="text" />
              <q-item-section v-else side>
                <CurrencyUSD :amount="taxReportData.income" />
              </q-item-section>
            </q-item>
            <q-item clickable v-ripple>
              <q-item-section>
                <q-item-label>Costs & expenses</q-item-label>
                <!-- <q-icon name="warning" /> -->
              </q-item-section>
              <q-skeleton v-if="walletLoading" type="text" />
              <q-item-section v-else side>
                <CurrencyUSD :amount="taxReportData.costs_expenses" />
              </q-item-section>
            </q-item>
            <q-item clickable v-ripple>
              <q-item-section>
                <q-item-label>Gifts, donations & lost coins</q-item-label>
              </q-item-section>
              <q-skeleton v-if="walletLoading" type="text" />
              <q-item-section v-else side>
                <CurrencyUSD
                  :amount="taxReportData.gifts_dotations_lost_coins"
                />
              </q-item-section>
            </q-item>
          </q-list>
          <q-separator />
          <q-card-section>
            Note: This is just an indication of your taxable gains. Download a
            Tax Report to see your every transaction profit and cost.
            <!-- Tax Report to see your short/long-term proceeds, cost-basis,
            disposals and detailed calculations that you can use in your
            official tax returns. -->
          </q-card-section>
        </q-card>
        <q-card>
          <q-card-section>
            <q-banner rounded class="bg-yellow-2">
              <template v-slot:avatar>
                <q-icon name="watch_later" />
              </template>
              Free For Now
            </q-banner>
            <br />
            <q-btn
              color="primary"
              icon="file_download"
              label="Export CSV"
              :loading="walletLoading"
              @click="exportToCSV"
            />
          </q-card-section>
        </q-card>
      </div>
      <div class="col-4 q-pa-md">
        <q-card class="my-card">
          <q-card-section>
            <div class="text-h6">Settings</div>
            <div class="text-caption text-grey">
              These settings are used to calculate your gains. To change any of
              these click here
            </div>
          </q-card-section>
          <q-separator />
          <q-list separator>
            <q-item clickable v-ripple>
              <q-item-section overline>Timezone</q-item-section>
              <q-skeleton v-if="userConfigLoading" type="text" />
              <q-item-section v-else side>{{
                userConfig.time_zone
              }}</q-item-section>
            </q-item>
            <q-item clickable v-ripple>
              <q-item-section overline>Base Currency</q-item-section>
              <q-skeleton v-if="userConfigLoading" type="text" />
              <q-item-section v-else side>{{
                userConfig.base_currency
              }}</q-item-section>
            </q-item>
            <q-item clickable v-ripple>
              <q-item-section overline>Cost basis method</q-item-section>
              <q-skeleton v-if="userConfigLoading" type="text" />
              <q-item-section v-else side>{{
                userConfig.tax_method
              }}</q-item-section>
            </q-item>
            <q-item clickable v-ripple>
              <q-item-section overline>Cost tracking method</q-item-section>
              <q-skeleton v-if="userConfigLoading" type="text" />
              <q-item-section v-else side>Universal</q-item-section>
            </q-item>
          </q-list>
        </q-card>
      </div>
    </div>
  </div>
</template>
<script setup lang="ts">
import { rate } from "@/api/baseCurrencies"
import { getUserConfig, getUserTaxProfit, getUserWalletsTag } from "@/api/user"
import CurrencyUSD from "@/components/CurrencyUSD.vue"
import type { SyncedTransaction } from "@/types/tokens"
import type { TaxReportData, UserConfig } from "@/types/user"
import { YearTimestamp, getYearTimestamps } from "@/utils/date"
import { numberToFixed } from "@/utils/math"
import { getAllSyncedTransactions } from "@/utils/syncedTransactions"
import moment from "moment-timezone"
import { exportFile } from "quasar"
import { computed, onMounted, ref, watch } from "vue"

const walletLoading = ref(false)
const userConfigLoading = ref(false)

// 初始化日期选项数组，包含当前年份以及前三年的年份
const dateOptions: YearTimestamp[] = getYearTimestamps().reverse()
dateOptions.push({ year: "All", timestamp: { start: 0, end: 0 } })
const selectedYear = ref(dateOptions[0])

const historyList = ref<SyncedTransaction[]>([])
const transactionAmount = ref(0)
const walletAmount = ref(0)
const userConfig = ref<UserConfig>({
  base_currency: "USD",
  time_zone: moment.tz.guess(),
  tax_method: "FIFO",
}) //用户选择税务报告所导出的单位为USD还是自选的货币

const taxReportData = ref<TaxReportData>({
  capital_gain_or_loss: 0,
  costs_expenses: 0,
  gifts_dotations_lost_coins: 0,
  income: 0,
  other_gain: 0,
})

onMounted(() => {
  getWalletHistory()
  getTaxProfit()
  getUserConfig().then((res) => {
    if (res) {
      userConfig.value = res
    }
  })
})

const getTaxProfit = async () => {
  walletLoading.value = true
  const res = await getUserTaxProfit(
    selectedYear.value.timestamp.start,
    selectedYear.value.timestamp.end,
  )
  console.log("getTaxProfit", res)
  if (res.Ok) {
    taxReportData.value = res.Ok
  } else {
    taxReportData.value = {
      capital_gain_or_loss: 0,
      costs_expenses: 0,
      gifts_dotations_lost_coins: 0,
      income: 0,
      other_gain: 0,
    }
  }
  walletLoading.value = false
}

const getWalletHistory = async () => {
  const wallets = await getUserWalletsTag()
  walletAmount.value = wallets.length
  //最新的排最上面
  getAllSyncedTransactions(0, 0, ["date-desc"], wallets).then((res) => {
    if (res.total && res.total != 0) {
      historyList.value = res.transactions
      transactionAmount.value = filterData.value.length
    }
  })
}

const filterData = computed((): SyncedTransaction[] => {
  console.log("selectedYear", selectedYear.value)
  let filterData = historyList.value
  // 按选定的日期区间过滤记录
  if (selectedYear.value.timestamp.start !== 0) {
    filterData = filterData.filter(
      (item) =>
        item.timestamp >= Number(selectedYear.value.timestamp.start) &&
        item.timestamp <= Number(selectedYear.value.timestamp.end),
    )
  }
  transactionAmount.value = filterData.length
  return filterData
})

// 监听 selectedYear 变化，如果有任何一个发生变化，强制重新计算 paginatedGroups
watch(selectedYear, () => {
  filterData.value // 触发 filterData 的重新计算
  getTaxProfit()
})

const exportToCSV = async () => {
  const columnNames = [
    "Hash",
    "Type",
    "Status",
    "Timestamp",
    "From",
    "To",
    "Amount",
    "Fee",
    "Token",
    "Currency",
    "Price",
    "Cost",
    "Income",
    "Profit",
  ]
  let exportRate = 1
  if (userConfig.value.base_currency !== "USD") {
    //如果用户选择使用他自己选择的货币汇率，则对即将导出的报告金额使用转换
    exportRate = rate
  }

  // 生成包含列名和数据的数组
  const data = [
    columnNames,
    ...filterData.value.map((transaction) => [
      transaction.hash,
      transaction.t_type,
      transaction.details.status,
      //Time format fixed to Switzerland
      moment(transaction.timestamp).format("DD.MM.YYYY hh:mm:ss"),
      // new Date(Number(transaction.timestamp)).toLocaleString("fr-CH"),
      transaction.details?.from,
      transaction.details?.to,
      transaction.details.amount,
      transaction.details.fee,
      transaction.details.currency.symbol,
      userConfig.value.base_currency,
      numberToFixed(transaction.details.price * exportRate, 2),
      numberToFixed(transaction.details.cost * exportRate, 2),
      numberToFixed(transaction.details.value * exportRate, 2),
      numberToFixed(transaction.details.profit * exportRate, 2),
    ]),
  ]

  console.log("taxreport", data)

  // 将数据转换为 CSV 格式的字符串
  const csvContent = data.map((row) => row.join(",")).join("\n")
  const todayDate = new Date().toLocaleDateString("fr-CH").replace(/\./g, "")
  const fileName = "Tax_Data_All_Wallet"
  // 使用 exportFile 函数导出 CSV 文件
  exportFile(
    `${selectedYear.value.year}_${fileName}_${todayDate}.csv`,
    csvContent,
    "text/csv",
  )
}
</script>

<style lang="scss" scoped>
.taxreport-container {
  .q-skeleton--type-text {
    width: 30px;
  }
}
</style>
@/types/token
