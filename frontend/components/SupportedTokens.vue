<template>
  <div>
    <q-avatar
      color="grey-4 clickable"
      class="q-ml-xs"
      size="40px"
      font-size="12px"
      @click="tokensDialogVisible = true"
      v-for="(token, index) in addedTokenList.slice(
        0,
        Math.min(addedTokenList.length, 5),
      )"
      :key="index"
    >
      <img :src="token.meta.logo" />
    </q-avatar>
    <q-avatar
      color="grey-4 clickable"
      class="q-ml-xs"
      size="40px"
      font-size="16px"
      @click="tokensDialogVisible = true"
    >
      +{{ addedTokenList.length > 5 ? addedTokenList.length - 5 : "" }}
    </q-avatar>
  </div>
  <q-dialog v-model="tokensDialogVisible">
    <q-card style="min-width: 400px">
      <div v-if="currentPage === 'tokenListPage'">
        <q-card-section class="row items-center">
          <div class="text-h6">Manage tokens</div>
          <q-space />
          <q-btn icon="close" flat round dense v-close-popup />
        </q-card-section>
        <q-card-section class="token-list">
          <q-banner
            inline-actions
            class="bg-primary text-white"
            v-if="isSyncNewToken"
          >
            New tokens in your wallet have been automatically imported for you.
            <template v-slot:action>
              <q-btn
                flat
                color="white"
                @click="autoImportTokens(false)"
                label="Disable"
              />
            </template>
          </q-banner>
          <q-card-section class="select-token">
            <div class="row q-gutter-xs">
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
                    :src="selectedToken.meta.logo"
                    alt="Icon"
                  />
                  {{ selectedToken?.symbol }}
                </template>
                <template v-slot:option="scope">
                  <q-item v-bind="scope.itemProps">
                    <q-item-section avatar>
                      <img
                        class="head-icon"
                        :src="scope.opt.meta.logo"
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
              <q-btn
                style="margin: 10px 0 10px 10px"
                @click="addSelectedToken()"
                >Add</q-btn
              >
            </div>
            <span class="text-caption" style="color: rgba(0, 0, 0, 0.54)">
              Need to click 'Sync All Transactions' again after adding new token
            </span>
            <div class="row q-mt-sm q-gutter-sm">
              <q-space />
              <q-btn
                v-if="!enableSyncNewToken"
                color="primary"
                @click="autoImportTokens(true)"
                >Enable automatic import</q-btn
              >
              <q-btn
                color="primary"
                label="Import"
                @click="currentPage = 'importPage'"
              />
            </div>
          </q-card-section>
          <q-list>
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
                  <img :src="token.meta.logo" />
                </q-avatar>
              </q-item-section>
              <q-item-section>
                <q-item-label>{{ token.name }}</q-item-label>
                <q-item-label caption>{{ token.symbol }}</q-item-label>
              </q-item-section>
              <q-item-section side>
                <q-btn
                  icon="delete_outline"
                  v-if="token.symbol !== 'ICP'"
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
          <q-btn
            color="secondary"
            @click="method()"
            :loading="loading"
            icon="cached"
            >Sync All Transactions</q-btn
          >
        </q-card-actions>
      </div>
      <div v-else-if="currentPage === 'importPage'">
        <q-card-section class="row items-center">
          <div class="text-h6">Import token</div>
          <q-space />
          <q-btn icon="close" flat round dense v-close-popup />
        </q-card-section>
        <q-card-section class="token-import">
          <div class="row q-gutter-xs">
            <q-select
              filled
              v-model="network"
              :options="networks"
              label="Network"
              style="min-width: 125px"
            />
            <q-input
              filled
              v-model="importTokenLedgerId"
              label="Token Ledger Canister Id"
            />
          </div>
        </q-card-section>
        <q-card-actions align="right">
          <q-btn
            flat
            label="Back"
            color="primary"
            @click="currentPage = 'tokenListPage'"
          />
          <q-btn
            color="secondary"
            :loading="importLoading"
            icon="download"
            @click="ImportDIYToken()"
            >Import Token</q-btn
          >
        </q-card-actions>
      </div>
    </q-card>
  </q-dialog>
</template>

