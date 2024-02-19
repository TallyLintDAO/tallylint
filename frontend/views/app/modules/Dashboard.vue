<template>
  <div class="dashboard-container">
    <!-- 为 ECharts 准备一个定义了宽高的 DOM -->
    <div class="row q-gutter-md">
      <div class="col-7" ref="echartsContainer" style="height: 400px"></div>
      <div class="col-4">
        <div class="q-pa-md" style="max-width: 300px">
          <el-date-picker
            v-model="date"
            type="daterange"
            range-separator="To"
            start-placeholder="Start date"
            end-placeholder="End date"
            :shortcuts="shortcuts"
            value-format="x"
            @change="changeDate()"
            size="large"
          />
        </div>
        <q-card flat bordered>
          <q-item>
            <q-item-section>
              <q-item-label caption> BREAKDOWN </q-item-label>
              <q-list>
                <q-item clickable v-ripple="true">
                  <q-item-section>Received</q-item-section>
                  <q-item-section side>$ {{ received }}</q-item-section>
                </q-item>
                <q-item clickable v-ripple="true">
                  <q-item-section>Sent</q-item-section>
                  <q-item-section side>$ {{ sent }}</q-item-section>
                </q-item>
                <q-item clickable v-ripple="true">
                  <q-item-section>Gains</q-item-section>
                  <q-item-section side>$ {{ gains }}</q-item-section>
                </q-item>
              </q-list>
            </q-item-section>
          </q-item>
        </q-card>
      </div>
    </div>
    <div class="row">
      <q-card flat bordered>
        <q-item>
          <q-item-section>
            <q-item-label caption style="font-size: 1rem">
              Holdings
            </q-item-label>
            <q-table
              :rows="rows"
              :columns="columns"
              row-key="name"
              :rowsPerPageOptions="[0]"
              hide-pagination
              flat
            >
              <template v-slot:header="props">
                <q-tr :props="props">
                  <q-th auto-width />
                  <q-th
                    v-for="col in props.cols"
                    :key="col.name"
                    :props="props"
                  >
                    {{ col.label }}
                  </q-th>
                </q-tr>
              </template>

              <template v-slot:body="props">
                <q-tr :props="props">
                  <q-td auto-width>
                    <q-btn
                      size="sm"
                      color="primary"
                      round
                      dense
                      @click="props.expand = !props.expand"
                      :icon="props.expand ? 'remove' : 'add'"
                    />
                  </q-td>
                  <q-td
                    v-for="col in props.cols"
                    :key="col.name"
                    :props="props"
                  >
                    {{ col.value }}
                  </q-td>
                </q-tr>
                <q-tr v-show="props.expand" :props="props" no-hover>
                  <q-td colspan="100%">
                    <div class="text-left">
                      <Progress
                        :wallets="wallets"
                        :symbol="props.row.token"
                        :price="props.row.price"
                        :totalBalance="icpBalance"
                      />
                    </div>
                  </q-td>
                </q-tr>
              </template>
            </q-table>
          </q-item-section>
        </q-item>
      </q-card>
    </div>
  </div>
</template>

<script lang="ts" setup>
import {
  getAllWalletDailyBalance,
  getDailyBalanceValue,
  getICPBalance,
  getWalletHistory,
} from "@/api/rosetta"
import { getICPNowPrice } from "@/api/token"
import { getUserWallet } from "@/api/user"
import Progress from "@/components/Progress.vue"
import type { TableColumn } from "@/types/model"
import type { Wallet, WalletHistory } from "@/types/user"
import { showMessageError } from "@/utils/message"
import type { EChartsType } from "echarts"
import * as echarts from "echarts"
import { onMounted, ref, watch } from "vue"

const echartsContainer = ref<null>(null)
let chart = <EChartsType>{} // 如果使用ref会导致无法显示tooltip，例如数据不会显示。
const date = ref("")
const totalHistory = ref<WalletHistory[]>([])
const received = ref(0)
const sent = ref(0)
const gains = ref(0)

const shortcuts = [
  {
    text: "Last 12 months",
    value: () => {
      const end = new Date()
      const start = new Date()
      start.setMonth(end.getMonth() - 12)
      return [start, end]
    },
  },
  {
    text: new Date().getFullYear().toString(),
    value: () => {
      const end = new Date()
      const start = new Date(end.getFullYear(), 0, 1)
      return [start, end]
    },
  },
  {
    text: (new Date().getFullYear() - 1).toString(),
    value: () => {
      const end = new Date(new Date().getFullYear() - 1, 11, 31)
      const start = new Date(new Date().getFullYear() - 1, 0, 1)
      return [start, end]
    },
  },
  {
    text: (new Date().getFullYear() - 2).toString(),
    value: () => {
      const end = new Date(new Date().getFullYear() - 2, 11, 31)
      const start = new Date(new Date().getFullYear() - 2, 0, 1)
      return [start, end]
    },
  },
]

const columns: TableColumn[] = [
  {
    name: "token",
    required: true,
    label: "Tokens",
    field: "token",
    align: "left",
  },
  {
    name: "balance",
    required: true,
    label: "Balance",
    sortable: true,
    field: "balance",
    align: "left",
  },
  // {
  //   name: "cost",
  //   required: true,
  //   label: "Cost",
  //   sortable: true,
  //   field: "cost",
  //   align: "left",
  // },
  {
    name: "price",
    required: true,
    label: "Price",
    sortable: true,
    field: "price",
    align: "left",
  },
  {
    name: "value",
    required: true,
    label: "Value",
    sortable: true,
    field: "value",
    align: "left",
  },
]

const wallets = ref<Wallet[]>([])
const icpBalance = ref(0)
const rows = ref<any[]>([
  {
    token: "ICP",
    balance: 0,
    cost: 0,
    price: 0,
    value: 0,
  },
])

