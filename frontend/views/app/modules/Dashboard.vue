<template>
  <div class="dashboard-container">
    DashBoard
    <!-- 为 ECharts 准备一个定义了宽高的 DOM -->
    <div ref="echartsContainer" style="width: 600px; height: 400px"></div>
    <q-card class="my-card" flat bordered>
      <q-item>
        <q-item-section>
          <q-item-label caption> BREAKDOWN </q-item-label>
          <q-list>
            <q-item clickable v-ripple>
              <q-item-section>Received</q-item-section>
            </q-item>
            <q-item clickable v-ripple>
              <q-item-section>Sent</q-item-section>
            </q-item>
            <q-item clickable v-ripple>
              <q-item-section>Realized gains</q-item-section>
            </q-item>
          </q-list>
        </q-item-section>
      </q-item>
    </q-card>
  </div>
</template>

<script lang="ts" setup>
import { getWalletHistory } from "@/api/rosetta"
import { getUserWallet } from "@/api/user"
import { showMessageError } from "@/utils/message"
import * as echarts from "echarts"
import { onMounted, ref } from "vue"

const echartsContainer = ref<null>(null)

onMounted(() => {
  initECharts()
  getWallet()
})

const getWallet = async () => {
  const res = await getUserWallet(false)
  if (res.Ok && res.Ok[0]) {
    const walletAdress = res.Ok[0].address
    const walletHistory = await getWalletHistory(walletAdress)
    console.log("history", walletHistory)
    const timestamps = walletHistory.history.map((record) =>
      new Date(Number(record.timestamp)).toLocaleString(),
    )
    const balances = walletHistory.history.map((record) => record.walletValue)
    // 基于准备好的dom，初始化echarts实例
    var chart = echarts.init(echartsContainer.value)
    chart.hideLoading()
    // 绘制图表
    chart.setOption({
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
      text: "Total Value",
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
