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
      :rows="computedBalancePercent"
      :columns="columns"
      row-key="name"
      :rowsPerPageOptions="[0]"
      hide-pagination
      flat
    >
      <template v-slot:body-cell-percentage="props">
        <q-td :props="props" class="flex-y-center">
          <div>
            {{ props.value }}
          </div>
          <div
            :style="{
              color: getBackgroundColor(props.rowIndex),
              'background-color': getBackgroundColor(props.rowIndex),
            }"
            class="rounded-borders q-ml-sm"
            style="width: 15px; height: 15px; display: inline-block"
          ></div>
        </q-td>
      </template>
    </q-table>
  </div>
</template>

<script lang="ts" setup>
import type { TableColumn } from "@/types/model"
import type { Wallet } from "@/types/user"
import { convertCurrency } from "@/utils/currencies"
import { calculatePercent } from "@/utils/math"
import { computed } from "vue"
const props = defineProps({
  wallets: {
    type: Array as () => Wallet[], // 指定类型为数组，且数组元素类型为 Wallet
    required: true,
  },
  symbol: {
    type: String,
    required: true,
  },
  price: {
    type: Number,
    required: true,
  },
  totalBalance: {
    type: Number,
    required: true,
  },
})
const columns: TableColumn[] = [
  {
    name: "name",
    required: true,
    label: "Name",
    field: "name",
    align: "left",
  },
  {
    name: "balance",
    required: true,
    // sortable: true, 排序会导致颜色匹配出现问题，先不用
    label: "Balance",
    field: "balance",
    align: "left",
  },
  {
    name: "value",
    required: true,
    // sortable: true,
    label: "Value",
    field: "value",
    align: "left",
  },
  {
    name: "percentage",
    required: true,
    // sortable: true,
    label: "Allocation",
    field: (row) => row.percentage + "%",
    align: "left",
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
    const value = convertCurrency(balance * props.price)
    const percentage = calculatePercent(balance, props.totalBalance)
    return { name: wallet.name, balance, value, percentage }
  })
  return res
})
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
