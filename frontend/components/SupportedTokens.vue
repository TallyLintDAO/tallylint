<template>
  <div>
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
      <q-card-section class="row items-center">
        <div class="text-h6">Manage tokens</div>
        <q-space />
        <q-btn icon="close" flat round dense v-close-popup />
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
                class="selected-icon q-mr-xs"
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
        <span class="text-caption" style="color: rgba(0, 0, 0, 0.54)">
          Need to click 'Sync All Wallets' again after adding new token
        </span>
        <div class="row q-mt-sm">
          <q-space />
          <q-btn @click="addSelectedToken()">Add</q-btn>
        </div>
      </q-card-section>
      <q-card-section class="token-list">
        <q-list>
          <q-item>
            <!-- 单独整个ICP看起来好看点。实际上没有任何用 -->
            <q-item-section avatar>
              <q-avatar
                color="grey-4"
                size="40px"
                font-size="12px"
                class="clickable"
                @click="
                  jumpToWebsite('https://dashboard.internetcomputer.org/')
                "
              >
                <img src="@/assets/dfinity.svg" />
              </q-avatar>
            </q-item-section>
            <q-item-section>
              <q-item-label>Internet Computer</q-item-label>
              <q-item-label caption>ICP</q-item-label>
            </q-item-section>
          </q-item>
          <q-item v-for="(token, index) in addedTokenList">
            <!-- 遍历已添加的token -->
            <q-item-section avatar>
              <q-avatar
                color="grey-4"
                size="40px"
                font-size="12px"
                class="clickable"
                @click="jumpToWebsite(token.meta.url)"
              >
                <img :src="SNS_AGGREGATOR_CANISTER_URL + token.meta.logo" />
              </q-avatar>
            </q-item-section>
            <q-item-section>
              <q-item-label>{{ token.name }}</q-item-label>
              <q-item-label caption>{{ token.symbol }}</q-item-label>
            </q-item-section>
            <q-item-section side>
              <q-btn
                icon="delete_outline"
                flat
                round
                dense
                @click="removeToken(index)"
              />
            </q-item-section>
          </q-item>
        </q-list>
      </q-card-section>

      <q-card-actions align="right">
        <q-btn flat label="Close" color="primary" v-close-popup />
      </q-card-actions>
    </q-card>
  </q-dialog>
</template>

<script setup lang="ts">
import { SNS_AGGREGATOR_CANISTER_URL, getAllSNSInfo } from "@/api/sns"
import type { ICRC1Info } from "@/types/sns"
import { onMounted, ref } from "vue"

const tokensDialogVisible = ref(false)
const tokensLoading = ref(true)

const tokens = ref<ICRC1Info[]>() //对应网络应该显示不同的币种们
const selectedToken = ref<ICRC1Info>()
const addedTokenList = ref<ICRC1Info[]>([])

const networks = ["ICRC-1"]
const network = ref("ICRC-1")

onMounted(() => {
  getICRC1Info()
})

const getICRC1Info = () => {
  tokensLoading.value = true
  getAllSNSInfo()
    .then((snses) => {
      console.log("snses", snses)
      tokens.value = snses
    })
    .finally(() => {
      tokensLoading.value = false
    })
}

const addSelectedToken = () => {
  if (selectedToken.value) {
    addedTokenList.value.push(selectedToken.value)
  }
}
const jumpToWebsite = (url: string) => {
  window.open(url, "_blank")
}
const removeToken = (index) => {
  addedTokenList.value.slice(index, 1)
}
</script>

<style lang="scss" scoped>
.token-list {
  padding: 0 !important;
}
</style>
