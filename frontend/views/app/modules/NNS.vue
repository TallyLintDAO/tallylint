<template>
  <div class="nns-container">
    <div class="q-pa-md">
      <q-table
        grid
        title="NNS"
        :rows="rows"
        :columns="columns"
        selection="multiple"
        :filter="filter"
        row-key="address"
      >
        <template v-slot:top>
          <div class="q-gutter-sm">
            <q-btn color="primary" @click="openDialog('add')"
              >Add NNS Neuron</q-btn
            >
            <el-tooltip effect="dark" placement="top-start">
              <template #content>
                Your Principal ID is {{ principal }}
                <q-icon
                  name="content_copy"
                  class="cursor-pointer"
                  @click="copyPid()"
                />
              </template>
              <q-btn
                flat
                color="primary"
                label="Help"
                icon="lightbulb_outline"
                @click="goHelp()"
              />
            </el-tooltip>
          </div>

          <q-space />
          <q-input
            borderless
            dense
            debounce="300"
            v-model="filter"
            placeholder="Search"
          >
            <template v-slot:append>
              <q-icon name="search" />
            </template>
          </q-input>
        </template>
        <template v-slot:item="props">
          <div
            class="q-pa-xs col-xs-12 col-sm-6 col-md-4 col-lg-3 grid-style-transition"
          >
            <q-card>
              <q-card-section>
                <q-card-section>
                  <div class="row justify-between items-center">
                    <div class="flex q-gutter-xs">
                      <img
                        class="head-icon"
                        src="@/assets/dfinity.svg"
                        alt="NNS Icon"
                      />
                      <div class="text-h6">{{ props.row.name }}</div>
                    </div>
                    <q-btn
                      v-if="props.row.from !== 'hotkey'"
                      flat
                      icon="more_vert"
                    >
                      <q-menu>
                        <q-list style="min-width: 100px">
                          <q-item clickable v-close-popup="true">
                            <q-item-section
                              @click="openDialog('edit', props.row)"
                            >
                              Edit
                            </q-item-section>
                          </q-item>
                          <q-item clickable v-close-popup="true">
                            <q-item-section @click="deleteItem(props.row.id)">
                              Delete
                            </q-item-section>
                          </q-item>
                        </q-list>
                      </q-menu>
                    </q-btn>
                  </div>
                </q-card-section>
                <q-list>
                  <q-item v-for="col in props.cols" :key="col.name">
                    <q-item-section>
                      <q-item-label>{{ col.label }}</q-item-label>
                      <q-item-label v-if="col.name === 'address'" caption>
                        <router-link :to="'/app/transactions/' + col.value">
                          {{ col.value }}
                        </router-link>
                      </q-item-label>
                      <q-item-label v-else caption>{{
                        col.value
                      }}</q-item-label>
                    </q-item-section>
                  </q-item>
                </q-list>
              </q-card-section>
            </q-card>
          </div>
        </template>
      </q-table>
    </div>
    <q-dialog v-model="dialogVisible">
      <q-card style="min-width: 350px">
        <q-card-section>
          <div class="text-h6">
            {{ isEdit ? "Edit Your Neuron" : "Your Neuron" }}
          </div>
        </q-card-section>
        <q-card-section class="q-pt-none">
          <q-form @submit="onSubmit" ref="form" class="q-gutter-md">
            <q-input
              filled
              :disable="isEdit"
              v-model="address"
              label="Neuron Account ID *"
              hint="Enter Neuron Account ID"
              lazy-rules
              :rules="[
                (val) =>
                  (val &&
                    val.length > 0 &&
                    (val.length === 63 || val.length === 64)) ||
                  'Please enter Principal ID or Account ID',
                (val) =>
                  (val && !rows.some((item) => item.address === val)) ||
                  isEdit ||
                  'Can not add this neuron, neuron account duplicated',
              ]"
            />
            <q-input
              filled
              v-model="neuron.name"
              label="Neuron Name *"
              hint="Nickname"
              lazy-rules
              :rules="[
                (val) => (val && val.length > 0) || 'Please type something',
              ]"
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
                label="Submit"
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
import { initAuth } from "@/api/auth"
import { DOCS_URL, NNS_HELP } from "@/api/constants/docs"
import { useUserStore } from "@/stores/user"
import type { WalletInfo } from "@/types/user"
import { confirmDialog } from "@/utils/dialog"
import {
  showMessageError,
  showMessageSuccess,
  showResultError,
} from "@/utils/message"
import { HttpAgent } from "@dfinity/agent"
import { GovernanceCanister } from "@dfinity/nns"
import { Principal } from "@dfinity/principal"
import { SnsWrapper, initSnsWrapper } from "@dfinity/sns"
import { QForm, copyToClipboard } from "quasar"
import { onMounted, ref } from "vue"

const userStore = useUserStore()

