<template>
  <div class="settings-container">
    <div class="row">
      <div class="col-md-12 col-lg-6">
        <q-card class="my-card">
          <q-card-section>
            <div class="text-h6">Portfolio settings</div>
          </q-card-section>

          <q-card-section class="q-pt-none">
            Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do
            eiusmod tempor incididunt ut labore et dolore magna aliqua.
          </q-card-section>
        </q-card>
      </div>
    </div>
  </div>
</template>

<script lang="ts" setup>
import { ref } from "vue"

const tab = ref("mails")
const splitterModel = ref(20)
</script>

<style lang="scss">
.settings-container {
}
</style>
