<template>
  <div>
    <div class="progress-bar">
      <div
        v-for="(wallet, index) in computedBalancePercent"
        :key="index"
        :style="{
          width: wallet.percentage + '%',
          backgroundColor: getBackgroundColor(index),
        }"
        class="bar"
      ></div>
    </div>
    <q-table
      title="Holding"
      :rows="computedBalancePercent"
      :columns="columns"
      row-key="name"
    />
  </div>
</template>

<script lang="ts" setup>
import type { Wallet } from "@/types/user"
import { calculatePercent } from "@/utils/math"
import { number } from "echarts"
import { computed, ref } from "vue"
const props = defineProps({
  wallets: {
    type: Array as () => Wallet[], // 指定类型为数组，且数组元素类型为 Wallet
    required: true,
  },
  symbol: {
    type: String,
    required: true,
  },
  totalBalance: {
    type: Number,
    required: true,
  },
})
// const totalBalance = ref(0)
const columns = [
  {
    name: "name",
    required: true,
    label: "Name",
    field: "name",
  },
  {
    name: "balance",
    required: true,
    label: "Balance",
    field: "balance",
  },
  {
    name: "value",
    required: true,
    label: "Value",
    field: "value",
  },
  {
    name: "percentage",
    required: true,
    label: "Allocation",
    field: (row) => row.percentage + "%",
  },
]

const colors = ["#3498db", "#e74c3c", "#2ecc71", "#f39c12", "#9b59b6"]
const getBackgroundColor = (index) => {
  return colors[index % colors.length]
}

//计算每个钱包对应的币种占比
const computedBalancePercent = computed(() => {
  const res = props.wallets.map((wallet) => {
    const token = wallet.tokens.find((t) => t.symbol === props.symbol)
    const balance = token ? token.balance : 0
    // totalBalance.value += balance

    const percentage = calculatePercent(balance, props.totalBalance)
    return { name: wallet.name, balance, percentage }
  })
  console.log("computedBalancePercent", res)
  console.log("totalBalance", props.totalBalance)
  return res
})

// watch(
//   () => props.wallets.length,
//   () => {
//     countBalancePercent()
//   },
// )
</script>

<style>
.progress-bar {
  height: 12px;
  width: 100%;
  background-color: #ecf0f1;
  margin-bottom: 10px;
  border-radius: 0.5rem;
  overflow: hidden;
}

.bar {
  height: 100%;
  vertical-align: top;
  display: inline-block;
}

.label {
  margin-top: 5px;
  text-align: center;
  color: #333;
}
</style>
