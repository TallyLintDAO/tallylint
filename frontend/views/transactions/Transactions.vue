<template>
  <div class="transactions-container">
    <div class="column">
      <div class="row items-center justify-between q-mb-sm">
        <div class="text-h4 row flex-y-center">
          Transactions
          <span class="transaction-number">
            {{ transactionAmount }}
          </span>
        </div>
        <div>
          <q-btn color="primary" @click="openDialog('add')"
            >Add Transaction</q-btn
          >
        </div>
      </div>
      <div class="header row q-mb-md justify-between">
        <div class="header-left row q-gutter-md">
          <q-select
            v-model="selectedWallet"
            @update:model-value="getSelectedWalletHistory(selectedWallet)"
            use-chips
            multiple
            option-label="name"
            option-value="address"
            :options="wallets"
            label="All Wallets"
          >
            <template v-slot:option="scope">
              <q-item v-bind="scope.itemProps">
                <q-item-section avatar>
                  <img
                    class="head-icon"
                    src="@/assets/dfinity.svg"
                    alt="NNS Icon"
                  />
                </q-item-section>
                <q-item-section>
                  <q-item-label>{{ scope.opt.name }}</q-item-label>
                  <q-item-label caption>Synced</q-item-label>
                </q-item-section>
              </q-item>
            </template>
          </q-select>
          <q-select
            use-chips
            multiple
            v-model="type"
            :options="typeOptions"
            label="Type"
          />
          <q-select
            use-chips
            multiple
            v-model="tag"
            :options="tagOptions"
            label="Tag"
          />
          <q-select
            multiple
            use-chips
            v-model="manual"
            :options="manualOptions"
            label="Manual"
          />
        </div>
        <div class="header-right row q-gutter-md flex-y-center">
          <el-date-picker
            v-model="date"
            type="daterange"
            range-separator="To"
            start-placeholder="Start date"
            end-placeholder="End date"
            :shortcuts="shortcuts"
            value-format="x"
          />
          <q-select
            rounded
            outlined
            v-model="sort"
            :options="sortOptions"
            label="Sort by"
          />
          <q-select
            rounded
            outlined
            v-model="pageSize"
            :options="pagesizeOptions"
            label="Per page"
          />
        </div>
      </div>
      <div v-if="showLoading">
        <q-spinner-cube size="xl" color="primary" />
      </div>
      <div v-else-if="transactionsList.length == 0">
        <span>No data available</span>
      </div>
      <div v-else>
        <q-list bordered separator>
          <template v-for="(transactions, date) in paginatedGroups" :key="date">
            <q-item>
              <q-item-section>
                <q-item-label class="text-h6" header>{{ date }}</q-item-label>
              </q-item-section>
            </q-item>
            <q-item
              v-for="transaction in transactions"
              :key="transaction.hash"
              clickable
              style="padding: 20px"
            >
              <!-- transaction log -->
              <div class="row items-center" style="width: 100%">
                <div class="col">
                  <q-icon
                    v-if="transaction.t_type === 'SEND'"
                    class="text-red-5"
                    size="md"
                    name="arrow_upward"
                  />
                  <q-icon
                    v-if="transaction.t_type === 'RECEIVE'"
                    class="text-green-6"
                    size="md"
                    name="arrow_downward"
                  />
                  {{ transaction.t_type }}
                  <br />
                  {{
                    new Date(transaction.timestamp).toLocaleTimeString(
                      "en-US",
                      { hour12: false },
                    )
                  }}
                </div>
                <div class="col">
                  <div class="flex-y-center q-gutter-sm">
                    <img
                      class="head-icon"
                      src="@/assets/dfinity.svg"
                      alt="NNS Icon"
                    />
                    <span>{{ transaction.walletName }}</span>
                  </div>
                  <span v-if="transaction.t_type === 'SEND'">-</span>
                  {{ transaction.details.currency.symbol }}
                  {{ transaction.details.amount }}
                  <br />
                  <span v-if="transaction.t_type === 'SEND'">
                    {{ "$" + transaction.details.cost + " cost basis" }}
                  </span>
                </div>
                <div class="col">
                  <q-icon size="md" name="arrow_forward" />
                </div>
                <div class="col">
                  {{ showUsername("", transaction.details.to || "") }}
                  <a
                    :href="
                      'https://dashboard.internetcomputer.org/transaction/' +
                      transaction.hash
                    "
                    target="_blank"
                  >
                    <q-icon name="open_in_new" />
                  </a>
                  <br />
                  <span>
                    {{ "≈ $" + transaction.details.value }}
                    <q-tooltip>
                      Market price by CoinGecko
                      <br />
                      ${{ transaction.details.price }} / ICP
                    </q-tooltip>
                  </span>
                  <b
                    v-if="transaction.t_type === 'SEND'"
                    :class="{
                      'text-green-6': transaction.details.profit > 0,
                      'text-red-5': transaction.details.profit < 0,
                    }"
                  >
                    {{ " · $ " + transaction.details.profit + " profit" }}
                  </b>
                </div>
                <div class="col">
                  <q-icon size="sm" name="more_vert"
                    ><q-menu>
                      <q-list style="min-width: 100px">
                        <q-item clickable v-close-popup="true">
                          <q-item-section
                            @click="openDialog('edit', transaction)"
                          >
                            Edit
                          </q-item-section>
                        </q-item>
                        <q-item clickable v-close-popup="true">
                          <q-item-section
                            @click="deleteTransaction(transaction.id)"
                          >
                            Delete
                          </q-item-section>
                        </q-item>
                      </q-list>
                    </q-menu></q-icon
                  >
                </div>
              </div>
            </q-item>
          </template>
        </q-list>
        <q-pagination
          v-model="currentPage"
          :max="maxPage"
          direction-links
          boundary-links
          class="justify-center"
        />
      </div>
    </div>
    <q-dialog v-model="dialogVisible">
      <q-card style="min-width: 450px">
        <q-card-section>
          <div class="text-h6">
            {{ isEdit ? "Edit Transaction" : "Add Transaction" }}
          </div>
        </q-card-section>
        <q-card-section class="q-pt-none">
          <q-form @submit="onSubmit" ref="form" class="q-gutter-md">
            <q-select
              filled
              v-model="transaction.t_type"
              :options="typeOptions"
              label="Type"
            />
            <q-card flat bordered class="my-card">
              <q-card-section>
                <div class="text-h6 row justify-between q-mb-md">
                  <div class="row items-center">
                    <q-icon class="text-red-5" size="md" name="arrow_upward" />
                    Send
                  </div>
                  <div class="text-grey text-caption row items-center">
                    Manual
                  </div>
                </div>
                <q-input
                  filled
                  label="From Address"
                  v-model="transaction.from"
                  class="q-mb-md"
                />
                <q-input
                  filled
                  label="Amount of tokens"
                  v-model="transaction.amount"
                  class="q-mb-md"
                >
                  <template v-slot:before>
                    <q-select
                      filled
                      map-options
                      v-model="transaction.currency"
                      :options="tokenList"
                      label="Token"
                      style="min-width: 100px"
                    >
                      <template v-slot:option="scope">
                        <q-item v-bind="scope.itemProps">
                          <q-item-section avatar>
                            <img
                              class="head-icon"
                              :src="scope.opt.icon"
                              alt="Icon"
                            />
                          </q-item-section>
                          <q-item-section>
                            <q-item-label class="text-subtitle1">
                              {{ scope.opt.label }}
                            </q-item-label>
                          </q-item-section>
                        </q-item>
                      </template>
                    </q-select>
                  </template>
                </q-input>
                <q-input
                  filled
                  label="Token Price"
                  v-model="transaction.price"
                />
              </q-card-section>

              <q-separator inset />

              <q-card-section>
                <div class="text-h6 row justify-between q-mb-md">
                  <div class="row items-center">
                    <q-icon
                      class="text-green-6"
                      size="md"
                      name="arrow_downward"
                    />
                    RECEIVE
                  </div>
                  <div class="text-grey text-caption row items-center">
                    Manual
                  </div>
                </div>
                <q-input
                  filled
                  label="To Target Address"
                  v-model="transaction.to"
                />
              </q-card-section>
            </q-card>
            <el-date-picker
              v-model="transaction.timestamp"
              type="datetime"
              placeholder="Transaction Datetime"
              value-format="x"
            />
            <q-input
              outlined
              label="Transaction Hash"
              v-model="transaction.from"
              class="q-mb-md"
            />
            <q-input
              outlined
              label="Description"
              v-model="transaction.commit"
              class="q-mb-md"
            />
            <div class="q-gutter-sm justify-end flex">
              <q-btn flat label="Cancel" v-close-popup="true" />
              <q-btn
                v-if="isEdit"
                :loading="loading"
                label="Update"
                type="submit"
                color="primary"
              />
              <q-btn
                v-else
                :loading="loading"
                label="Save"
                type="submit"
                color="primary"
              />
            </div>
          </q-form>
        </q-card-section>
      </q-card>
    </q-dialog>
  </div>
