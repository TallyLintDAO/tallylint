<template>
  <div class="transactions-container">
    <div class="column">
      <div class="row items-center justify-between">
        <div>Transaction {{ walletList.length }}</div>
        <div><q-btn color="primary">Add Transaction</q-btn></div>
      </div>
      <div class="header q-gutter-md row q-mb-md items-end">
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
          v-model="costMethod"
          :options="costMethodOptions"
          label="Cost Basis Method"
        />
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
        <q-select v-model="date" :options="typeOptions" label="Date" />
        <q-select
          multiple
          use-chips
          v-model="manual"
          :options="manualOptions"
          label="Manual"
        />
        <q-btn
          v-if="walletList.length > 0"
          flat
          color="primary"
          icon="file_download"
          label="Export CSV"
          @click="exportToCSV"
        />
      </div>
      <div v-if="showLoading">
        <q-spinner-cube size="xl" color="primary" />
      </div>
      <div v-else-if="wallets.length === 0">
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
              v-ripple="true"
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
                      This is the market price of the sent coin by CoinGecko
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
                  <q-icon size="sm" name="more_vert" />
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
  </div>
</template>

<script lang="ts" setup>
import { getAllTransactions } from "@/api/rosetta"
import { getUserNeuron, getUserWallet } from "@/api/user"
import type { InferredTransaction } from "@/types/sns"
import type { WalletTag } from "@/types/user"
import { showUsername } from "@/utils/avatars"
import { getNNS } from "@/utils/nns"
import { exportFile } from "quasar"
import { computed, onMounted, ref } from "vue"
import { useRoute } from "vue-router"

const route = useRoute()

const address = route.params.address
const walletList = ref<InferredTransaction[]>([])
const costMethodOptions = ["FIFO"]
const costMethod = ref("FIFO")
const type = ref([])
const typeOptions = ["SEND", "RECEIVE"]
const tag = ref([])
const tagOptions = ["Reward", "Mining", "Gift"]
const date = ref({ from: 0, to: 0 })
const manual = ref([])
const manualOptions = ["Manual"]
const selectedWallet = ref<WalletTag[]>([])
const wallets = ref<WalletTag[]>([])
const showLoading = ref(true)

const currentPage = ref(1)
const maxPage = ref(1)
const pageSize = ref(10)

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
    const paginatedData = walletList.value.slice(start, end)

    return groupedTransactions(paginatedData)
  },
)

onMounted(() => {
  console.log("route", address)
  getWallets().then(() => {
    if (address) {
      if (Array.isArray(address)) {
        // 如果route是数组，传递整个数组
        getSelectedWalletHistory(
          address.map((addr) => ({
            address: addr,
            name: "",
            from: "",
          })),
        )
      } else {
        // 如果route是字符串，构造包含单个地址的数组
        getSelectedWalletHistory([
          {
            address: address,
            name: "",
            from: "",
          },
        ])
      }
    } else {
      // 如果route不存在，默认查询所有地址
      getSelectedWalletHistory(wallets.value)
    }
  })
})

const getWallets = async () => {
  showLoading.value = true
  const [userWallets, neuronWallets, nnsWallets] = await Promise.all([
    getUserWallet(false),
    getUserNeuron(false),
    getNNS(),
  ])
  if (userWallets.Ok && neuronWallets.Ok) {
    const mapToWallet = (wallet: { name: any; address: any; from: any }) => ({
      name: wallet.name,
      address: wallet.address,
      from: wallet.from,
    })
    const userWalletList = userWallets.Ok.map(mapToWallet)
    const neuronWalletList = neuronWallets.Ok.map(mapToWallet)
    const nnsWalletList = nnsWallets.map((wallet, index) => ({
      name: "hotkey " + index + 1,
      address: wallet.address,
      from: "hotkey",
    }))

    wallets.value.push(...userWalletList, ...neuronWalletList, ...nnsWalletList)
  }
}

const getSelectedWalletHistory = async (selectedWallets: WalletTag[]) => {
  showLoading.value = true
  let targetWallets: WalletTag[]
  //如果没有选择任何钱包，则查询所有钱包
  selectedWallets.length !== 0
    ? (targetWallets = selectedWallets)
    : (targetWallets = wallets.value)
  getAllTransactions(targetWallets)
    .then((res) => {
      console.log("getWalletHistory", res)
      if (res.total && res.total != 0) {
        walletList.value = res.transactions
        maxPage.value = Math.ceil(res.total / pageSize.value)
      }
    })
    .finally(() => {
      showLoading.value = false
    })
}

const exportToCSV = async () => {
  const columnNames = [
    "Hash",
    "Type",
    "Status",
    "Timestamp",
    "From",
    "To",
    "Amount",
    "Fee",
    "Memo",
    "Price",
    "Cost",
    "Income",
    "Profit",
  ]

  // 生成包含列名和数据的数组
  const data = [
    columnNames,
    ...walletList.value.map((transaction) => [
      transaction.hash,
      transaction.t_type,
      transaction.details.status,
      //Time format fixed to Switzerland
      new Date(Number(transaction.timestamp)).toLocaleString("fr-CH"),
      transaction.details?.from,
      transaction.details?.to,
      transaction.details.amount,
      transaction.details.fee,
      "",
      transaction.details.price,
      transaction.details.cost,
      transaction.details.value,
      transaction.details.profit,
    ]),
  ]

  // 将数据转换为 CSV 格式的字符串
  const csvContent = data.map((row) => row.join(",")).join("\n")
  const todayDate = new Date().toLocaleDateString("fr-CH").replace(/\./g, "")
  const fileName = address ? "Tax_Data_" + address : "Tax_Data_All_Wallet"
  // 使用 exportFile 函数导出 CSV 文件
  exportFile(`${todayDate}_${fileName}.csv`, csvContent, "text/csv")
}
</script>

<style lang="scss">
.transactions-container {
  .header {
    .q-select {
      min-width: 150px;
    }
  }
}
</style>
