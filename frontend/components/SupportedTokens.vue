<template>
  <div>
    <q-avatar color="grey-4 clickable" size="40px" font-size="12px">
      <img src="@/assets/dfinity.svg" />
    </q-avatar>
    <q-avatar
      color="grey-4 clickable"
      class="q-ml-xs"
      size="40px"
      font-size="12px"
    >
      +7
    </q-avatar>
  </div>
</template>

<script setup lang="ts">
import { ref } from "vue"

const tokens = ref()
</script>

<style lang="scss" scoped></style>
