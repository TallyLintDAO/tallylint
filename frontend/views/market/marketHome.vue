<template>
  <div class="market-container">
    <marketNav />
    <marketBanner />
    market
  </div>
</template>

<script lang="ts" setup>
import marketBanner from "./modules/banner.vue"
import marketNav from "./modules/marketNavigator.vue"
</script>

<style lang="scss">
.market-container {
  min-height: 100vh;
}
</style>