</template>

<script lang="ts" setup>
import { getAllTransactions } from "@/api/rosetta"
import { deleteSyncedTransactions, getUserAllWallets } from "@/api/user"
import type { InferredTransaction } from "@/types/sns"
import type { WalletTag } from "@/types/user"
import { showUsername } from "@/utils/avatars"
import { confirmDialog } from "@/utils/dialog"
import { showMessageSuccess } from "@/utils/message"
import { getAllSyncedTransactions } from "@/utils/syncedTransactions"
import type { QForm } from "quasar"
import { computed, onMounted, ref } from "vue"
import { useRoute } from "vue-router"

const route = useRoute()

const address = route.params.address
const transactionsList = ref<InferredTransaction[]>([])
const transaction = ref({
  hash: "",
  timestamp: 0,
  from: "",
  to: "",
  amount: 0,
  price: 0,
  currency: { decimals: 8, symbol: "ICP" },
  t_type: "",
  tag: "",
  manual: false,
  commit: "",
})
const type = ref([])
const typeOptions = ["SEND", "RECEIVE"]
const tag = ref([])
const tagOptions = ["Reward", "Mining", "Gift"]
const date = ref("") //采用这个方便判定为空
const manual = ref([])
const manualOptions = ["Manual"]
const sort = ref("Recent")
const sortOptions = ["Recent", "Oldest first", "Highest gains", "Lowest gains"]
const tokenList = [
  {
    decimals: 8,
    symbol: "ICP",
    label: "ICP",
    icon: "/frontend/assets/dfinity.svg",
    value: {
      decimals: 8,
      symbol: "ICP",
    },
  },
]
const selectedWallet = ref<WalletTag[]>([])
const wallets = ref<WalletTag[]>([])
const form = ref<QForm | null>(null)

