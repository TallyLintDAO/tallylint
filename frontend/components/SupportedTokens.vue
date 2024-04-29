<template>
  <div @click="tokensDialogVisible = true">
    Support Tokens:
    <q-avatar
      color="grey-4 clickable"
      size="40px"
      font-size="12px"
      @click="tokensDialogVisible = true"
    >
      <img src="@/assets/dfinity.svg" />
    </q-avatar>
    <q-avatar
      color="grey-4 clickable"
      class="q-ml-xs"
      size="40px"
      font-size="12px"
      @click="tokensDialogVisible = true"
    >
      +7
    </q-avatar>
  </div>
  <q-dialog v-model="tokensDialogVisible">
    <q-card style="min-width: 400px">
      <q-card-section>
        <div class="text-h6">Manage tokens</div>
      </q-card-section>

      <q-card-section class="q-pt-none">
        <div class="row q-gutter-xs">
          <q-select
            filled
            v-model="network"
            :options="networks"
            label="Network"
            style="min-width: 125px"
          />
          <q-select
            filled
            :loading="tokensLoading"
            v-model="selectedToken"
            :options="tokens"
            label="Selected Token"
            class="col"
          >
            <template v-slot:selected>
              <img
                v-if="selectedToken"
                class="selected-icon"
                :src="SNS_AGGREGATOR_CANISTER_URL + selectedToken.meta.logo"
                alt="Icon"
              />
              {{ selectedToken?.symbol }}
            </template>
            <template v-slot:option="scope">
              <q-item v-bind="scope.itemProps">
                <q-item-section avatar>
                  <img
                    class="head-icon"
                    :src="SNS_AGGREGATOR_CANISTER_URL + scope.opt.meta.logo"
                    alt="Icon"
                  />
                </q-item-section>
                <q-item-section>
                  <q-item-label>{{ scope.opt.symbol }}</q-item-label>
                  <q-item-label caption>{{ scope.opt.name }}</q-item-label>
                </q-item-section>
              </q-item>
            </template>
          </q-select>
        </div>
        Need to manually synchronize transactions again after adding new token
        <div class="row justify-between">
          <q-btn>Manuage</q-btn>
          <q-btn>Add</q-btn>
        </div>
      </q-card-section>

      <q-card-actions align="right">
        <q-btn flat label="OK" color="primary" v-close-popup />
      </q-card-actions>
    </q-card>
  </q-dialog>
</template>

<script setup lang="ts">
import { SNS_AGGREGATOR_CANISTER_URL, getAllSNSInfo } from "@/api/sns"
import type { SnsInfo } from "@/types/sns"
import { onMounted, ref } from "vue"

const tokensDialogVisible = ref(false)
const tokensLoading = ref(true)

const tokens = ref<SnsInfo[]>() //对应网络应该显示不同的币种们
const selectedToken = ref<SnsInfo>()

const networks = ["ICRC-1"]
const network = ref("ICRC-1")

onMounted(() => {
  getICRC1Info()
})

const getICRC1Info = () => {
  tokensLoading.value = true
  getAllSNSInfo().then((snses) => {
    console.log("snses", snses)
    tokens.value = snses
  })
  tokensLoading.value = false
}
</script>

<style lang="scss" scoped></style>
