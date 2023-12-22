<template>
  <div>
    <div class="progress-bar">
      <div
        v-for="(balance, walletName, index) in wallets"
        :key="walletName"
        :style="{
          width: balance + '%',
          backgroundColor: getBackgroundColor(index),
        }"
        class="bar"
      ></div>
    </div>
    <div
      v-for="(balance, walletName) in wallets"
      :key="walletName"
      class="label"
    >
      {{ walletName }}: {{ balance }}%
    </div>
  </div>
</template>

<script lang="ts" setup>
import { ref } from "vue"

const wallets = ref({
  Wallet1: 50,
  Wallet2: 30,
  Wallet3: 20,
})
const colors = ["#3498db", "#e74c3c", "#2ecc71", "#f39c12", "#9b59b6"]
const getBackgroundColor = (index) => {
  return colors[index % colors.length]
}
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
