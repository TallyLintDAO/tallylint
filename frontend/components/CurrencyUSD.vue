<template>
  <span>
    {{ convertCurrency(amountConvert) }}
    <q-tooltip v-if="rate_time_last_update !== 0">
      ${{ props.amount }}
      <br />
      Currency exchange rate date:
      {{ showCustomTimezoneDate(rate_time_last_update) }}
      <br />
      from exchangerate-api.com
    </q-tooltip>
  </span>
</template>

<script lang="ts" setup>
import { rate, rate_time_last_update } from "@/api/baseCurrencies"
import { convertCurrency } from "@/utils/currencies"
import { showCustomTimezoneDate } from "@/utils/date"
import { numberToFixed } from "@/utils/math"
import { onMounted, ref } from "vue"
const props = defineProps({
  amount: {
    type: Number,
    required: true,
  },
})
const amountConvert = ref(0)
onMounted(async () => {
  amountConvert.value = numberToFixed(props.amount * rate, 2)
})
// watch好像不需要，初始化的时候赋值已经足够
// watch(
//   () => props.amount,
//   async (newVal) => {
//     amountConvert.value = numberToFixed(props.amount * rate, 2)
//     console.log("amountConvert.value", amountConvert.value)
//   },
// )
</script>

<style></style>