const showLoading = ref(false)
const loading = ref(false)
const dialogVisible = ref(false)
const isEdit = ref(false)

const currentPage = ref(1)
const maxPage = ref(1)
const pageSize = ref(10)
const transactionAmount = ref(0)
const pagesizeOptions = ["5", "10", "25", "50"]
const shortcuts = [
  {
    text: "Last 12 months",
    value: () => {
      const end = new Date()
      const start = new Date()
      start.setMonth(end.getMonth() - 12)
      return [start, end]
    },
  },
  {
    text: new Date().getFullYear().toString(),
    value: () => {
      const end = new Date()
      const start = new Date(end.getFullYear(), 0, 1)
      return [start, end]
    },
  },
  {
    text: (new Date().getFullYear() - 1).toString(),
    value: () => {
      const end = new Date(new Date().getFullYear() - 1, 11, 31)
      const start = new Date(new Date().getFullYear() - 1, 0, 1)
      return [start, end]
    },
  },
  {
    text: (new Date().getFullYear() - 2).toString(),
    value: () => {
      const end = new Date(new Date().getFullYear() - 2, 11, 31)
      const start = new Date(new Date().getFullYear() - 2, 0, 1)
      return [start, end]
    },
  },
]

const groupedTransactions = (
  transactions: InferredTransaction[],
): {
  [date: string]: InferredTransaction[]
} => {
  const groups = {}
  transactions.forEach((transaction) => {
    const date = new Date(Number(transaction.timestamp)).toLocaleDateString()
    if (!groups[date]) {
      groups[date] = []
    }
    groups[date].push(transaction)
  })
  return groups
}
//先分页，再分组。
const paginatedGroups = computed(
  (): {
    [date: string]: InferredTransaction[]
  } => {
    const start = (currentPage.value - 1) * pageSize.value
    const end = start + pageSize.value
    let paginatedData = transactionsList.value
    // 分页之前先按选定的日期区间过滤记录
    if (date.value) {
      paginatedData = paginatedData.filter(
        (item) =>
          item.timestamp >= Number(date.value[0]) &&
          item.timestamp <= Number(date.value[1]),
      )
    }
    //在slice分页之前处理页码和数量
    maxPage.value = Math.ceil(paginatedData.length / pageSize.value)
    transactionAmount.value = paginatedData.length
    paginatedData = paginatedData.slice(start, end)
    return groupedTransactions(paginatedData)
  },
)