<script setup lang="ts">
import { queryIcUserNewAssetsListWithoutICP } from "@/api/icrc1"
import { getSNSInfoCache } from "@/api/sns"
import type { ICRC1Info } from "@/types/tokens"
import type { WalletInfo } from "@/types/user"
import { showMessageError } from "@/utils/message"
import {
  getStorage,
  getTokenList,
  setStorage,
  setTokenList,
} from "@/utils/storage"
import { onMounted, ref, watch } from "vue"

const props = defineProps<{
  method: () => void | Promise<void>
  userWallets: WalletInfo[]
  loading: boolean
}>()
const tokensDialogVisible = ref(false)
const tokensLoading = ref(true)
const importLoading = ref(false) //用户导入自定义代币的loading
const isSyncNewToken = ref(false) // 是否存在未同步的新代币，如果是则检查新钱包里包含的代币并导入
const enableSyncNewToken = ref(true) //是否启用自动导入新代币列表的功能

const importTokenLedgerId = ref() //导入代币的ledger canister id
const currentPage = ref<"tokenListPage" | "importPage">("tokenListPage")
const networks = ["ICRC-1"]
const network = ref("ICRC-1")

const tokens = ref<ICRC1Info[]>() //可选择的token列表，对应网络应该显示不同的币种们
const selectedToken = ref<ICRC1Info>() //用户选择的准备添加的单个代币
const addedTokenList = ref<ICRC1Info[]>([]) // 已添加的代币列表
const checkedNewTokenList = ref<ICRC1Info[]>([]) //检查出来的未同步新代币列表

onMounted(() => {
  getICRC1Info()
  init()
})

watch(
  [() => props.userWallets, () => tokens.value], // 同时监听 props.userWallets 和 tokens 的变化
  (newValue, oldValue) => {
    if (newValue.length > 0) {
      queryUserNewAssets()
    }
  },
)

const ImportDIYToken = () => {
  importLoading.value = true
  //先查重
  //再调用canister对应方法
}

//是否限制代币自动导入功能
const autoImportTokens = (enable: boolean) => {
  enableSyncNewToken.value = enable
  isSyncNewToken.value = false //要禁用此功能的用户应该也不喜欢看到这条横幅
  setStorage("EnableSyncNewToken", enableSyncNewToken.value)
}
//查询是否有尚未同步的代币
const queryUserNewAssets = async () => {
  //用户禁止自动导入新代币
  if (!enableSyncNewToken.value) {
    return
  }
  // 处理新导入的钱包
  if (props.userWallets.length > 0 && tokens.value) {
    // 遍历新导入的钱包并调用 queryUserNewAssets 收集代币
    checkedNewTokenList.value = await queryIcUserNewAssetsListWithoutICP(
      tokens.value,
      props.userWallets,
    )
    if (checkedNewTokenList.value.length > 0) {
      //只有检查到新代币才显示那个页面
      isSyncNewToken.value = true
    }
    //将新代币导入旧代币列表并保存
    addedTokenList.value = addedTokenList.value.concat(
      checkedNewTokenList.value,
    )
    setTokenList(addedTokenList.value)
  } else {
    console.log("No new wallets detected.")
  }
}

const openDialog = () => {
  tokensDialogVisible.value = true
}

const getICRC1Info = () => {
  tokensLoading.value = true
  getSNSInfoCache()
    .then((snses) => {
      tokens.value = snses
    })
    .finally(() => {
      tokensLoading.value = false
    })
}

const addSelectedToken = () => {
  if (
    selectedToken.value &&
    !addedTokenList.value.includes(selectedToken.value)
  ) {
    addedTokenList.value.push(selectedToken.value)
    setTokenList(addedTokenList.value)
  } else {
    showMessageError("The token to be added is empty or already exists")
  }
}
const jumpToWebsite = (url: string) => {
  window.open(url, "_blank")
}
const removeToken = (index: number) => {
  addedTokenList.value.splice(index, 1)
  setTokenList(addedTokenList.value)
}

const init = () => {
  //如果返回值为空则为true
  enableSyncNewToken.value = getStorage("EnableSyncNewToken") ?? true
  const tokenList = getTokenList()
  if (tokenList !== null) {
    addedTokenList.value = tokenList
  }
}
defineExpose({
  openDialog,
})
</script>

<style lang="scss" scoped>
.token-list {
  padding: 0 !important;
}
</style>