const dialogVisible = ref(false)
const filter = ref("") //搜索框
const principal = ref(useUserStore().principal) //搜索框
const columns = [
  {
    name: "address",
    required: true,
    label: "Neuron Account",
    field: (row) => row.address,
  },
  { name: "id", label: "Neuron Id", field: "id" },
  { name: "maturity", label: "Maturity", field: "maturity" },
  { name: "stakedMaturity", label: "StakedMaturity", field: "stakedMaturity" },
]
const form = ref<QForm | null>(null)

const rows = ref<any[]>([])
const loading = ref(false)
const isEdit = ref(false) // dialog是否是edit功能，否就是add功能
const address = ref("")
const neuron = ref({
  id: 0n,
  address: "",
  principal_id: [] as string[], // 无值就用[]，而不是[""]，不然opt类型会报错
  from: "NNS",
  name: "",
  transactions: 0,
  last_transaction_time: 0,
  last_sync_time: 0,
})

onMounted(() => {
  getNNS()
  // getSNS()
})

const resetNeuron = () => {
  //将值重置为初始值。
  neuron.value = {
    id: 0n,
    address: "",
    principal_id: [] as string[],
    from: "NNS",
    name: "",
    transactions: 0,
    last_transaction_time: 0,
    last_sync_time: 0,
  }
}

const openDialog = (action: string, itemInfo?: WalletInfo) => {
  if (action === "edit" && itemInfo) {
    isEdit.value = true
    address.value = itemInfo.address
    neuron.value = { ...itemInfo }
  } else {
    //不为edit就是add
    isEdit.value = false
    address.value = ""
    resetNeuron()
  }
  dialogVisible.value = true
}

const onSubmit = async () => {
  loading.value = true
  const validationSuccess = await form.value?.validate()
  if (validationSuccess) {
    if (isEdit.value) {
      await editItem()
    } else {
      await addItem()
    }
    dialogVisible.value = false
  } else {
    // 数据验证失败
    // 用户至少输入了一个无效值
  }
  loading.value = false
}

const addItem = async () => {
  const { address, name, from, principal_id } = neuron.value
  const res = await addUserWallet(
    address.trim(),
    name.trim(),
    from,
    principal_id,
  )
  if (res.Ok) {
    rows.value.push({ ...neuron.value })
    resetNeuron()
    dialogVisible.value = false
    // getWallets(true)
  } else {
    showResultError(res)
  }
}

const editItem = async () => {
  const { id, from, name } = neuron.value
  const res = await editUserWallet(id, from, name)
  if (res.Ok) {
    // getWallets(true)
  } else {
    showResultError(res)
  }
}

const deleteItem = (itemId: bigint) => {
  confirmDialog({
    title: "Delete Neuron",
    message:
      "Are you sure delete this Neuron? Delete Neuron will clear this Neuron history info",
    okMethod: (data) => {
      deleteUserWallet(itemId).then((res) => {
        if (res.Ok) {
          getWallets(true)
          showMessageSuccess("delete wallet success")
        }
      })
    },
  })
}

const getNNS = async () => {
  const ai = await initAuth()
  if (ai.info) {
    const identity = ai.info.identity
    const agent = new HttpAgent({ identity })
    console.log("agent", agent)
    const neuron = GovernanceCanister.create({
      agent: agent,
    })
    //获取授权当前pid的神经元列表
    neuron.listNeurons({ certified: false }).then((res) => {
      console.log("getListNeurons", res)
      if (res.length > 0) {
        for (const neuron of res) {
          if (neuron.fullNeuron) {
            const {
              id,
              accountIdentifier,
              maturityE8sEquivalent,
              stakedMaturityE8sEquivalent,
            } = neuron.fullNeuron
            const neuronData = {
              id: id,
              address: accountIdentifier,
              //1e8是10的八次方，除以1e8得到原数
              maturity: Number(maturityE8sEquivalent) / 1e8,
              stakedMaturity: Number(stakedMaturityE8sEquivalent) / 1e8,
              from: "hotkey",
            }
            console.log("neuronData", neuronData)
            rows.value.push(neuronData)
          }
        }
      }
    })
  }
}
const getSNS = async () => {
  const snsWrapper: SnsWrapper = await initSnsWrapper({
    rootOptions: {
      //根容器id在dashboard
      canisterId: Principal.fromText("3e3x2-xyaaa-aaaaq-aaalq-cai"),
    },
  })

  const { metadata: meta, swapState } = snsWrapper
  const [metadata, token] = await meta({})

  console.log("Summary data:", metadata, token)
}
const goHelp = () => {
  // To DOCS: NNS Help
  window.open(DOCS_URL + NNS_HELP)
}
const copyPid = () => {
  copyToClipboard(principal.value)
    .then(() => {
      showMessageSuccess(`copy ${principal.value} success`)
    })
    .catch(() => {
      showMessageError("copy failed")
    })
}
</script>

<style lang="scss">
.nns-container {
  .head-icon {
    width: 32px !important;
  }
  .q-table__top {
    padding: 0;
  }
}
</style>
