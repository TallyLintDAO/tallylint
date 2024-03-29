<template>
  <div class="taxreport-container">
    <div class="text-h4 row q-gutter-md items-center">
      <span> Tax Report For </span>
      <q-select
        outlined
        v-model="selectedYear"
        :options="dateOptions"
        emit-value
        map-options
        style="max-width: 200px; font-size: 2.125rem"
      />
    </div>
    <div
      v-if="selectedYear.start !== 0"
      class="text-subtitle2 q-mb-md text-grey"
    >
      {{
        `Jan 1, ${new Date(
          selectedYear.start,
        ).getFullYear()} to Dec 31, ${new Date(
          selectedYear.start,
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
              TaxLint needs your full transaction history (fiat → crypto →
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
              <q-item-section v-else side>$ 0</q-item-section>
            </q-item>
            <q-item clickable v-ripple>
              <q-item-section>
                Other gains (futures, derivatives etc)
              </q-item-section>
              <q-skeleton v-if="walletLoading" type="text" />
              <q-item-section v-else side>$ 0</q-item-section>
            </q-item>
            <q-item clickable v-ripple>
              <q-item-section overline>Income</q-item-section>
              <q-skeleton v-if="walletLoading" type="text" />
              <q-item-section v-else side>$ 0</q-item-section>
            </q-item>
            <q-item clickable v-ripple>
              <q-item-section>
                <q-item-label>Costs & expenses</q-item-label>
                <!-- <q-icon name="warning" /> -->
              </q-item-section>
              <q-skeleton v-if="walletLoading" type="text" />
              <q-item-section v-else side>$ 0</q-item-section>
            </q-item>
            <q-item clickable v-ripple>
              <q-item-section>
                <q-item-label>Gifts, donations & lost coins</q-item-label>
              </q-item-section>
              <q-skeleton v-if="walletLoading" type="text" />
              <q-item-section v-else side>$ 0</q-item-section>
            </q-item>
          </q-list>
          <q-separator />
          <q-card-section>
            Note: This is just an indication of your taxable gains. Download a
            Tax Report to see your short/long-term proceeds, cost-basis,
            disposals and detailed calculations that you can use in your
            official tax returns.
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
              <q-item-section overline>Home Country</q-item-section>
              <q-skeleton v-if="userConfigLoading" type="text" />
              <q-item-section v-else side>None</q-item-section>
            </q-item>
            <q-item clickable v-ripple>
              <q-item-section overline>Base Currency</q-item-section>
              <q-skeleton v-if="userConfigLoading" type="text" />
              <q-item-section v-else side>USD</q-item-section>
            </q-item>
            <q-item clickable v-ripple>
              <q-item-section overline>Cost basis method</q-item-section>
              <q-skeleton v-if="userConfigLoading" type="text" />
              <q-item-section v-else side>FIFO</q-item-section>
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
import { getUserAllWallets, getUserTaxProfit } from "@/api/user"
import type { syncedTransaction } from "@/types/sns"
import { YearTimestamp, getYearTimestamps } from "@/utils/date"
import { getAllSyncedTransactions } from "@/utils/syncedTransactions"
import { exportFile } from "quasar"
import { computed, onMounted, ref, watch } from "vue"

const walletLoading = ref(false)
const userConfigLoading = ref(false)
// 初始化日期选项数组，包含当前年份以及前三年的年份
const dateOptions: YearTimestamp[] = getYearTimestamps().reverse()
dateOptions.push({ label: "All", value: { start: 0, end: 0 } })
const selectedYear = ref(dateOptions[0].value)

const historyList = ref<syncedTransaction[]>([])
const transactionAmount = ref(0)
const walletAmount = ref(0)

onMounted(() => {
  getWalletHistory()
  getTaxProfit()
})

const getTaxProfit = async () => {
  const res = await getUserTaxProfit()
  console.log("getTaxProfit", res)
}

const getWalletHistory = async () => {
  walletLoading.value = true
  const wallets = await getUserAllWallets()
  walletAmount.value = wallets.length
  getAllSyncedTransactions(0, 0, [], wallets)
    .then((res) => {
      if (res.total && res.total != 0) {
        historyList.value = res.transactions
        transactionAmount.value = filterData.value.length
      }
    })
    .finally(() => {
      walletLoading.value = false
    })
}

const filterData = computed((): syncedTransaction[] => {
  let filterData = historyList.value
  // 按选定的日期区间过滤记录
  if (selectedYear.value.start !== 0) {
    filterData = filterData.filter(
      (item) =>
        item.timestamp >= Number(selectedYear.value.start) &&
        item.timestamp <= Number(selectedYear.value.end),
    )
  }
  transactionAmount.value = filterData.length
  return filterData
})

// 监听 selectedYear 变化，如果有任何一个发生变化，强制重新计算 paginatedGroups
watch(selectedYear, () => {
  filterData.value // 触发 filterData 的重新计算
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
    "Memo",
    "Price",
    "Cost",
    "Income",
    "Profit",
  ]

  // 生成包含列名和数据的数组
  const data = [
    columnNames,
    ...filterData.value.map((transaction) => [
      transaction.hash,
      transaction.t_type,
      transaction.details.status,
      //Time format fixed to Switzerland
      new Date(Number(transaction.timestamp)).toLocaleString("fr-CH"),
      transaction.details?.from,
      transaction.details?.to,
      transaction.details.amount,
      transaction.details.fee,
      "",
      transaction.details.price,
      transaction.details.cost,
      transaction.details.value,
      transaction.details.profit,
    ]),
  ]

  console.log("taxreport", data)

  // 将数据转换为 CSV 格式的字符串
  const csvContent = data.map((row) => row.join(",")).join("\n")
  const todayDate = new Date().toLocaleDateString("fr-CH").replace(/\./g, "")
  const fileName = "Tax_Data_All_Wallet"
  // 使用 exportFile 函数导出 CSV 文件
  exportFile(`${todayDate}_${fileName}.csv`, csvContent, "text/csv")
}
</script>

<style lang="scss" scoped>
.taxreport-container {
  .q-skeleton--type-text {
    width: 30px;
  }
}
</style>
