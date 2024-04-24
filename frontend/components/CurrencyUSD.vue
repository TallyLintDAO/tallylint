<template>
  <span>
    {{ convertCurrency(amountConvert) }}
    <q-tooltip>
      ${{ props.amount }}, currency rates from exchangerate-api.com, today's
      prices only
    </q-tooltip>
  </span>
</template>

<script lang="ts" setup>
import { getUserCurrencyRate } from "@/api/baseCurrencies"
import { convertCurrency } from "@/utils/currencies"
import { numberToFixed } from "@/utils/math"
import { onMounted, ref, watch } from "vue"
const props = defineProps({
  amount: {
    type: Number,
    required: true,
  },
})
const amountConvert = ref(0)
onMounted(async () => {
  const rate = (await getUserCurrencyRate()).rate || 1
  amountConvert.value = numberToFixed(props.amount * rate, 2)
})
watch(
  () => props.amount,
  async (newVal) => {
    const rate = (await getUserCurrencyRate()).rate || 1
    amountConvert.value = numberToFixed(props.amount * rate, 2)
    console.log("amountConvert.value", amountConvert.value)
  },
)
</script>

<style></style>