onMounted(() => {
  console.log("route", address)
  getWallets().then(() => {
    let walletsToQuery: WalletTag[]

    if (address) {
      // 如果 address 存在
      walletsToQuery = Array.isArray(address)
        ? // 如果 address 是数组，则直接使用
          address.map((addr) => ({ address: addr, name: "", from: "" }))
        : // 如果 address 是字符串，则构造包含单个地址的数组
          [{ address: address, name: "", from: "" }]
    } else {
      // 如果 address 不存在，则默认查询所有地址
      walletsToQuery = wallets.value
    }

    getSelectedWalletHistory(walletsToQuery)
  })
})

const getWallets = async () => {
  showLoading.value = true
  wallets.value = await getUserAllWallets()
}

const getSelectedWalletHistory = async (selectedWallets: WalletTag[]) => {
  showLoading.value = true
  currentPage.value = 1

  let targetWallets: WalletTag[]
  //如果没有选择任何钱包，则查询所有钱包
  selectedWallets.length !== 0
    ? (targetWallets = selectedWallets)
    : (targetWallets = wallets.value)
  getAllSyncedTransactions(0, 0, ["date-asc"], targetWallets)
  getAllTransactions(targetWallets)
    .then((res) => {
      console.log("getWalletHistory", res)
      if (res.total && res.total != 0) {
        transactionsList.value = res.transactions
        maxPage.value = Math.ceil(res.total / pageSize.value)
        transactionAmount.value = res.total
      }
    })
    .finally(() => {
      showLoading.value = false
    })
}

const openDialog = (action: string, itemInfo?: any) => {
  if (action === "edit" && itemInfo) {
    isEdit.value = true
    //理论上来说item里manual属性会覆盖transaction里的manual属性，所以这里不对manual做修改
  } else {
    //不为edit就是add
    isEdit.value = false
    transaction.value.manual = true
  }
  dialogVisible.value = true
}

const deleteTransaction = (transactionId: number) => {
  confirmDialog({
    title: "Delete Transaction",
    message:
      "Are you sure delete this transaction? Deleted transaction can't restore",
    okMethod: () => {
      deleteSyncedTransactions(transactionId).then((res) => {
        if (res.Ok) {
          showMessageSuccess("delete transaction success")
        }
      })
    },
  })
}
const onSubmit = () => {}
</script>

<style lang="scss">
.transactions-container {
  .transaction-number {
    font-size: 14px;
    line-height: 15px;
    color: #6c757d;
    border: 1px solid silver;
    border-radius: 10px;
    padding: 4px 8px;
    margin-left: 10px;
  }
  .header {
    .q-select {
      min-width: 150px;
    }
  }
}
</style>
@/utils/syncedTransactions
