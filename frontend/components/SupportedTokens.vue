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
            <q-btn style="margin: 10px 0 10px 10px" @click="addSelectedToken()"
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
              flat
              no-caps
              color="primary"
              label="+ Don’t see your token? Import"
              @click="currentPage = 'importPage'"
            />
          </div>
        </q-card-section>
        <q-card-actions align="between">
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
          <q-form ref="formRef" @submit="ImportDIYToken()">
            <q-select
              filled
              v-model="network"
              :options="networks"
              label="Network"
              class="q-field--with-bottom"
            />
            <div class="text-body1">
              TallyLint requires an index canister id to get the history of
              transactions. But some tokens may not have an index canister. If
              the wrong canister id is provided, then the function may not work
              properly
              <br />
              <div class="q-py-md">
                <a
                  @click="
                    jumpToWebsite(
                      'https://internetcomputer.org/docs/current/developer-docs/daos/nns/using-the-nns-dapp/nns-dapp-importing-tokens',
                    )
                  "
                  color="primary"
                  class="text-weight-bold"
                >
                  <q-icon name="open_in_new" size="sm"></q-icon>
                  Read how ICRC token Integration works
                </a>
              </div>
            </div>
            <q-input
              filled
              v-model="tokenLedgerId"
              label="Token Ledger Canister Id *"
              lazy-rules
              :rules="[
                (val: string) => (val && val.length > 0) || 'Please input ledger canister id',
              ]"
              class="col"
            />
            <q-input
              filled
              v-model="tokenIndexId"
              label="Token Index Canister Id *"
              :rules="[
                (val: string) => (val && val.length > 0) || 'Please input index canister id',
              ]"
              class="col"
            />
          </q-form>
        </q-card-section>
        <q-card-actions align="between">
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
import { getDIYToken, queryIcUserNewAssetsListWithoutICP } from "@/api/icrc1"
import { getSNSInfoCache } from "@/api/sns"
import type { ICRC1Info } from "@/types/tokens"
import type { WalletInfo } from "@/types/user"
import { showMessageError, showMessageSuccess } from "@/utils/message"
import {
  getStorage,
  getTokenList,
  setStorage,
  setTokenList,
} from "@/utils/storage"
import type { QForm } from "quasar"
import { onMounted, ref, watch } from "vue"

const props = defineProps<{
  method: () => void | Promise<void>
  userWallets: WalletInfo[]
  loading: boolean
}>()
const formRef = ref<QForm | null>(null)
const tokensDialogVisible = ref(false)
const tokensLoading = ref(true)
const importLoading = ref(false) //用户导入自定义代币的loading
const isSyncNewToken = ref(false) // 是否存在未同步的新代币，如果是则检查新钱包里包含的代币并导入
const enableSyncNewToken = ref(true) //是否启用自动导入新代币列表的功能

const tokenLedgerId = ref("") //导入代币的ledger canister id
const tokenIndexId = ref("") //导入代币的index canister id
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

const ImportDIYToken = async () => {
  const isValid = formRef.value?.validate()
  if (!isValid) {
    return
  }
  if (tokenLedgerId.value === tokenIndexId.value) {
    showMessageError("ledger canister id is duplicated with index canister id.")
    return
  }

  // 查重，检查是否已存在具有相同ledger ID的代币
  const isDuplicate = addedTokenList.value.some(
    (token) => token.canisters.ledger === tokenLedgerId.value,
  )
  if (isDuplicate) {
    showMessageError("This token already exists")
    return
  }
  importLoading.value = true
  //再调用canister对应方法
  try {
    // 调用canister方法获取代币信息
    const newTokenInfo = await getDIYToken(
      tokenLedgerId.value,
      tokenIndexId.value,
    )
    showMessageSuccess("Tokens have been successfully imported")
    // 将新代币信息添加到导入列表
    addedTokenList.value.push(newTokenInfo)
    currentPage.value = "tokenListPage"
    //保存新的tokenList
    setTokenList(addedTokenList.value)
  } catch (error) {
    console.error("tokenLedgerId error:", error)
  } finally {
    importLoading.value = false
  }
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