onMounted(() => {
  initECharts()
  getWallet()
  getICPPrice()
  // getAllSNSInfo().then((snses) => {
  //   const sns = snses.find((sns) => sns.symbol === "CHAT")
  //   if (sns) {
  //     // getICRC1Price(sns.canisters.ledger)
  //     getAllTransactionsICRC1(
  //       {
  //         address:
  //           "bcc6t-arcy7-qgvxt-v3ubw-6xndu-ld6nf-vwetx-so4q3-pyjlv-3udyi-nae",
  //         name: "",
  //         from: "",
  //       },
  //       sns.canisters.index,
  //       sns.canisters.ledger,
  //       { decimals: sns.decimals, symbol: sns.symbol },
  //     )
  //   }
  // })
})

const getBalance = async (address: string, walletName: string) => {
  //获取用户当前钱包资产
  const balance = await getICPBalance(address)
  console.log(address + " balance: ", balance)
  wallets.value.push({
    address: address,
    name: walletName,
    tokens: [{ symbol: "ICP", balance: balance }],
  })
}

watch(
  () => wallets.value.length,
  () => {
    console.log("watch", wallets.value)
    icpBalance.value = wallets.value.reduce(
      // token[0] 目前暂为ICP
      (total, wallet) => total + wallet.tokens[0].balance,
      0,
    )
    rows.value[0].balance = icpBalance.value.toFixed(8)
    rows.value[0].value = (rows.value[0].balance * rows.value[0].price).toFixed(
      2,
    )
    console.log("icpBalance", icpBalance.value)
    console.log("rows", rows.value)
  },
)

const getICPPrice = () => {
  getICPNowPrice().then((res) => {
    rows.value[0].price = res
  })
}

const getWallet = async () => {
  const res = await getUserWallet(false)
  if (res.Ok && res.Ok[0]) {
    //清空钱包，以免出现重复的问题
    wallets.value.length = 0
    for (const walletInfo of res.Ok) {
      //TODO 这里作为BREAKDOWN的数值，有bug，多个钱包的资产总值没有计算，而下面的echarts图表没有bug，已经计算了。
      //将用户的每个钱包地址下的交易记录查出来，并总和到一起
      const walletHistory = await getWalletHistory(walletInfo.address)
      getBalance(walletInfo.address, walletInfo.name)
      totalHistory.value = totalHistory.value.concat(walletHistory.history)
    }
    const walletDailyBalance = await getAllWalletDailyBalance(res.Ok)
    let timestamps = Object.keys(walletDailyBalance).sort()
    const balances = await getDailyBalanceValue(walletDailyBalance)
    // 按选定的日期区间过滤记录
    if (date.value) {
      totalHistory.value = totalHistory.value.filter(
        (item) =>
          item.timestamp >= Number(date.value[0]) &&
          item.timestamp <= Number(date.value[1]),
      )
      //为echarts图表过滤
      timestamps = timestamps.filter((timestamp) => {
        const currentDate = Number(new Date(timestamp))
        return (
          currentDate >= Number(date.value[0]) &&
          currentDate <= Number(date.value[1])
        )
      })
    }
    getDetail()
    // 基于准备好的dom，初始化echarts实例

    chart.hideLoading()
    // 绘制图表
    chart.setOption({
      title: {
        text: `Synchronized Total History (${res.Ok.length} wallets)`,
      },
      series: [
        {
          name: "Value",
          type: "line",
          symbol: "none",
          smooth: true,
          sampling: "lttb",
          itemStyle: {
            color: "rgb(255, 70, 131)",
          },
          areaStyle: {
            color: new echarts.graphic.LinearGradient(0, 0, 0, 1, [
              {
                offset: 0,
                color: "rgb(255, 158, 68)",
              },
              {
                offset: 1,
                color: "rgb(255, 70, 131)",
              },
            ]),
          },
          data: balances,
        },
      ],
      xAxis: { data: timestamps },
    })
  } else {
    showMessageError("please add wallet for dashboard")
  }
}
const getDetail = () => {
  if (totalHistory.value) {
    totalHistory.value.forEach((transaction) => {
      if (transaction.t_type === "RECEIVE") {
        received.value += transaction.amount * transaction.price
      } else if (transaction.t_type === "SEND") {
        sent.value += transaction.amount * transaction.price
      }
    })
    //保留小数点
    received.value = Number(received.value.toFixed(2))
    sent.value = Number(sent.value.toFixed(2))
    //gain暂且使用这种简单的方式计算
    gains.value = Number((received.value - sent.value).toFixed(2))
  }
}
// 初始化 ECharts 实例
const initECharts = () => {
  chart = echarts.init(echartsContainer.value)
  // 配置图表选项
  const option = {
    tooltip: {
      trigger: "axis",
      position: function (pt) {
        return [pt[0], "10%"]
      },
    },
    title: {
      left: "center",
      text: "Total History (0 wallets)",
    },
    toolbox: {
      feature: {
        dataZoom: {
          yAxisIndex: "none",
        },
        saveAsImage: {},
      },
    },
    xAxis: {
      type: "category",
      boundaryGap: false,
      data: [],
    },
    yAxis: {
      type: "value",
      boundaryGap: false, //是否留白
    },
    dataZoom: [
      {
        type: "inside",
        start: 0,
        end: 100,
      },
      {
        //启用图表下方的进度条，方便拖放
        start: 0,
        end: 100,
      },
    ],
  }

  // 使用 setOption 方法将配置应用到图表
  chart.setOption(option)
  chart.showLoading()
}
const changeDate = () => {
  console.log("date", date.value)
  chart.showLoading()
  getWallet()
}
</script>

<style lang="scss" scoped></style>
