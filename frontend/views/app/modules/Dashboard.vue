<template>
  <div class="dashboard-container">
    DashBoard
    <!-- 为 ECharts 准备一个定义了宽高的 DOM -->
    <div class="row q-gutter-md">
      <div class="col-7" ref="echartsContainer" style="height: 400px"></div>
      <div class="col-4">
        <div class="q-pa-md" style="max-width: 300px">
          <q-input filled v-model="date" mask="date" :rules="['date']">
            <template v-slot:append>
              <q-icon name="event" class="cursor-pointer">
                <q-popup-proxy
                  ref="qDateProxy"
                  cover
                  transition-show="scale"
                  transition-hide="scale"
                >
                  <q-date v-model="date">
                    <div class="row items-center justify-end">
                      <q-btn
                        v-close-popup="true"
                        label="Close"
                        color="primary"
                        flat
                      />
                    </div>
                  </q-date>
                </q-popup-proxy>
              </q-icon>
            </template>
          </q-input>
        </div>
        <q-card flat bordered>
          <q-item>
            <q-item-section>
              <q-item-label caption> BREAKDOWN </q-item-label>
              <q-list>
                <q-item clickable v-ripple="true">
                  <q-item-section>Received</q-item-section>
                  <q-item-section side>0</q-item-section>
                </q-item>
                <q-item clickable v-ripple="true">
                  <q-item-section>Sent</q-item-section>
                  <q-item-section side>0</q-item-section>
                </q-item>
                <q-item clickable v-ripple="true">
                  <q-item-section>Realized gains</q-item-section>
                  <q-item-section side>0</q-item-section>
                </q-item>
              </q-list>
            </q-item-section>
          </q-item>
        </q-card>
      </div>
    </div>
  </div>
</template>

<script lang="ts" setup>
import { getWalletHistory } from "@/api/rosetta"
import { getUserWallet } from "@/api/user"
import type { WalletHistory } from "@/types/user"
import { showMessageError } from "@/utils/message"
import * as echarts from "echarts"
import { onMounted, ref } from "vue"

const echartsContainer = ref<null>(null)
const date = ref("2023/01/01")

onMounted(() => {
  initECharts()
  getWallet()
})

const getWallet = async () => {
  const res = await getUserWallet(false)
  if (res.Ok && res.Ok[0]) {
    let totalHistory: Array<WalletHistory> = []
    for (const walletInfo of res.Ok) {
      //将用户的每个钱包地址下的交易记录查出来，并总和到一起
      const walletHistory = await getWalletHistory(walletInfo.address)
      totalHistory = totalHistory.concat(walletHistory.history)
    }
    // 按时间戳排序交易记录数组
    totalHistory.sort((a, b) => a.timestamp - b.timestamp)
    console.log("totalHistory", totalHistory)
    const timestamps = totalHistory.map((record) =>
      new Date(Number(record.timestamp)).toLocaleString(),
    )
    const balances = totalHistory.map((record) => record.walletValue)
    // 基于准备好的dom，初始化echarts实例
    var chart = echarts.init(echartsContainer.value)
    chart.hideLoading()
    // 绘制图表
    chart.setOption({
      title: {
        text: `Total History (${res.Ok.length} wallets)`,
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
// 初始化 ECharts 实例
const initECharts = () => {
  const chart = echarts.init(echartsContainer.value)
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
        restore: {},
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
</script>

<style lang="scss" scoped></style>
