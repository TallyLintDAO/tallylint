<template>
  <div class="market-container">
    <marketNav />
    <marketBanner />
    market
  </div>
</template>

<script lang="ts" setup>
import marketBanner from "./modules/Banner.vue"
import marketNav from "@/components/market/MarketNavigator.vue"
</script>

<style lang="scss">
.market-container {
  min-height: 100vh;
}
</style>
