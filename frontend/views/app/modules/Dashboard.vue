<template>
  <div class="dashboard-container">
    DashBoard
    <!-- 为 ECharts 准备一个定义了宽高的 DOM -->
    <div id="main" style="width: 600px; height: 400px"></div>
  </div>
</template>

<script lang="ts" setup>
import { ref, onMounted } from "vue"
import * as echarts from "echarts"
import { getWalletHistory } from "@/api/rosetta"
import { getUserWallet } from "@/api/user";
import { showMessageError } from "@/utils/message";
onMounted(() => {
  getWallet()
})
const getWallet = async () => {
  const res = await getUserWallet(false);
  if (res.Ok && res.Ok[0]) {
    const walletAdress = res.Ok[0].address;
    const walletHistory = await getWalletHistory(walletAdress)
    console.log("history", walletHistory)
    const timestamps = walletHistory.history.map((record) => new Date(Number(record.timestamp)).toLocaleString());
    const balances = walletHistory.history.map((record) => record.walletValue);
    // 基于准备好的dom，初始化echarts实例
    var walletValueChart = echarts.init(document.getElementById("main"))
    // 绘制图表
    walletValueChart.setOption({
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
        data: timestamps,
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
        { //启用图表下方的进度条，方便拖放
          start: 0,
          end: 100,
        },
      ],
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
    })
    window.addEventListener('resize', () => {
      walletValueChart.resize();
    });
  } else {
    showMessageError('please add wallet for dashboard')
  }
}
</script>

<style lang="scss" scoped></style>
